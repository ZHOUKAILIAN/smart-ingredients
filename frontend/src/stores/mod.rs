//! State management stores

use leptos::prelude::*;
use shared::AnalysisResponse;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub analysis_id: RwSignal<Option<Uuid>>,
    pub analysis_result: RwSignal<Option<AnalysisResponse>>,
    pub error_message: RwSignal<Option<String>>,
}
