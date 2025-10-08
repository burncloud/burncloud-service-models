use crate::{
    Model, InstalledModel, CreateModelRequest, UpdateModelRequest, ModelFilter,
    ModelType, ModelStatus, SizeCategory, ServiceError
};
use burncloud_database_models::{ModelsService as DatabaseModelsService, BasicModel, BasicInstalledModel, BasicModelType, BasicSizeCategory, BasicModelStatus};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;

/// High-level service for managing models with business logic
///
/// This service sits between the client layer and database layer,
/// providing validation, preprocessing, and business rule enforcement.
pub struct ModelsService {
    database_service: Arc<DatabaseModelsService>,
}

impl ModelsService {
    /// Create a new ModelsService instance
    pub async fn new(database: Arc<burncloud_database::Database>) -> Result<Self, ServiceError> {
        let database_service = Arc::new(DatabaseModelsService::new(database).await?);
        Ok(Self { database_service })
    }

    /// Create a new model with validation and preprocessing
    pub async fn create_model(&self, request: CreateModelRequest) -> Result<Model, ServiceError> {
        // Validate the request
        crate::validation::validate_create_model(&request)?;

        // Preprocess data
        let processed_model = crate::preprocessing::preprocess_create_model(request)?;

        // Convert to basic model for database layer
        let basic_model = service_model_to_basic_update(&processed_model)?;

        // Create in database
        let created_basic = self.database_service.repository().create_model(&basic_model.try_into()?).await?;

        // Convert back to service model
        let created_basic_model: BasicModel = created_basic.try_into()?;
        basic_model_to_service(&created_basic_model)
    }

    /// Get a model by ID
    pub async fn get_model(&self, id: Uuid) -> Result<Option<Model>, ServiceError> {
        let basic_result = self.database_service.repository().get_model_by_id(id).await?;

        match basic_result {
            Some(basic_model) => {
                let basic_model: BasicModel = basic_model.try_into()?;
                Ok(Some(basic_model_to_service(&basic_model)?))
            }
            None => Ok(None),
        }
    }

    /// List models with filtering and pagination
    pub async fn list_models(&self, filter: ModelFilter) -> Result<Vec<Model>, ServiceError> {
        // Apply business logic to filter (e.g., access control, data sanitization)
        let sanitized_filter = self.sanitize_filter(filter);

        // Get all models from database (we'll implement filtering at service level for now)
        let basic_models = self.database_service.repository().get_all_models().await?;

        // Convert to service models
        let mut service_models = Vec::new();
        for basic_table in basic_models {
            let basic_model: BasicModel = basic_table.try_into()?;
            service_models.push(basic_model_to_service(&basic_model)?);
        }

        // Apply filtering
        let filtered_models = self.apply_filter(service_models, sanitized_filter);

        Ok(filtered_models)
    }

    /// Update a model
    pub async fn update_model(&self, id: Uuid, request: UpdateModelRequest) -> Result<Model, ServiceError> {
        // Validate the update request
        crate::validation::validate_update_model(&request)?;

        // Get existing model
        let existing = self.get_model(id).await?
            .ok_or_else(|| ServiceError::NotFound(format!("Model with ID {} not found", id)))?;

        // Apply updates
        let updated_model = self.apply_model_updates(existing, request)?;

        // Convert to basic model and save
        let basic_model = service_model_to_basic_update(&updated_model)?;
        let updated_basic = self.database_service.repository().update_model(&basic_model.try_into()?).await?;

        // Convert back to service model
        let updated_basic_model: BasicModel = updated_basic.try_into()?;
        basic_model_to_service(&updated_basic_model)
    }

    /// Delete a model
    pub async fn delete_model(&self, id: Uuid) -> Result<bool, ServiceError> {
        // Check if model exists and can be deleted
        let model = self.get_model(id).await?;
        if model.is_none() {
            return Ok(false);
        }

        // Business logic: check if model is in use, etc.
        self.validate_model_deletion(id).await?;

        // Delete from database
        Ok(self.database_service.repository().delete_model(id).await?)
    }

