//! Unit tests for validation logic
//!
//! Tests the input validation functions for CreateModelRequest and UpdateModelRequest.

use burncloud_service_models::{
    validation::{validate_create_model, validate_update_model},
    CreateModelRequest, UpdateModelRequest, ModelType, ServiceError
};
use std::collections::HashMap;

fn create_valid_request() -> CreateModelRequest {
    CreateModelRequest {
        name: "valid-model-name".to_string(),
        display_name: "Valid Model Name".to_string(),
        version: "1.0.0".to_string(),
        model_type: ModelType::Chat,
        provider: "ValidProvider".to_string(),
        file_size: 1_000_000_000,
        description: Some("A valid description".to_string()),
        license: Some("MIT".to_string()),
        tags: vec!["valid".to_string(), "test".to_string()],
        languages: vec!["English".to_string()],
        file_path: None,
        download_url: Some("https://example.com/model.bin".to_string()),
        config: HashMap::new(),
        is_official: false,
    }
}

#[test]
fn test_valid_create_model_request() {
    let request = create_valid_request();
    let result = validate_create_model(&request);
    assert!(result.is_ok());
}

#[test]
fn test_invalid_model_name() {
    // Empty name
    let mut request = create_valid_request();
    request.name = "".to_string();
    assert!(validate_create_model(&request).is_err());

    // Too long name
    request.name = "a".repeat(101);
    assert!(validate_create_model(&request).is_err());

    // Invalid characters
    request.name = "invalid@name#123".to_string();
    assert!(validate_create_model(&request).is_err());

    // Name with spaces
    request.name = "invalid name".to_string();
    assert!(validate_create_model(&request).is_err());

    // Name starting with number
    request.name = "123invalid".to_string();
    assert!(validate_create_model(&request).is_err());
}

#[test]
fn test_valid_model_names() {
    let valid_names = vec![
        "llama-3-8b-chat",
        "codellama_7b",
        "mistral-7b-instruct-v2",
        "gpt-4o-mini",
        "claude-3-haiku",
        "a",
        "model123",
        "my-awesome-model_v1",
    ];

    for name in valid_names {
        let mut request = create_valid_request();
        request.name = name.to_string();
        assert!(validate_create_model(&request).is_ok(), "Name '{}' should be valid", name);
    }
}

#[test]
fn test_invalid_display_name() {
    let mut request = create_valid_request();

    // Empty display name
    request.display_name = "".to_string();
    assert!(validate_create_model(&request).is_err());

    // Too long display name
    request.display_name = "a".repeat(201);
    assert!(validate_create_model(&request).is_err());
}

#[test]
fn test_invalid_version() {
    let mut request = create_valid_request();

    // Empty version
    request.version = "".to_string();
    assert!(validate_create_model(&request).is_err());

    // Invalid version formats
    let invalid_versions = vec![
        "1",
        "1.0",
        "1.0.0.0",
        "v1.0.0",
        "not.a.version",
        "1.0.x",
    ];

    for version in invalid_versions {
        request.version = version.to_string();
        assert!(validate_create_model(&request).is_err(), "Version '{}' should be invalid", version);
    }
}

#[test]
fn test_valid_versions() {
    let valid_versions = vec![
        "1.0.0",
        "0.1.0",
        "10.20.30",
        "1.0.1",
        "2.1.0",
    ];

    for version in valid_versions {
        let mut request = create_valid_request();
        request.version = version.to_string();
        assert!(validate_create_model(&request).is_ok(), "Version '{}' should be valid", version);
    }
}

#[test]
fn test_invalid_provider() {
    let mut request = create_valid_request();

    // Empty provider
    request.provider = "".to_string();
    assert!(validate_create_model(&request).is_err());

    // Too long provider
    request.provider = "a".repeat(101);
    assert!(validate_create_model(&request).is_err());
}

#[test]
fn test_invalid_file_size() {
    let mut request = create_valid_request();

    // Zero file size
    request.file_size = 0;
    assert!(validate_create_model(&request).is_err());
}

#[test]
fn test_invalid_description() {
    let mut request = create_valid_request();

    // Too long description
    request.description = Some("a".repeat(2001));
    assert!(validate_create_model(&request).is_err());
}

#[test]
fn test_invalid_license() {
    let mut request = create_valid_request();

    // Too long license
    request.license = Some("a".repeat(201));
    assert!(validate_create_model(&request).is_err());
}

