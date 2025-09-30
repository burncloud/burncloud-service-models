use thiserror::Error;

/// Service layer error types
#[derive(Debug, Error)]
pub enum ServiceError {
    /// Database operation failed
    #[error("Database error: {0}")]
    Database(String),

    /// Input validation failed
    #[error("Validation error: {0}")]
    Validation(String),

    /// Business rule violation
    #[error("Business rule violation: {0}")]
    BusinessRule(String),

    /// Requested resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Operation not authorized
    #[error("Insufficient permissions: {0}")]
    Unauthorized(String),

    /// Resource already exists (conflict)
    #[error("Resource conflict: {0}")]
    Conflict(String),

    /// Invalid input data
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Internal service error
    #[error("Internal service error: {0}")]
    Internal(String),

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// UUID parsing error
    #[error("Invalid UUID: {0}")]
    InvalidUuid(#[from] uuid::Error),
}

// Add From implementations for common error types
impl From<burncloud_database_models::DatabaseError> for ServiceError {
    fn from(err: burncloud_database_models::DatabaseError) -> Self {
        ServiceError::Database(err.to_string())
    }
}

impl From<String> for ServiceError {
    fn from(err: String) -> Self {
        ServiceError::Internal(err)
    }
}

impl ServiceError {
    /// Create a new validation error
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    /// Create a new business rule error
    pub fn business_rule(msg: impl Into<String>) -> Self {
        Self::BusinessRule(msg.into())
    }

    /// Create a new not found error
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::NotFound(resource.into())
    }

    /// Create a new unauthorized error
    pub fn unauthorized(msg: impl Into<String>) -> Self {
        Self::Unauthorized(msg.into())
    }

    /// Create a new conflict error
    pub fn conflict(msg: impl Into<String>) -> Self {
        Self::Conflict(msg.into())
    }

    /// Create a new invalid input error
    pub fn invalid_input(msg: impl Into<String>) -> Self {
        Self::InvalidInput(msg.into())
    }

    /// Create a new internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }

    /// Check if this error is a not found error
    pub fn is_not_found(&self) -> bool {
        matches!(self, Self::NotFound(_))
    }

    /// Check if this error is a validation error
    pub fn is_validation(&self) -> bool {
        matches!(self, Self::Validation(_))
    }

    /// Check if this error is a conflict error
    pub fn is_conflict(&self) -> bool {
        matches!(self, Self::Conflict(_))
    }

    /// Get the error code for API responses
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::Database(_) => "DATABASE_ERROR",
            Self::Validation(_) => "VALIDATION_ERROR",
            Self::BusinessRule(_) => "BUSINESS_RULE_VIOLATION",
            Self::NotFound(_) => "RESOURCE_NOT_FOUND",
            Self::Unauthorized(_) => "UNAUTHORIZED",
            Self::Conflict(_) => "RESOURCE_CONFLICT",
            Self::InvalidInput(_) => "INVALID_INPUT",
            Self::Internal(_) => "INTERNAL_ERROR",
            Self::Serialization(_) => "SERIALIZATION_ERROR",
            Self::InvalidUuid(_) => "INVALID_UUID",
        }
    }

    /// Get HTTP status code equivalent
    pub fn http_status(&self) -> u16 {
        match self {
            Self::Database(_) | Self::Internal(_) | Self::Serialization(_) => 500,
            Self::Validation(_) | Self::InvalidInput(_) | Self::InvalidUuid(_) => 400,
            Self::BusinessRule(_) => 422,
            Self::NotFound(_) => 404,
            Self::Unauthorized(_) => 401,
            Self::Conflict(_) => 409,
        }
    }
}

/// Result type for service operations
pub type ServiceResult<T> = Result<T, ServiceError>;

/// Validation result for input validation
#[derive(Debug)]
pub struct ValidationResult {
    /// Whether validation passed
    pub is_valid: bool,
    /// List of validation errors
    pub errors: Vec<String>,
}

impl ValidationResult {
    /// Create a successful validation result
    pub fn success() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
        }
    }

    /// Create a failed validation result with errors
    pub fn failure(errors: Vec<String>) -> Self {
        Self {
            is_valid: false,
            errors,
        }
    }

    /// Add an error to the validation result
    pub fn add_error(&mut self, error: String) {
        self.is_valid = false;
        self.errors.push(error);
    }

    /// Convert to ServiceError if validation failed
    pub fn into_result(self) -> ServiceResult<()> {
        if self.is_valid {
            Ok(())
        } else {
            Err(ServiceError::validation(self.errors.join("; ")))
        }
    }
}