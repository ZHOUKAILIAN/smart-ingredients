//! State management stores

use leptos::prelude::*;
use shared::AnalysisResponse;
use uuid::Uuid;

/// Loading state for async operations
#[derive(Clone, Debug, PartialEq)]
pub enum LoadingState {
    Idle,
    OcrProcessing,
    LlmAnalyzing,
}

/// Page state for result pages
#[derive(Clone, Debug, PartialEq)]
pub enum ResultPageState {
    Summary,
    Detail,
}

/// Error information
#[derive(Clone, Debug)]
pub struct ErrorInfo {
    pub title: String,
    pub message: String,
    pub recoverable: bool,
}

#[derive(Clone)]
pub struct AppState {
    // Existing fields
    pub analysis_id: RwSignal<Option<Uuid>>,
    pub analysis_result: RwSignal<Option<AnalysisResponse>>,
    pub error_message: RwSignal<Option<String>>,
    pub ocr_text: RwSignal<Option<String>>,
    pub confirmed_text: RwSignal<Option<String>>,

    // New fields for interaction optimization
    pub loading_state: RwSignal<LoadingState>,
    pub result_page_state: RwSignal<ResultPageState>,
    pub error: RwSignal<Option<ErrorInfo>>,
    pub selected_image_path: RwSignal<Option<String>>,
}