    /// Get installed models
    pub async fn get_installed_models(&self) -> Result<Vec<InstalledModel>, ServiceError> {
        let basic_installed = self.database_service.repository().get_installed_models().await?;

        let mut service_installed = Vec::new();
        for (basic_model_table, basic_installed_table) in basic_installed {
            let basic_installed = burncloud_database_models::db_to_basic_installed_model((basic_model_table, basic_installed_table))?;
            service_installed.push(basic_installed_model_to_service(&basic_installed)?);
        }

        Ok(service_installed)
    }

    /// Install a model
    pub async fn install_model(&self, model_id: Uuid, install_path: String) -> Result<InstalledModel, ServiceError> {
        // Validate installation request
        self.validate_model_installation(model_id, &install_path).await?;

        // Create installation record
        let basic_installed = self.database_service.repository().install_model(model_id, install_path).await?;

        // Get the model data to construct full InstallModel
        let model_table = self.database_service.repository().get_model_by_id(model_id).await?
            .ok_or_else(|| ServiceError::NotFound(format!("Model {} not found", model_id)))?;

        let basic_installed_model = burncloud_database_models::db_to_basic_installed_model((model_table, basic_installed))?;
        basic_installed_model_to_service(&basic_installed_model)
    }

    /// Update model status
    pub async fn update_model_status(&self, model_id: Uuid, status: ModelStatus) -> Result<(), ServiceError> {
        // Convert to basic status
        let basic_status = service_status_to_basic(status);

        // Update in database
        self.database_service.repository().update_model_status(model_id, basic_status.to_string()).await?;

        Ok(())
    }

    /// Get model statistics
    pub async fn get_model_stats(&self) -> Result<ModelServiceStats, ServiceError> {
        let stats = self.database_service.get_statistics().await?;

        // Convert database statistics to service statistics
        let mut models_by_type = HashMap::new();
        for (type_str, count) in stats.models_by_type {
            if let Ok(model_type) = type_str.parse::<ModelType>() {
                models_by_type.insert(model_type, count);
            }
        }

        // Count running models from installed models
        let installed_models = self.get_installed_models().await?;
        let running_count = installed_models.iter()
            .filter(|m| m.status == ModelStatus::Running)
            .count();

        Ok(ModelServiceStats {
            total_models: stats.total_models,
            installed_count: stats.installed_count,
            official_count: stats.official_count,
            running_count,
            total_size_bytes: stats.total_size_bytes as u64,
            models_by_type,
        })
    }

    // Private helper methods

    fn sanitize_filter(&self, filter: ModelFilter) -> ModelFilter {
        // Apply any business logic for filter sanitization
        // For example, limit the maximum results, validate search terms, etc.
        let mut sanitized = filter;

        // Limit results to reasonable maximum
        if let Some(limit) = sanitized.limit {
            sanitized.limit = Some(limit.min(1000));
        } else {
            sanitized.limit = Some(100); // Default limit
        }

        sanitized
    }

    fn apply_filter(&self, models: Vec<Model>, filter: ModelFilter) -> Vec<Model> {
        let mut filtered: Vec<Model> = models.into_iter()
            .filter(|model| {
                // Filter by model type
                if let Some(filter_type) = &filter.model_type {
                    if &model.model_type != filter_type {
                        return false;
                    }
                }

                // Filter by provider
                if let Some(filter_provider) = &filter.provider {
                    if &model.provider != filter_provider {
                        return false;
                    }
                }

                // Filter by official status
                if let Some(filter_official) = filter.is_official {
                    if model.is_official != filter_official {
                        return false;
                    }
                }

                // Filter by search query
                if let Some(search) = &filter.search {
                    let search_lower = search.to_lowercase();
                    let name_matches = model.name.to_lowercase().contains(&search_lower);
                    let display_name_matches = model.display_name.to_lowercase().contains(&search_lower);
                    let description_matches = model.description
                        .as_ref()
                        .map(|d| d.to_lowercase().contains(&search_lower))
                        .unwrap_or(false);

                    if !name_matches && !display_name_matches && !description_matches {
                        return false;
                    }
                }

                true
            })
            .collect();

        // Apply offset and limit
        if let Some(offset) = filter.offset {
            if offset as usize >= filtered.len() {
                return Vec::new();
            }
            filtered = filtered.into_iter().skip(offset as usize).collect();
        }

        if let Some(limit) = filter.limit {
            filtered.truncate(limit as usize);
        }

        filtered
    }

