use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

/// Core model type representing an AI model in the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Model {
    /// Unique identifier for the model
    pub id: Uuid,
    /// Unique name for the model (used in URLs and references)
    pub name: String,
    /// Human-readable display name
    pub display_name: String,
    /// Optional description of the model
    pub description: Option<String>,
    /// Model version (semantic versioning)
    pub version: String,
    /// Type of model (Chat, Code, Text, etc.)
    pub model_type: ModelType,
    /// Size category of the model
    pub size_category: SizeCategory,
    /// File size in bytes
    pub file_size: u64,
    /// Provider/organization that created the model
    pub provider: String,
    /// License under which the model is distributed
    pub license: Option<String>,
    /// Tags associated with the model
    pub tags: Vec<String>,
    /// Supported languages
    pub languages: Vec<String>,
    /// Path to the model file (if downloaded)
    pub file_path: Option<String>,
    /// Checksum for file integrity verification
    pub checksum: Option<String>,
    /// URL for downloading the model
    pub download_url: Option<String>,
    /// Model configuration parameters
    pub config: HashMap<String, serde_json::Value>,
    /// User rating (0.0 to 5.0)
    pub rating: Option<f32>,
    /// Number of times downloaded
    pub download_count: u64,
    /// Whether this is an official model
    pub is_official: bool,
    /// When the model was created
    pub created_at: DateTime<Utc>,
    /// When the model was last updated
    pub updated_at: DateTime<Utc>,
}

/// Represents an installed model instance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InstalledModel {
    /// Unique identifier for the installation
    pub id: Uuid,
    /// Reference to the base model
    pub model: Model,
    /// Local installation path
    pub install_path: String,
    /// When the model was installed
    pub installed_at: DateTime<Utc>,
    /// Current runtime status
    pub status: ModelStatus,
    /// Port the model is running on (if running)
    pub port: Option<u16>,
    /// Process ID (if running)
    pub process_id: Option<u32>,
    /// Last time the model was used
    pub last_used: Option<DateTime<Utc>>,
    /// Number of times the model has been used
    pub usage_count: u64,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Model types supported by the system
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ModelType {
    /// Conversational AI models
    Chat,
    /// Code generation and completion models
    Code,
    /// Text generation models
    Text,
    /// Text embedding models
    Embedding,
    /// Image generation models
    Image,
    /// Image generation models (alias)
    ImageGeneration,
    /// Audio processing models
    Audio,
    /// Audio/Speech models (alias)
    Speech,
    /// Video processing models
    Video,
    /// Multimodal models
    Multimodal,
    /// Other specialized models
    Other,
}

/// Model size categories based on file size
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SizeCategory {
    /// Less than 3GB
    Small,
    /// 3GB to 8GB
    Medium,
    /// 8GB to 30GB
    Large,
    /// Greater than 30GB
    XLarge,
}

/// Runtime status of an installed model
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ModelStatus {
    /// Model is not running
    Stopped,
    /// Model is currently starting up
    Starting,
    /// Model is running and ready
    Running,
    /// Model is stopping
    Stopping,
    /// Model encountered an error
    Error,
}

/// Request payload for creating a new model
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateModelRequest {
    /// Unique name for the model
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    /// Human-readable display name
    #[validate(length(min = 1, max = 200))]
    pub display_name: String,
    /// Model version
    #[validate(length(min = 1, max = 50))]
    pub version: String,
    /// Type of model
    pub model_type: ModelType,
    /// Provider/organization
    #[validate(length(min = 1, max = 100))]
    pub provider: String,
    /// File size in bytes
    #[validate(range(min = 1))]
    pub file_size: u64,
    /// Optional description
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    /// Optional license
    #[validate(length(max = 200))]
    pub license: Option<String>,
    /// Tags (each tag 1-50 characters)
    #[validate(length(max = 20))]
    pub tags: Vec<String>,
    /// Supported languages
    #[validate(length(max = 10))]
    pub languages: Vec<String>,
    /// Optional file path
    #[validate(length(max = 500))]
    pub file_path: Option<String>,
    /// Optional download URL
    #[validate(url)]
    pub download_url: Option<String>,
    /// Configuration parameters
    pub config: HashMap<String, serde_json::Value>,
    /// Whether this is an official model
    pub is_official: bool,
}

