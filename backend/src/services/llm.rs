//! LLM service for ingredient analysis

use anyhow::Result;

/// Analyze ingredients and return health assessment
///
/// # Arguments
///
/// * `text` - Extracted text from OCR
///
/// # Returns
///
/// Analysis result with health score and recommendations
pub async fn analyze_ingredients(_text: &str) -> Result<shared::AnalysisResult> {
    // TODO: Integrate LLM service
    // Options:
    // 1. DeepSeek API
    // 2. 智谱 AI API
    Ok(shared::AnalysisResult {
        health_score: 75,
        ingredients: vec![],
        warnings: vec![],
        recommendation: "Placeholder recommendation".to_string(),
    })
}