    fn apply_model_updates(&self, mut model: Model, request: UpdateModelRequest) -> Result<Model, ServiceError> {
        // Apply updates field by field
        if let Some(display_name) = request.display_name {
            model.display_name = display_name;
        }

        if let Some(description) = request.description {
            model.description = Some(description);
        }

        if let Some(version) = request.version {
            model.version = version;
        }

        if let Some(license) = request.license {
            model.license = Some(license);
        }

        if let Some(tags) = request.tags {
            model.tags = tags;
        }

        if let Some(languages) = request.languages {
            model.languages = languages;
        }

        if let Some(file_path) = request.file_path {
            model.file_path = Some(file_path);
        }

        if let Some(download_url) = request.download_url {
            model.download_url = Some(download_url);
        }

        if let Some(config) = request.config {
            model.config = config;
        }

        if let Some(rating) = request.rating {
            model.rating = Some(rating);
        }

        // Update timestamp
        model.updated_at = Utc::now();

        Ok(model)
    }

    async fn validate_model_deletion(&self, _model_id: Uuid) -> Result<(), ServiceError> {
        // Add business logic for deletion validation
        // For example: check if model is currently running, has dependencies, etc.
        Ok(())
    }

    async fn validate_model_installation(&self, model_id: Uuid, install_path: &str) -> Result<(), ServiceError> {
        // Validate model exists
        self.get_model(model_id).await?
            .ok_or_else(|| ServiceError::NotFound(format!("Model {} not found", model_id)))?;

        // Validate install path
        if install_path.is_empty() {
            return Err(ServiceError::Validation("Install path cannot be empty".to_string()));
        }

        // Check if path is writable, has enough space, etc.
        Ok(())
    }
}

/// Service-level model statistics
#[derive(Debug, Clone)]
pub struct ModelServiceStats {
    pub total_models: usize,
    pub installed_count: usize,
    pub official_count: usize,
    pub running_count: usize,
    pub total_size_bytes: u64,
    pub models_by_type: HashMap<ModelType, usize>,
}

// Conversion functions between service and basic types

fn service_model_to_basic_update(model: &Model) -> Result<BasicModel, ServiceError> {
    Ok(BasicModel {
        id: model.id,
        name: model.name.clone(),
        display_name: model.display_name.clone(),
        description: model.description.clone(),
        version: model.version.clone(),
        model_type: service_type_to_basic(model.model_type),
        size_category: service_size_to_basic(model.size_category),
        file_size: model.file_size,
        provider: model.provider.clone(),
        license: model.license.clone(),
        tags: model.tags.clone(),
        languages: model.languages.clone(),
        file_path: model.file_path.clone(),
        checksum: model.checksum.clone(),
        download_url: model.download_url.clone(),
        config: model.config.clone(),
        rating: model.rating,
        download_count: model.download_count,
        is_official: model.is_official,
        created_at: model.created_at,
        updated_at: model.updated_at,
    })
}

fn basic_model_to_service(basic: &BasicModel) -> Result<Model, ServiceError> {
    Ok(Model {
        id: basic.id,
        name: basic.name.clone(),
        display_name: basic.display_name.clone(),
        description: basic.description.clone(),
        version: basic.version.clone(),
        model_type: basic_type_to_service(basic.model_type),
        size_category: basic_size_to_service(basic.size_category),
        file_size: basic.file_size,
        provider: basic.provider.clone(),
        license: basic.license.clone(),
        tags: basic.tags.clone(),
        languages: basic.languages.clone(),
        file_path: basic.file_path.clone(),
        checksum: basic.checksum.clone(),
        download_url: basic.download_url.clone(),
        config: basic.config.clone(),
        rating: basic.rating,
        download_count: basic.download_count,
        is_official: basic.is_official,
        created_at: basic.created_at,
        updated_at: basic.updated_at,
    })
}

