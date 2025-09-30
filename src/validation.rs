use crate::{CreateModelRequest, UpdateModelRequest, ValidationResult, ServiceError, ServiceResult, ModelType};
use std::collections::HashMap;
use validator::{Validate, ValidationErrors};
use regex::Regex;
use std::sync::OnceLock;

/// Regex pattern for valid model names
static MODEL_NAME_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn get_model_name_regex() -> &'static Regex {
    MODEL_NAME_REGEX.get_or_init(|| {
        Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap()
    })
}

/// Validates create model requests
pub fn validate_create_model(request: &CreateModelRequest) -> ServiceResult<()> {
    let mut result = ValidationResult::success();

    // Use validator crate for basic validation
    if let Err(errors) = request.validate() {
        add_validation_errors(&mut result, errors);
    }

    // Additional business validation
    validate_model_name(&request.name, &mut result);
    validate_tags(&request.tags, &mut result);
    validate_languages(&request.languages, &mut result);
    validate_config(&request.config, &mut result);

    // Validate version format
    if let Err(err) = validate_version(&request.version) {
        result.add_error(err.to_string());
    }

    result.into_result()
}

/// Validates update model requests
pub fn validate_update_model(request: &UpdateModelRequest) -> ServiceResult<()> {
    let mut result = ValidationResult::success();

    // Check that at least one field is being updated
    if is_empty_update(request) {
        result.add_error("At least one field must be provided for update".to_string());
        return result.into_result();
    }

    // Use validator crate for basic validation
    if let Err(errors) = request.validate() {
        add_validation_errors(&mut result, errors);
    }

    // Additional business validation
    if let Some(ref tags) = request.tags {
        validate_tags(tags, &mut result);
    }
    if let Some(ref languages) = request.languages {
        validate_languages(languages, &mut result);
    }
    if let Some(ref config) = request.config {
        validate_config(config, &mut result);
    }

    // Validate version format if provided
    if let Some(ref version) = request.version {
        if let Err(err) = validate_version(version) {
            result.add_error(err.to_string());
        }
    }

    result.into_result()
}

/// Validates model name format and uniqueness requirements
fn validate_model_name(name: &str, result: &mut ValidationResult) {
    let name_regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$").unwrap(); // Must start with letter

    if name.is_empty() {
        result.add_error("Model name cannot be empty".to_string());
        return;
    }

    if name.len() > 100 {
        result.add_error("Model name cannot exceed 100 characters".to_string());
    }

    if !name_regex.is_match(name) {
        result.add_error("Model name must start with a letter and can only contain letters, numbers, underscores, and hyphens".to_string());
    }

    // Check for reserved names
    let reserved_names = vec!["admin", "api", "system", "root", "config", "public", "private"];
    if reserved_names.contains(&name.to_lowercase().as_str()) {
        result.add_error(format!("Model name '{}' is reserved", name));
    }
}

/// Validates tags array
fn validate_tags(tags: &[String], result: &mut ValidationResult) {
    if tags.len() > 20 {
        result.add_error("Maximum 20 tags allowed".to_string());
    }

    for (i, tag) in tags.iter().enumerate() {
        if tag.is_empty() {
            result.add_error(format!("Tag {} cannot be empty", i + 1));
        } else if tag.len() > 50 {
            result.add_error(format!("Tag {} cannot exceed 50 characters", i + 1));
        } else if !is_valid_tag_format(tag) {
            result.add_error(format!("Tag {} contains invalid characters", i + 1));
        }
    }

    // Check for duplicate tags
    let mut unique_tags = std::collections::HashSet::new();
    for tag in tags {
        let lowercase_tag = tag.to_lowercase();
        if !unique_tags.insert(lowercase_tag) {
            result.add_error(format!("Duplicate tag: {}", tag));
        }
    }
}

/// Validates languages array
fn validate_languages(languages: &[String], result: &mut ValidationResult) {
    if languages.len() > 10 {
        result.add_error("Maximum 10 languages allowed".to_string());
    }

    for (i, language) in languages.iter().enumerate() {
        if language.is_empty() {
            result.add_error(format!("Language {} cannot be empty", i + 1));
        } else if language.len() > 50 {
            result.add_error(format!("Language {} cannot exceed 50 characters", i + 1));
        } else if !is_valid_language_code(language) {
            result.add_error(format!("Language {} is not a valid language code or name", i + 1));
        }
    }
}

/// Validates configuration object
fn validate_config(config: &HashMap<String, serde_json::Value>, result: &mut ValidationResult) {
    if config.len() > 100 {
        result.add_error("Configuration cannot have more than 100 keys".to_string());
    }

    for (key, value) in config {
        if key.is_empty() {
            result.add_error("Configuration key cannot be empty".to_string());
        } else if key.len() > 100 {
            result.add_error(format!("Configuration key '{}' cannot exceed 100 characters", key));
        }

        // Validate value size (prevent extremely large configs)
        if let Ok(serialized) = serde_json::to_string(value) {
            if serialized.len() > 10_000 {
                result.add_error(format!("Configuration value for key '{}' is too large", key));
            }
        }
    }
}

