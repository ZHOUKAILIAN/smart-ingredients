//! State management stores

use leptos::prelude::*;
use shared::{AnalysisResponse, UserProfile};
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

#[derive(Clone, Debug, PartialEq)]
pub enum AnalysisSource {
    NewAnalysis,
    History,
}

/// Error information
#[derive(Clone, Debug)]
pub struct ErrorInfo {
    pub title: String,
    pub message: String,
    pub recoverable: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ToastLevel {
    Error,
    Warning,
    Success,
    Info,
}

#[derive(Clone, Debug)]
pub struct ToastMessage {
    pub id: u64,
    pub level: ToastLevel,
    pub title: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabRoute {
    Home,
    History,
    Community,
    Profile,
}

impl TabRoute {
    pub fn path(&self) -> &'static str {
        match self {
            TabRoute::Home => "/",
            TabRoute::History => "/history",
            TabRoute::Community => "/community",
            TabRoute::Profile => "/profile",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            TabRoute::Home => "首页",
            TabRoute::History => "历史",
            TabRoute::Community => "社区",
            TabRoute::Profile => "我的",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            TabRoute::Home => "🏠",
            TabRoute::History => "📋",
            TabRoute::Community => "🧩",
            TabRoute::Profile => "👤",
        }
    }
}

#[derive(Clone, Copy)]
pub struct AppState {
    // Existing fields
    pub analysis_id: RwSignal<Option<Uuid>>,
    pub analysis_result: RwSignal<Option<AnalysisResponse>>,
    pub analysis_preference: RwSignal<Option<String>>,
    pub error_message: RwSignal<Option<String>>,
    pub ocr_text: RwSignal<Option<String>>,
    pub confirmed_text: RwSignal<Option<String>>,

    // New fields for interaction optimization
    pub loading_state: RwSignal<LoadingState>,
    pub result_page_state: RwSignal<ResultPageState>,
    pub error: RwSignal<Option<ErrorInfo>>,
    pub selected_image_path: RwSignal<Option<String>>,
    pub toasts: RwSignal<Vec<ToastMessage>>,
    pub auth_user: RwSignal<Option<UserProfile>>,
    pub auth_loading: RwSignal<bool>,
    pub analysis_source: RwSignal<AnalysisSource>,

    // New fields for navigation
    pub current_tab: RwSignal<TabRoute>,
    pub last_home_path: RwSignal<String>,
    pub last_history_path: RwSignal<String>,
    pub last_community_path: RwSignal<String>,
    pub last_profile_path: RwSignal<String>,

    /// When `true`, the capture page opens directly in scan mode
    /// (bypassing the home hero view). Set by onboarding, result, or history pages.
    pub open_in_scan_mode: RwSignal<bool>,
}