/// Request payload for updating an existing model
#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
pub struct UpdateModelRequest {
    /// New display name
    #[validate(length(min = 1, max = 200))]
    pub display_name: Option<String>,
    /// New description
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    /// New version
    #[validate(length(min = 1, max = 50))]
    pub version: Option<String>,
    /// New license
    #[validate(length(max = 200))]
    pub license: Option<String>,
    /// New tags
    #[validate(length(max = 20))]
    pub tags: Option<Vec<String>>,
    /// New supported languages
    #[validate(length(max = 10))]
    pub languages: Option<Vec<String>>,
    /// New file path
    #[validate(length(max = 500))]
    pub file_path: Option<String>,
    /// New download URL
    #[validate(url)]
    pub download_url: Option<String>,
    /// New configuration
    pub config: Option<HashMap<String, serde_json::Value>>,
    /// New rating
    #[validate(range(min = 0.0, max = 5.0))]
    pub rating: Option<f32>,
}

/// Filter options for listing models
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelFilter {
    /// Filter by model type
    pub model_type: Option<ModelType>,
    /// Filter by provider
    pub provider: Option<String>,
    /// Filter by official status
    pub is_official: Option<bool>,
    /// Search query for name/description
    pub search: Option<String>,
    /// Maximum number of results
    pub limit: Option<u32>,
    /// Number of results to skip
    pub offset: Option<u32>,
}

impl std::fmt::Display for ModelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelType::Chat => write!(f, "Chat"),
            ModelType::Code => write!(f, "Code"),
            ModelType::Text => write!(f, "Text"),
            ModelType::Embedding => write!(f, "Embedding"),
            ModelType::Image => write!(f, "Image"),
            ModelType::ImageGeneration => write!(f, "Image"),
            ModelType::Audio => write!(f, "Audio"),
            ModelType::Speech => write!(f, "Audio"),
            ModelType::Video => write!(f, "Video"),
            ModelType::Multimodal => write!(f, "Multimodal"),
            ModelType::Other => write!(f, "Other"),
        }
    }
}

impl std::str::FromStr for ModelType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "chat" => Ok(ModelType::Chat),
            "code" => Ok(ModelType::Code),
            "text" => Ok(ModelType::Text),
            "embedding" => Ok(ModelType::Embedding),
            "image" | "imagegeneration" => Ok(ModelType::Image),
            "audio" | "speech" => Ok(ModelType::Audio),
            "video" => Ok(ModelType::Video),
            "multimodal" => Ok(ModelType::Multimodal),
            "other" => Ok(ModelType::Other),
            _ => Err(format!("Invalid model type: {}", s)),
        }
    }
}

impl std::fmt::Display for SizeCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SizeCategory::Small => write!(f, "Small"),
            SizeCategory::Medium => write!(f, "Medium"),
            SizeCategory::Large => write!(f, "Large"),
            SizeCategory::XLarge => write!(f, "XLarge"),
        }
    }
}

impl From<u64> for SizeCategory {
    fn from(size_bytes: u64) -> Self {
        let size_gb = size_bytes as f64 / 1_073_741_824.0; // 1024^3
        match size_gb {
            s if s < 3.0 => SizeCategory::Small,
            s if s < 8.0 => SizeCategory::Medium,
            s if s < 30.0 => SizeCategory::Large,
            _ => SizeCategory::XLarge,
        }
    }
}

impl std::fmt::Display for ModelStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelStatus::Stopped => write!(f, "Stopped"),
            ModelStatus::Starting => write!(f, "Starting"),
            ModelStatus::Running => write!(f, "Running"),
            ModelStatus::Stopping => write!(f, "Stopping"),
            ModelStatus::Error => write!(f, "Error"),
        }
    }
}

impl std::str::FromStr for ModelStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "stopped" => Ok(ModelStatus::Stopped),
            "starting" => Ok(ModelStatus::Starting),
            "running" => Ok(ModelStatus::Running),
            "stopping" => Ok(ModelStatus::Stopping),
            "error" => Ok(ModelStatus::Error),
            _ => Err(format!("Invalid model status: {}", s)),
        }
    }
}

impl Model {
    /// Format file size in human-readable format
    pub fn formatted_size(&self) -> String {
        let size_gb = self.file_size as f64 / 1_073_741_824.0; // 1024^3
        if size_gb < 1.0 {
            let size_mb = self.file_size as f64 / 1_048_576.0; // 1024^2
            format!("{:.1}MB", size_mb)
        } else {
            format!("{:.1}GB", size_gb)
        }
    }
}

impl InstalledModel {
    /// Create an InstalledModel from a Model
    pub fn from_model(model: Model, install_path: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            model,
            install_path,
            installed_at: now,
            status: ModelStatus::Stopped,
            port: None,
            process_id: None,
            last_used: None,
            usage_count: 0,
            created_at: now,
            updated_at: now,
        }
    }

    /// Mark the model as used
    pub fn mark_used(&mut self) {
        self.last_used = Some(chrono::Utc::now());
        self.usage_count += 1;
        self.updated_at = chrono::Utc::now();
    }

    /// Check if the model is currently running
    pub fn is_running(&self) -> bool {
        matches!(self.status, ModelStatus::Running)
    }
}