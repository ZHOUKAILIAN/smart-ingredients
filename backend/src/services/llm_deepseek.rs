//! DeepSeek LLM provider implementation

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

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

        let response: DeepSeekResponse = response.error_for_status()?.json().await?;
        let content = response
            .choices
            .get(0)
            .ok_or_else(|| anyhow::anyhow!("DeepSeek response missing choices"))?
            .message
            .content
            .clone();

        let result: shared::AnalysisResult = serde_json::from_str(&content)?;
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
3. 对高风险配料给出明确警告
4. recommendation 提供实用的食用建议"#,
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