fn basic_installed_model_to_service(basic: &BasicInstalledModel) -> Result<InstalledModel, ServiceError> {
    let model = basic_model_to_service(&basic.model)?;

    Ok(InstalledModel {
        id: basic.id,
        model,
        install_path: basic.install_path.clone(),
        installed_at: basic.installed_at,
        status: basic_status_to_service(basic.status),
        port: basic.port.map(|p| p as u16), // Convert u32 to u16
        process_id: basic.process_id,
        last_used: basic.last_used,
        usage_count: basic.usage_count,
        created_at: basic.created_at,
        updated_at: basic.updated_at,
    })
}

fn service_type_to_basic(service_type: ModelType) -> BasicModelType {
    match service_type {
        ModelType::Chat => BasicModelType::Chat,
        ModelType::Code => BasicModelType::Code,
        ModelType::Text => BasicModelType::Text,
        ModelType::Embedding => BasicModelType::Embedding,
        ModelType::Image | ModelType::ImageGeneration => BasicModelType::Image,
        ModelType::Audio | ModelType::Speech => BasicModelType::Audio,
        ModelType::Video => BasicModelType::Video,
        ModelType::Multimodal => BasicModelType::Multimodal,
        ModelType::Other => BasicModelType::Other,
    }
}

fn basic_type_to_service(basic_type: BasicModelType) -> ModelType {
    match basic_type {
        BasicModelType::Chat => ModelType::Chat,
        BasicModelType::Code => ModelType::Code,
        BasicModelType::Text => ModelType::Text,
        BasicModelType::Embedding => ModelType::Embedding,
        BasicModelType::Image => ModelType::Image,
        BasicModelType::Audio => ModelType::Audio,
        BasicModelType::Video => ModelType::Video,
        BasicModelType::Multimodal => ModelType::Multimodal,
        BasicModelType::Other => ModelType::Other,
    }
}

fn service_size_to_basic(service_size: SizeCategory) -> BasicSizeCategory {
    match service_size {
        SizeCategory::Small => BasicSizeCategory::Small,
        SizeCategory::Medium => BasicSizeCategory::Medium,
        SizeCategory::Large => BasicSizeCategory::Large,
        SizeCategory::XLarge => BasicSizeCategory::XLarge,
    }
}

fn basic_size_to_service(basic_size: BasicSizeCategory) -> SizeCategory {
    match basic_size {
        BasicSizeCategory::Small => SizeCategory::Small,
        BasicSizeCategory::Medium => SizeCategory::Medium,
        BasicSizeCategory::Large => SizeCategory::Large,
        BasicSizeCategory::XLarge => SizeCategory::XLarge,
    }
}

fn service_status_to_basic(service_status: ModelStatus) -> BasicModelStatus {
    match service_status {
        ModelStatus::Running => BasicModelStatus::Running,
        ModelStatus::Starting => BasicModelStatus::Starting,
        ModelStatus::Stopping => BasicModelStatus::Stopping,
        ModelStatus::Stopped => BasicModelStatus::Stopped,
        ModelStatus::Error => BasicModelStatus::Error,
    }
}

fn basic_status_to_service(basic_status: BasicModelStatus) -> ModelStatus {
    match basic_status {
        BasicModelStatus::Running => ModelStatus::Running,
        BasicModelStatus::Starting => ModelStatus::Starting,
        BasicModelStatus::Stopping => ModelStatus::Stopping,
        BasicModelStatus::Stopped => ModelStatus::Stopped,
        BasicModelStatus::Error => ModelStatus::Error,
    }
}