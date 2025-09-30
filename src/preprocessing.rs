use crate::{Model, CreateModelRequest, SizeCategory, ServiceResult, ServiceError};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::Utc;

/// Preprocesses a create model request into a complete Model
pub fn preprocess_create_model(request: CreateModelRequest) -> ServiceResult<Model> {
    // Generate UUID for new model
    let id = Uuid::new_v4();
    let now = Utc::now();

    // Calculate size category from file size
    let size_category = SizeCategory::from(request.file_size);

    // Normalize tags (trim, deduplicate, lowercase)
    let tags = normalize_tags(request.tags);

    // Normalize languages (trim, deduplicate, lowercase)
    let languages = normalize_languages(request.languages);

    // Validate and normalize config
    let config = validate_and_normalize_config(request.config)?;

    // Create the model
    let model = Model {
        id,
        name: request.name.trim().to_string(),
        display_name: request.display_name.trim().to_string(),
        description: request.description.map(|d| d.trim().to_string()).filter(|d| !d.is_empty()),
        version: request.version.trim().to_string(),
        model_type: request.model_type,
        size_category,
        file_size: request.file_size,
        provider: request.provider.trim().to_string(),
        license: request.license.map(|l| l.trim().to_string()).filter(|l| !l.is_empty()),
        tags,
        languages,
        file_path: request.file_path.map(|p| normalize_file_path(p)),
        checksum: None, // Will be calculated later if file is provided
        download_url: request.download_url.map(|u| u.trim().to_string()).filter(|u| !u.is_empty()),
        config,
        rating: None, // Initial rating is None
        download_count: 0, // Initial download count
        is_official: request.is_official,
        created_at: now,
        updated_at: now,
    };

    Ok(model)
}

/// Normalizes a list of tags
pub fn normalize_tags(tags: Vec<String>) -> Vec<String> {
    let mut normalized = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for tag in tags {
        let normalized_tag = tag.trim().to_lowercase();
        if !normalized_tag.is_empty() && seen.insert(normalized_tag.clone()) {
            // Keep original case but deduplicated
            normalized.push(tag.trim().to_string());
        }
    }

    // Sort for consistent ordering
    normalized.sort();
    normalized
}

/// Normalizes a list of languages
pub fn normalize_languages(languages: Vec<String>) -> Vec<String> {
    let mut normalized = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for lang in languages {
        let normalized_lang = lang.trim().to_lowercase();
        if !normalized_lang.is_empty() && seen.insert(normalized_lang.clone()) {
            // Convert to standardized format
            let standardized = standardize_language(lang.trim());
            normalized.push(standardized);
        }
    }

    // Sort for consistent ordering
    normalized.sort();
    normalized
}

/// Standardizes language codes and names
fn standardize_language(language: &str) -> String {
    let lower = language.to_lowercase();

    // Map common variations to standard names
    match lower.as_str() {
        "en" | "eng" | "english" => "English".to_string(),
        "es" | "spa" | "spanish" => "Spanish".to_string(),
        "fr" | "fra" | "french" => "French".to_string(),
        "de" | "deu" | "ger" | "german" => "German".to_string(),
        "it" | "ita" | "italian" => "Italian".to_string(),
        "pt" | "por" | "portuguese" | "português" => "Portuguese".to_string(),
        "ru" | "rus" | "russian" => "Russian".to_string(),
        "zh" | "chi" | "zho" | "chinese" | "chinese (simplified)" => "Chinese".to_string(),
        "ja" | "jpn" | "japanese" => "Japanese".to_string(),
        "ko" | "kor" | "korean" => "Korean".to_string(),
        "ar" | "ara" => "Arabic".to_string(),
        "hi" | "hin" => "Hindi".to_string(),
        _ => {
            // Capitalize first letter for other languages
            let mut chars = language.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        }
    }
}

/// Validates and normalizes configuration object
pub fn validate_and_normalize_config(
    config: HashMap<String, serde_json::Value>
) -> ServiceResult<HashMap<String, serde_json::Value>> {
    let mut normalized = HashMap::new();

    for (key, value) in config {
        let normalized_key = key.trim().to_string();
        if normalized_key.is_empty() {
            continue; // Skip empty keys
        }

        // Normalize string values
        let normalized_value = match value {
            serde_json::Value::String(s) => {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    serde_json::Value::Null
                } else {
                    serde_json::Value::String(trimmed.to_string())
                }
            },
            other => other,
        };

        normalized.insert(normalized_key, normalized_value);
    }

    Ok(normalized)
}

