//! Shared types and utilities for Smart Ingredients
//!
//! This crate contains common types used by both frontend and backend,
//! ensuring API contract consistency.

use serde::{Deserialize, Serialize};

mod analysis;
mod auth;
mod error;
mod ingredient;
mod user;

pub use analysis::*;
pub use auth::*;
pub use error::*;
pub use ingredient::*;
pub use user::*;

/// OCR status tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OcrStatus {
    /// OCR is queued
    Pending,
    /// OCR is processing
    Processing,
    /// OCR completed
    Completed,
    /// OCR failed
    Failed,
}

/// LLM status tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LlmStatus {
    /// LLM is queued
    Pending,
    /// LLM is processing
    Processing,
    /// LLM completed
    Completed,
    /// LLM failed
    Failed,
}

/// Analysis status tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalysisStatus {
    /// OCR is queued
    OcrPending,
    /// OCR is processing
    OcrProcessing,
    /// OCR completed, waiting for user confirmation
    OcrCompleted,
    /// OCR failed
    OcrFailed,
    /// LLM is queued
    LlmPending,
    /// LLM is processing
    LlmProcessing,
    /// Analysis completed successfully
    Completed,
    /// Analysis failed
    Failed,
}