/// Check if tag format is valid (alphanumeric with some special characters)
fn is_valid_tag_format(tag: &str) -> bool {
    let tag_regex = Regex::new(r"^[a-zA-Z0-9\s\-_\.]+$").unwrap();
    tag_regex.is_match(tag)
}

/// Check if language code is valid (basic validation)
fn is_valid_language_code(language: &str) -> bool {
    // Accept common language codes and names
    let language_lower = language.to_lowercase();

    // ISO 639-1 codes (2-letter)
    let iso_639_1 = vec![
        "en", "es", "fr", "de", "it", "pt", "ru", "zh", "ja", "ko",
        "ar", "hi", "bn", "ur", "fa", "tr", "pl", "nl", "sv", "da",
        "no", "fi", "cs", "hu", "ro", "el", "he", "th", "vi", "id",
    ];

    // Common language names
    let language_names = vec![
        "english", "spanish", "french", "german", "italian", "portuguese",
        "russian", "chinese", "japanese", "korean", "arabic", "hindi",
        "bengali", "urdu", "persian", "turkish", "polish", "dutch",
        "swedish", "danish", "norwegian", "finnish", "czech", "hungarian",
        "romanian", "greek", "hebrew", "thai", "vietnamese", "indonesian",
    ];

    // Check if it's a valid ISO code or language name
    if language_lower.len() == 2 && iso_639_1.contains(&language_lower.as_str()) {
        return true;
    }

    if language_names.contains(&language_lower.as_str()) {
        return true;
    }

    // Also accept longer language names if they contain only letters and spaces
    let name_regex = Regex::new(r"^[a-zA-Z\s]+$").unwrap();
    name_regex.is_match(language) && language.len() <= 50
}

/// Check if update request is empty
fn is_empty_update(request: &UpdateModelRequest) -> bool {
    request.display_name.is_none() &&
    request.description.is_none() &&
    request.version.is_none() &&
    request.license.is_none() &&
    request.tags.is_none() &&
    request.languages.is_none() &&
    request.file_path.is_none() &&
    request.download_url.is_none() &&
    request.config.is_none() &&
    request.rating.is_none()
}

/// Convert validator errors to validation result
fn add_validation_errors(result: &mut ValidationResult, errors: ValidationErrors) {
    for (field, field_errors) in errors.field_errors() {
        for error in field_errors {
            let message = match error.message {
                Some(ref msg) => format!("{}: {}", field, msg),
                None => format!("{}: Invalid value", field),
            };
            result.add_error(message);
        }
    }
}

/// Validates model type string
pub fn validate_model_type(model_type_str: &str) -> ServiceResult<ModelType> {
    model_type_str.parse().map_err(|e| ServiceError::validation(e))
}

/// Validates file size is reasonable
pub fn validate_file_size(size: u64) -> ServiceResult<()> {
    const MAX_FILE_SIZE: u64 = 1024 * 1024 * 1024 * 200; // 200GB max

    if size == 0 {
        return Err(ServiceError::validation("File size must be greater than 0"));
    }

    if size > MAX_FILE_SIZE {
        return Err(ServiceError::validation("File size exceeds maximum allowed size (200GB)"));
    }

    Ok(())
}

/// Validates version string format (basic semantic versioning)
pub fn validate_version(version: &str) -> ServiceResult<()> {
    let version_regex = Regex::new(r"^\d+\.\d+\.\d+(\-[a-zA-Z0-9\-]+)?(\+[a-zA-Z0-9\-\.]+)?$").unwrap();

    if version_regex.is_match(version) {
        Ok(())
    } else {
        Err(ServiceError::validation("Version must follow semantic versioning format (e.g., 1.0.0)"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ModelType;
    use std::collections::HashMap;

    #[test]
    fn test_validate_model_name() {
        let mut result = ValidationResult::success();

        // Valid names
        validate_model_name("my-model", &mut result);
        assert!(result.is_valid);

        result = ValidationResult::success();
        validate_model_name("model_v2", &mut result);
        assert!(result.is_valid);

        // Invalid names
        result = ValidationResult::success();
        validate_model_name("model with spaces", &mut result);
        assert!(!result.is_valid);

        result = ValidationResult::success();
        validate_model_name("admin", &mut result);
        assert!(!result.is_valid);
    }

    #[test]
    fn test_validate_tags() {
        let mut result = ValidationResult::success();

        // Valid tags
        let tags = vec!["ml".to_string(), "nlp".to_string(), "transformer".to_string()];
        validate_tags(&tags, &mut result);
        assert!(result.is_valid);

        // Invalid tags (duplicates)
        result = ValidationResult::success();
        let tags = vec!["ml".to_string(), "ML".to_string()];
        validate_tags(&tags, &mut result);
        assert!(!result.is_valid);
    }

    #[test]
    fn test_validate_version() {
        assert!(validate_version("1.0.0").is_ok());
        assert!(validate_version("2.1.3-alpha").is_ok());
        assert!(validate_version("1.0.0+build.1").is_ok());

        assert!(validate_version("1.0").is_err());
        assert!(validate_version("v1.0.0").is_err());
        assert!(validate_version("invalid").is_err());
    }
}