#[test]
fn test_invalid_tags() {
    let mut request = create_valid_request();

    // Too many tags
    request.tags = (0..21).map(|i| format!("tag{}", i)).collect();
    assert!(validate_create_model(&request).is_err());

    // Tag too long
    request.tags = vec!["a".repeat(51)];
    assert!(validate_create_model(&request).is_err());

    // Empty tag
    request.tags = vec!["".to_string()];
    assert!(validate_create_model(&request).is_err());
}

#[test]
fn test_invalid_languages() {
    let mut request = create_valid_request();

    // Too many languages
    request.languages = (0..11).map(|i| format!("Language{}", i)).collect();
    assert!(validate_create_model(&request).is_err());

    // Language too long
    request.languages = vec!["a".repeat(51)];
    assert!(validate_create_model(&request).is_err());

    // Empty language
    request.languages = vec!["".to_string()];
    assert!(validate_create_model(&request).is_err());
}

#[test]
fn test_invalid_download_url() {
    let mut request = create_valid_request();

    // Invalid URL format
    request.download_url = Some("not-a-url".to_string());
    assert!(validate_create_model(&request).is_err());

    // Empty URL
    request.download_url = Some("".to_string());
    assert!(validate_create_model(&request).is_err());
}

#[test]
fn test_valid_download_urls() {
    let valid_urls = vec![
        "https://example.com/model.bin",
        "http://localhost:8080/model",
        "https://huggingface.co/models/llama-7b/resolve/main/model.bin",
        "ftp://ftp.example.com/models/model.bin",
    ];

    for url in valid_urls {
        let mut request = create_valid_request();
        request.download_url = Some(url.to_string());
        assert!(validate_create_model(&request).is_ok(), "URL '{}' should be valid", url);
    }
}

#[test]
fn test_update_model_validation() {
    // Valid update with some fields
    let update_request = UpdateModelRequest {
        display_name: Some("Updated Name".to_string()),
        description: Some("Updated description".to_string()),
        version: Some("2.0.0".to_string()),
        rating: Some(4.5),
        ..Default::default()
    };
    assert!(validate_update_model(&update_request).is_ok());

    // Empty update (no fields provided) should fail
    let empty_update = UpdateModelRequest::default();
    assert!(validate_update_model(&empty_update).is_err());
}

#[test]
fn test_update_model_invalid_fields() {
    // Invalid display name
    let mut update = UpdateModelRequest {
        display_name: Some("".to_string()),
        ..Default::default()
    };
    assert!(validate_update_model(&update).is_err());

    // Invalid version
    update = UpdateModelRequest {
        version: Some("invalid.version".to_string()),
        ..Default::default()
    };
    assert!(validate_update_model(&update).is_err());

    // Invalid rating (too high)
    update = UpdateModelRequest {
        rating: Some(6.0),
        ..Default::default()
    };
    assert!(validate_update_model(&update).is_err());

    // Invalid rating (negative)
    update = UpdateModelRequest {
        rating: Some(-1.0),
        ..Default::default()
    };
    assert!(validate_update_model(&update).is_err());
}

#[test]
fn test_validation_error_messages() {
    let mut request = create_valid_request();
    request.name = "".to_string();

    match validate_create_model(&request) {
        Err(ServiceError::Validation(msg)) => {
            assert!(msg.contains("name") || msg.contains("required"));
        },
        _ => panic!("Expected validation error"),
    }
}

#[test]
fn test_edge_case_values() {
    let mut request = create_valid_request();

    // Minimum valid file size
    request.file_size = 1;
    assert!(validate_create_model(&request).is_ok());

    // Maximum file size (should be valid)
    request.file_size = u64::MAX;
    assert!(validate_create_model(&request).is_ok());

    // Single character name (valid)
    request.name = "a".to_string();
    assert!(validate_create_model(&request).is_ok());

    // Maximum length name (100 chars)
    request.name = "a".repeat(100);
    assert!(validate_create_model(&request).is_ok());

    // Single character display name (valid)
    request.display_name = "A".to_string();
    assert!(validate_create_model(&request).is_ok());

    // Maximum length display name (200 chars)
    request.display_name = "A".repeat(200);
    assert!(validate_create_model(&request).is_ok());
}