//! Rule-based ingredient analysis

use serde::Deserialize;
use sqlx::PgPool;
use std::collections::{HashMap, HashSet};

use crate::services::llm::PreferenceType;

#[derive(Debug, Clone, Deserialize)]
pub struct RuleItem {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub category: String,
    pub risk_level: String,
    #[serde(default)]
    pub groups: Vec<String>,
    pub description: String,
    #[serde(default)]
    pub evidence: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct RuleRow {
    id: String,
    name: String,
    aliases: Vec<String>,
    category: String,
    risk_level: String,
    groups: Vec<String>,
    description: String,
    evidence: Option<String>,
    source: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RuleEvaluation {
    pub hits: Vec<shared::RuleHit>,
    pub confidence: shared::ConfidenceInfo,
}

#[derive(Clone)]
pub struct RuleEngine {
    items: Vec<RuleItem>,
    lookup: HashMap<String, RuleItem>,
    load_error: Option<String>,
}

impl RuleEngine {
    pub fn load_from_path(path: &str) -> Self {
        match load_items_from_path(path) {
            Ok(items) => Self::build(items),
            Err(err) => Self {
                items: Vec::new(),
                lookup: HashMap::new(),
                load_error: Some(format!("rules load failed: {}", err)),
            },
        }
    }

    pub async fn try_load_from_db(pool: &PgPool) -> anyhow::Result<Self> {
        let rows = sqlx::query_as::<_, RuleRow>(
            "SELECT id, name, aliases, category, risk_level, groups, description, evidence, source \
             FROM rules WHERE enabled = true",
        )
        .fetch_all(pool)
        .await?;

        let items = rows
            .into_iter()
            .map(|row| RuleItem {
                id: row.id,
                name: row.name,
                aliases: row.aliases,
                category: row.category,
                risk_level: row.risk_level,
                groups: row.groups,
                description: row.description,
                evidence: row.evidence,
                source: row.source,
            })
            .collect();

        Ok(Self::build(items))
    }

    fn build(items: Vec<RuleItem>) -> Self {
        let mut lookup = HashMap::new();
        for item in &items {
            let normalized = normalize_token(&item.name);
            lookup.insert(normalized, item.clone());
            for alias in &item.aliases {
                lookup.insert(normalize_token(alias), item.clone());
            }
        }
        Self {
            items,
            lookup,
            load_error: None,
        }
    }

    pub fn evaluate(&self, text: &str, preference: PreferenceType) -> RuleEvaluation {
        if let Some(error) = &self.load_error {
            return RuleEvaluation {
                hits: Vec::new(),
                confidence: shared::ConfidenceInfo {
                    level: "low".to_string(),
                    reasons: vec![format!("规则库不可用：{}", error)],
                    factors: vec![shared::ConfidenceFactor {
                        key: "rule_engine".to_string(),
                        label: "规则引擎".to_string(),
                        score: -30,
                        detail: Some("规则库加载失败".to_string()),
                    }],
                },
            };
        }

        let mut hits = Vec::new();
        let mut seen = HashSet::new();
        for token in split_ingredients(text) {
            if token.is_empty() {
                continue;
            }
            if let Some(item) = self.lookup.get(&token) {
                if !seen.insert(item.id.clone()) {
                    continue;
                }
                let mut risk_level = item.risk_level.clone();
                if should_raise_risk(preference, &item.groups) {
                    risk_level = bump_risk(&risk_level);
                }
                hits.push(shared::RuleHit {
                    name: item.name.clone(),
                    category: item.category.clone(),
                    risk_level,
                    description: item.description.clone(),
                    group_tags: item.groups.clone(),
                    evidence: item.evidence.clone(),
                    source: item.source.clone(),
                });
            }
        }

        let confidence = build_confidence(&hits, text);

        RuleEvaluation { hits, confidence }
    }
}

pub fn load_items_from_path(path: &str) -> anyhow::Result<Vec<RuleItem>> {
    let content = std::fs::read_to_string(path)?;
    let items = serde_json::from_str::<Vec<RuleItem>>(&content)?;
    Ok(items)
}

fn build_confidence(hits: &[shared::RuleHit], text: &str) -> shared::ConfidenceInfo {
    let text_len = text.trim().chars().count();
    let hit_count = hits.len();
    let mut score = 50;

    let mut factors = Vec::new();
    if hit_count > 0 {
        score += 30;
        factors.push(shared::ConfidenceFactor {
            key: "rule_hits".to_string(),
            label: "规则命中".to_string(),
            score: 30,
            detail: Some(format!("命中 {} 条规则", hit_count)),
        });
    } else {
        score -= 10;
        factors.push(shared::ConfidenceFactor {
            key: "rule_hits".to_string(),
            label: "规则命中".to_string(),
            score: -10,
            detail: Some("未命中规则".to_string()),
        });
    }

    if text_len < 6 {
        score -= 20;
        factors.push(shared::ConfidenceFactor {
            key: "text_length".to_string(),
            label: "文本长度".to_string(),
            score: -20,
            detail: Some("文本过短".to_string()),
        });
    } else if text_len < 20 {
        score -= 10;
        factors.push(shared::ConfidenceFactor {
            key: "text_length".to_string(),
            label: "文本长度".to_string(),
            score: -10,
            detail: Some("文本偏短".to_string()),
        });
    } else {
        factors.push(shared::ConfidenceFactor {
            key: "text_length".to_string(),
            label: "文本长度".to_string(),
            score: 10,
            detail: Some("文本长度充足".to_string()),
        });
        score += 10;
    }

    let level = if score >= 75 {
        "high"
    } else if score >= 50 {
        "medium"
    } else {
        "low"
    };

    let mut reasons = Vec::new();
    if hit_count > 0 {
        reasons.push("命中规则库成分".to_string());
    } else {
        reasons.push("未命中规则，基于模型解释".to_string());
    }
    if text_len < 6 {
        reasons.push("OCR 文本过短，可信度降低".to_string());
    }

    shared::ConfidenceInfo {
        level: level.to_string(),
        reasons,
        factors,
    }
}

fn split_ingredients(text: &str) -> Vec<String> {
    let cleaned = text
        .replace("配料表", "")
        .replace("配料", "")
        .replace('：', ":");
    cleaned
        .split(|c| matches!(c, ',' | '，' | '、' | ';' | '；' | '\n' | '/' | '|'))
        .map(|item| normalize_token(item))
        .filter(|item| !item.is_empty())
        .collect()
}

fn normalize_token(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .replace([' ', '\t'], "")
        .replace('（', "(")
        .replace('）', ")")
}

fn should_raise_risk(preference: PreferenceType, groups: &[String]) -> bool {
    let tag = match preference {
        PreferenceType::Allergy => "allergy",
        PreferenceType::Kids => "kids",
        PreferenceType::WeightLoss => "weight_loss",
        PreferenceType::Health => "health",
        PreferenceType::Fitness => "fitness",
        PreferenceType::None => return false,
    };
    groups.iter().any(|group| group == tag)
}

fn bump_risk(level: &str) -> String {
    match level.trim().to_lowercase().as_str() {
        "low" => "medium",
        "medium" => "high",
        "high" => "high",
        _ => "medium",
    }
    .to_string()
}
