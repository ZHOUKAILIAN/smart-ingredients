//! Reusable components

mod confirm_modal;
mod error_display;
mod example_images;
mod export_preview_modal;
mod health_score_card;
mod icons;
mod image_preview;
mod ingredient_card;
mod ingredient_card_list;
mod ingredient_table;
mod loading_spinner;
mod preference_selector;
mod risk_badge;
mod share_button;
mod summary_card;
mod tab_icons;
mod toast;
mod usage_tips;

pub use confirm_modal::ConfirmModal;
pub use error_display::ErrorDisplay;
pub use example_images::ExampleImages;
pub use export_preview_modal::ExportPreviewModal;
pub use health_score_card::HealthScoreCard;
pub use icons::{IconArrowLeft, IconCamera, IconChart, IconCheckBadge};
pub use image_preview::ImagePreview;
pub use ingredient_card::IngredientCard;
pub use ingredient_card_list::IngredientCardList;
pub use ingredient_table::{IngredientRow, IngredientTable};
pub use loading_spinner::LoadingSpinner;
pub use preference_selector::{
    get_preference_description, get_preference_icon, get_preference_label, PreferenceCard,
    PreferenceSelector, PREFERENCE_OPTIONS,
};
pub use risk_badge::RiskBadge;
pub use share_button::{ShareButton, ShareExportProps};
pub use summary_card::SummaryCard;
pub use tab_icons::{IconHistory, IconHome, IconUser};
pub use toast::ToastHost;
pub use usage_tips::UsageTips;

mod bottom_nav;
mod main_layout;
pub use bottom_nav::BottomNav;
pub use main_layout::MainLayout;