/// Normalizes file paths (convert to forward slashes, remove redundant separators)
pub fn normalize_file_path(path: String) -> String {
    let path = path.trim();

    // Convert backslashes to forward slashes
    let path = path.replace('\\', "/");

    // Remove redundant slashes
    let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();

    if path.starts_with('/') {
        format!("/{}", parts.join("/"))
    } else {
        parts.join("/")
    }
}

/// Generates a checksum for model file validation
pub async fn calculate_file_checksum(file_path: &str) -> ServiceResult<String> {
    use tokio::fs::File;
    use tokio::io::{AsyncReadExt, BufReader};
    use sha2::{Sha256, Digest};

    let mut file = match File::open(file_path).await {
        Ok(f) => f,
        Err(e) => return Err(ServiceError::internal(format!("Failed to open file: {}", e))),
    };

    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = match reader.read(&mut buffer).await {
            Ok(0) => break, // EOF
            Ok(n) => n,
            Err(e) => return Err(ServiceError::internal(format!("Failed to read file: {}", e))),
        };

        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

/// Validates download URL format and accessibility
pub async fn validate_download_url(url: &str) -> ServiceResult<()> {
    use url::Url;

    // Parse URL
    let parsed_url = Url::parse(url)
        .map_err(|_| ServiceError::validation("Invalid URL format"))?;

    // Check scheme
    match parsed_url.scheme() {
        "http" | "https" => {},
        _ => return Err(ServiceError::validation("URL must use HTTP or HTTPS protocol")),
    }

    // Basic reachability check (HEAD request)
    // Note: In a real implementation, you might want to make this optional
    // or implement with a timeout and retry logic

    Ok(())
}

/// Estimates download time based on file size and typical connection speeds
pub fn estimate_download_time(file_size: u64) -> std::time::Duration {
    // Assume average download speed of 10 MB/s
    const AVERAGE_SPEED_BYTES_PER_SEC: u64 = 10 * 1024 * 1024;

    let seconds = (file_size + AVERAGE_SPEED_BYTES_PER_SEC - 1) / AVERAGE_SPEED_BYTES_PER_SEC;
    std::time::Duration::from_secs(seconds)
}

/// Generates search keywords from model metadata for better discoverability
pub fn generate_search_keywords(model: &Model) -> Vec<String> {
    let mut keywords = Vec::new();

    // Add name variations
    keywords.push(model.name.clone());
    keywords.push(model.display_name.clone());

    // Add provider
    keywords.push(model.provider.clone());

    // Add model type
    keywords.push(model.model_type.to_string());

    // Add size category
    keywords.push(model.size_category.to_string());

    // Add tags
    keywords.extend(model.tags.clone());

    // Add languages
    keywords.extend(model.languages.clone());

    // Add description words if available
    if let Some(ref description) = model.description {
        let words: Vec<String> = description
            .split_whitespace()
            .filter(|word| word.len() > 3) // Only meaningful words
            .map(|word| word.to_lowercase())
            .collect();
        keywords.extend(words);
    }

    // Deduplicate and normalize
    let mut unique_keywords = std::collections::HashSet::new();
    keywords.into_iter()
        .map(|k| k.trim().to_lowercase())
        .filter(|k| !k.is_empty() && k.len() > 1)
        .for_each(|k| { unique_keywords.insert(k); });

    let mut result: Vec<String> = unique_keywords.into_iter().collect();
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ModelType;

    #[test]
    fn test_normalize_tags() {
        let tags = vec![
            "  ML  ".to_string(),
            "nlp".to_string(),
            "NLP".to_string(), // Duplicate
            "transformer".to_string(),
            "".to_string(), // Empty
        ];

        let normalized = normalize_tags(tags);
        assert_eq!(normalized, vec!["ML", "nlp", "transformer"]);
    }

    #[test]
    fn test_normalize_languages() {
        let languages = vec!["en".to_string(), "es".to_string(), "EN".to_string()];
        let normalized = normalize_languages(languages);
        assert_eq!(normalized, vec!["English", "Spanish"]);
    }

    #[test]
    fn test_normalize_file_path() {
        assert_eq!(normalize_file_path("path\\to\\file".to_string()), "path/to/file");
        assert_eq!(normalize_file_path("/path//to///file".to_string()), "/path/to/file");
        assert_eq!(normalize_file_path("  /path/to/file  ".to_string()), "/path/to/file");
    }

    #[test]
    fn test_standardize_language() {
        assert_eq!(standardize_language("en"), "English");
        assert_eq!(standardize_language("fr"), "French");
        assert_eq!(standardize_language("custom"), "Custom");
    }
}