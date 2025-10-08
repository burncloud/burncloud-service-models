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