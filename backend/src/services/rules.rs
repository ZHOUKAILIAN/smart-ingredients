//! Rule-based ingredient analysis

use serde::Deserialize;
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
        let content = std::fs::read_to_string(path);
        match content {
            Ok(raw) => match serde_json::from_str::<Vec<RuleItem>>(&raw) {
                Ok(items) => Self::build(items),
                Err(err) => Self {
                    items: Vec::new(),
                    lookup: HashMap::new(),
                    load_error: Some(format!("rules parse failed: {}", err)),
                },
            },
            Err(err) => Self {
                items: Vec::new(),
                lookup: HashMap::new(),
                load_error: Some(format!("rules load failed: {}", err)),
            },
        }
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
                });
            }
        }

        let confidence = if hits.is_empty() {
            shared::ConfidenceInfo {
                level: "medium".to_string(),
                reasons: vec!["未命中规则，基于模型解释".to_string()],
            }
        } else {
            shared::ConfidenceInfo {
                level: "high".to_string(),
                reasons: vec!["命中规则库成分".to_string()],
            }
        };

        RuleEvaluation { hits, confidence }
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
