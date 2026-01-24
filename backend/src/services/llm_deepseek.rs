//! DeepSeek LLM provider implementation

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::config::LlmConfig;
use crate::services::llm::LlmProviderClient;

#[derive(Clone)]
pub struct DeepSeekClient {
    http: reqwest::Client,
    config: LlmConfig,
}

impl DeepSeekClient {
    pub fn new(config: &LlmConfig, http: reqwest::Client) -> Self {
        Self {
            http,
            config: config.clone(),
        }
    }
}

#[async_trait]
impl LlmProviderClient for DeepSeekClient {
    async fn analyze_ingredients(&self, text: &str) -> anyhow::Result<shared::AnalysisResult> {
        let prompt = build_analysis_prompt(text);
        let request = DeepSeekRequest {
            model: self.config.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
            temperature: 0.3,
        };

        let response = self
            .http
            .post(&self.config.api_url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&request)
            .timeout(self.config.timeout)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            return Err(anyhow::anyhow!(
                "DeepSeek API error: status={}, body={}",
                status,
                truncate_for_log(&body, 2000)
            ));
        }

        let response: DeepSeekResponse = serde_json::from_str(&body).map_err(|err| {
            anyhow::anyhow!(
                "DeepSeek API response parse error: {}; body={}",
                err,
                truncate_for_log(&body, 2000)
            )
        })?;
        let content = response
            .choices
            .first()
            .ok_or_else(|| anyhow::anyhow!("DeepSeek response missing choices"))?
            .message
            .content
            .clone();

        let result = parse_analysis_result(&content).map_err(|err| {
            warn!(
                "DeepSeek content parse failed: {}; content={}",
                err,
                truncate_for_log(&content, 2000)
            );
            err
        })?;
        Ok(result)
    }
}

fn build_analysis_prompt(text: &str) -> String {
    format!(
        r#"你是一个专业的食品配料分析专家。请分析以下配料表，并返回 JSON 格式的健康评估。

配料表：
{}

请严格按照以下 JSON 格式返回：
{{
  "health_score": <0-100 的整数>,
  "summary": "<1-3 句概括配料表特点>",
  "table": [
    {{
      "name": "<配料名称>",
      "category": "<additive|allergen|nutrition|other>",
      "function": "<主要作用或用途>",
      "risk_level": "<low|medium|high|unknown>",
      "note": "<补充说明，可为空>"
    }}
  ],
  "ingredients": [
    {{
      "name": "<配料名称>",
      "category": "<additive|allergen|nutrition>",
      "risk_level": "<low|medium|high>",
      "description": "<简短说明>"
    }}
  ],
  "warnings": [
    {{
      "warning_type": "<警告类型>",
      "ingredients": ["<配料1>", "<配料2>"],
      "message": "<警告信息>"
    }}
  ],
  "recommendation": "<总体建议>"
}}

要求：
1. health_score 基于配料的整体健康程度评分
2. 识别所有添加剂、过敏原和关键营养成分
3. summary 简要概括配料表特点
4. table 与配料顺序保持一致，同名可去重并保留风险更高项
5. 对高风险配料给出明确警告
6. recommendation 提供实用的食用建议"#,
        text
    )
}

#[derive(Debug, Serialize)]
struct DeepSeekRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct DeepSeekResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

fn parse_analysis_result(content: &str) -> anyhow::Result<shared::AnalysisResult> {
    let trimmed = content.trim();
    if let Ok(result) = serde_json::from_str::<shared::AnalysisResult>(trimmed) {
        return Ok(result);
    }

    let without_fence = strip_code_fence(trimmed);
    if let Ok(result) = serde_json::from_str::<shared::AnalysisResult>(&without_fence) {
        return Ok(result);
    }

    let extracted = extract_json_block(&without_fence).ok_or_else(|| {
        anyhow::anyhow!("unable to extract JSON object from DeepSeek content")
    })?;
    serde_json::from_str::<shared::AnalysisResult>(extracted).map_err(Into::into)
}

fn strip_code_fence(content: &str) -> String {
    let trimmed = content.trim();
    if !trimmed.starts_with("```") {
        return trimmed.to_string();
    }

    let mut lines = trimmed.lines();
    let _ = lines.next();
    let mut remainder = lines.collect::<Vec<_>>().join("\n");
    if remainder.ends_with("```") {
        remainder.truncate(remainder.len() - 3);
    }
    remainder.trim().to_string()
}

fn extract_json_block(content: &str) -> Option<&str> {
    let start = content.find('{')?;
    let end = content.rfind('}')?;
    if end <= start {
        return None;
    }
    content.get(start..=end)
}

fn truncate_for_log(value: &str, max: usize) -> String {
    if value.len() <= max {
        value.to_string()
    } else {
        format!("{}...<truncated>", &value[..max])
    }
}
