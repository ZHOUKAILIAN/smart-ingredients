//! Shared types and utilities for Smart Ingredients
//!
//! This crate contains common types used by both frontend and backend,
//! ensuring API contract consistency.

use serde::{Deserialize, Serialize};

mod analysis;
mod error;
mod ingredient;

pub use analysis::*;
pub use error::*;
pub use ingredient::*;

/// Analysis status tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AnalysisStatus {
    /// Analysis is queued
    Pending,
    /// OCR and LLM processing in progress
    Processing,
    /// Analysis completed successfully
    Completed,
    /// Analysis failed
    Failed,
}
