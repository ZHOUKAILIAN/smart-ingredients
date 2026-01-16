//! Ingredient-related types

use serde::{Deserialize, Serialize};

/// Ingredient category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IngredientCategory {
    /// Preservatives and additives
    Additive,
    /// Common allergens
    Allergen,
    /// Nutritional components
    Nutrition,
    /// Flavorings and seasonings
    Flavoring,
    /// Colorings
    Coloring,
    /// Other ingredients
    Other,
}

/// Health risk level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthRisk {
    /// Generally safe
    Low,
    /// Moderate concern
    Medium,
    /// High concern, avoid if possible
    High,
}

/// Ingredient database record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    /// Database ID
    pub id: i32,
    /// Ingredient name
    pub name: String,
    /// Category
    pub category: IngredientCategory,
    /// Health risk level
    pub health_risk: HealthRisk,
    /// Description
    pub description: Option<String>,
}
