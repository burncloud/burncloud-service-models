//! # BurnCloud Service Models
//!
//! Service layer for BurnCloud model management system.
//! This layer provides business logic, validation, and data preprocessing
//! between the client and database layers.
//!
//! ## Modules
//! - `model`: Core model types and enums
//! - `validation`: Input validation logic
//! - `preprocessing`: Data preprocessing utilities
//! - `error`: Error types and handling
//! - `service`: Main service implementation

pub mod model;
pub mod validation;
pub mod preprocessing;
pub mod error;
pub mod service;

pub use model::*;
pub use validation::*;
pub use preprocessing::*;
pub use error::*;
pub use service::*;

// Type aliases for backward compatibility
pub type ModelSize = SizeCategory;
pub type RuntimeConfig = std::collections::HashMap<String, serde_json::Value>;

/// Available model with download information
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AvailableModel {
    /// Base model information
    #[serde(flatten)]
    pub model: Model,
    /// Whether the model is downloadable
    pub is_downloadable: bool,
    /// Estimated download time
    pub estimated_download_time: Option<std::time::Duration>,
}

impl AvailableModel {
    /// Create from a Model
    pub fn from_model(model: Model, _system_requirements: SystemRequirements) -> Self {
        Self {
            model,
            is_downloadable: true,
            estimated_download_time: None,
        }
    }
}

/// System requirements structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemRequirements {
    pub min_memory_gb: f32,
    pub recommended_memory_gb: f32,
    pub min_disk_space_gb: f32,
    pub requires_gpu: bool,
    pub supported_os: Vec<String>,
    pub supported_architectures: Vec<String>,
}