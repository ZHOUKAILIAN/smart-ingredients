//! DeepSeek LLM provider implementation

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::config::LlmConfig;
use crate::services::llm::{LlmProviderClient, PreferenceType};

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
    async fn analyze_ingredients(
        &self,
        text: &str,
        preference: PreferenceType,
    ) -> anyhow::Result<shared::AnalysisResult> {
        let prompt = build_analysis_prompt(text, preference);
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

fn build_analysis_prompt(text: &str, preference: PreferenceType) -> String {
    let preference_instruction = build_preference_instruction(preference);
    format!(
        r#"你是一个专业的食品配料分析专家。请分析以下配料表，并返回 JSON 格式的健康评估。

配料表：
{}

分析偏好：{}
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
  "overall_assessment": "<总体评价>",
  "recommendation": "<摄入建议或频次建议>",
  "focus_summary": "<偏好相关总结，可为空>",
  "focus_ingredients": ["<偏好相关成分1>", "<偏好相关成分2>"],
  "score_breakdown": [
    {{
      "dimension": "<additives_processing|sugar_fat|nutrition_value|sensitive|formula_complexity>",
      "score": <0-100 的整数>,
      "reason": "<简短理由>"
    }}
  ]
}}

要求：
1. health_score 基于配料的整体健康程度评分
2. 识别所有添加剂、过敏原和关键营养成分
3. summary 简要概括配料表特点
4. table 与配料顺序保持一致，同名可去重并保留风险更高项
5. 对高风险配料给出明确警告
6. overall_assessment 简要给出总体评价或结论
7. recommendation 必须紧扣分析偏好，给出摄入建议/频次建议；控制在 20-30 字左右
8. score_breakdown 的 dimension 必须使用指定枚举值
9. focus_summary 与 focus_ingredients 根据偏好给出重点信息"#,
        text,
        preference.as_key(),
        preference_instruction
    )
}

fn build_preference_instruction(preference: PreferenceType) -> &'static str {
    match preference {
        PreferenceType::WeightLoss => {
            "请重点关注热量、糖分、脂肪与反式脂肪酸，提示高糖高脂风险；recommendation 给出控糖控脂/热量管理的摄入建议。"
        }
        PreferenceType::Health => {
            "请重点关注添加剂、防腐剂、色素、香精与加工程度；recommendation 强调减少添加剂摄入与选择更天然配方的摄入建议。"
        }
        PreferenceType::Fitness => {
            "请重点关注蛋白质含量与质量、碳水类型、优质脂肪；recommendation 围绕蛋白摄入与碳水质量给出摄入建议。"
        }
        PreferenceType::Allergy => {
            "请重点识别常见过敏原并提高其风险提示；recommendation 给出明确的过敏规避建议。"
        }
        PreferenceType::Kids => {
            "请重点关注色素、香精、防腐剂、糖分与咖啡因等不适合儿童成分；recommendation 强调儿童适宜性与替代选择。"
        }
        PreferenceType::None => "按通用健康标准分析即可；recommendation 提供通用的摄入建议。",
    }
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

    let extracted = extract_json_block(&without_fence)
        .ok_or_else(|| anyhow::anyhow!("unable to extract JSON object from DeepSeek content"))?;
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
