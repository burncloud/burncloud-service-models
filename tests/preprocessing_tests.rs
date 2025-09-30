//! Unit tests for preprocessing logic
//!
//! Tests data preprocessing and normalization functions.

use burncloud_service_models::{
    preprocessing::{preprocess_create_model, normalize_tags},
    CreateModelRequest, ModelType
};
use std::collections::HashMap;

fn create_test_request() -> CreateModelRequest {
    CreateModelRequest {
        name: "  test-model  ".to_string(),
        display_name: "  Test Model Display  ".to_string(),
        version: "1.0.0".to_string(),
        model_type: ModelType::Chat,
        provider: "  TestProvider  ".to_string(),
        file_size: 5_000_000_000,
        description: Some("  A test description with extra spaces  ".to_string()),
        license: Some("  MIT  ".to_string()),
        tags: vec![
            "  AI  ".to_string(),
            "machine-learning".to_string(),
            "AI".to_string(), // Duplicate
            "  CHAT  ".to_string(),
            "".to_string(), // Empty
        ],
        languages: vec![
            "  English  ".to_string(),
            "spanish".to_string(),
            "FRENCH".to_string(),
            "english".to_string(), // Duplicate
        ],
        file_path: Some("  /path/to/model.bin  ".to_string()),
        download_url: Some("  https://example.com/model  ".to_string()),
        config: HashMap::new(),
        is_official: false,
    }
}

#[test]
fn test_preprocess_create_model() {
    let request = create_test_request();
    let processed = preprocess_create_model(request).unwrap();

    // Test string trimming
    assert_eq!(processed.name, "test-model");
    assert_eq!(processed.display_name, "Test Model Display");
    assert_eq!(processed.provider, "TestProvider");
    assert_eq!(processed.description, Some("A test description with extra spaces".to_string()));
    assert_eq!(processed.license, Some("MIT".to_string()));
    assert_eq!(processed.file_path, Some("/path/to/model.bin".to_string()));
    assert_eq!(processed.download_url, Some("https://example.com/model".to_string()));

    // Test tag normalization
    assert!(processed.tags.contains(&"AI".to_string()));
    assert!(processed.tags.contains(&"machine-learning".to_string()));
    assert!(processed.tags.contains(&"CHAT".to_string()));
    assert!(!processed.tags.contains(&"".to_string())); // Empty removed
    assert_eq!(processed.tags.iter().filter(|t| t.to_lowercase() == "ai").count(), 1); // Duplicates removed

    // Test language normalization
    assert!(processed.languages.contains(&"English".to_string()));
    assert!(processed.languages.contains(&"Spanish".to_string()));
    assert!(processed.languages.contains(&"French".to_string()));
    assert_eq!(processed.languages.iter().filter(|l| l.to_lowercase() == "english").count(), 1);
}

#[test]
fn test_normalize_tags() {
    let tags = vec![
        "  AI  ".to_string(),
        "Machine-Learning".to_string(),
        "ai".to_string(), // Duplicate
        "DEEP_LEARNING".to_string(),
        "".to_string(), // Empty
        "  ".to_string(), // Whitespace only
    ];

    let normalized = normalize_tags(tags);

    assert_eq!(normalized.len(), 3);
    assert!(normalized.contains(&"AI".to_string()));
    assert!(normalized.contains(&"Machine-Learning".to_string()));
    assert!(normalized.contains(&"DEEP_LEARNING".to_string()));
    assert!(!normalized.contains(&"".to_string()));
}

#[test]
fn test_normalize_tags_edge_cases() {
    // Empty input
    let empty_tags: Vec<String> = vec![];
    let normalized = normalize_tags(empty_tags);
    assert!(normalized.is_empty());

    // All empty/whitespace tags
    let whitespace_tags = vec!["".to_string(), "  ".to_string(), "\t\n".to_string()];
    let normalized = normalize_tags(whitespace_tags);
    assert!(normalized.is_empty());

    // Special characters
    let special_tags = vec![
        "ai/ml".to_string(),
        "machine_learning".to_string(),
        "deep-learning".to_string(),
        "tag@with#symbols".to_string(),
    ];
    let normalized = normalize_tags(special_tags);
    assert!(normalized.contains(&"ai/ml".to_string()));
    assert!(normalized.contains(&"machine_learning".to_string()));
    assert!(normalized.contains(&"deep-learning".to_string()));
    assert!(normalized.contains(&"tag@with#symbols".to_string()));
}

#[test]
fn test_language_normalization() {
    let mut request = create_test_request();
    request.languages = vec![
        "english".to_string(),
        "SPANISH".to_string(),
        "french".to_string(),
        "Chinese (Simplified)".to_string(),
        "português".to_string(),
    ];

    let processed = preprocess_create_model(request).unwrap();

    // Languages should be title-cased
    assert!(processed.languages.contains(&"English".to_string()));
    assert!(processed.languages.contains(&"Spanish".to_string()));
    assert!(processed.languages.contains(&"French".to_string()));
    assert!(processed.languages.contains(&"Chinese".to_string())); // "Chinese (Simplified)" -> "Chinese"
    assert!(processed.languages.contains(&"Portuguese".to_string())); // "português" -> "Portuguese"
}

#[test]
fn test_file_size_preprocessing() {
    let mut request = create_test_request();

    // Test various file sizes
    let test_sizes = vec![
        1_000_000,      // 1MB
        1_000_000_000,  // 1GB
        5_000_000_000,  // 5GB
        15_000_000_000, // 15GB
        50_000_000_000, // 50GB
    ];

    for size in test_sizes {
        request.file_size = size;
        let processed = preprocess_create_model(request.clone()).unwrap();
        assert_eq!(processed.file_size, size);
        // Size category should be calculated automatically by the service layer
    }
}

#[test]
fn test_config_preprocessing() {
    let mut request = create_test_request();

    // Add some config data
    request.config.insert("temperature".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.7).unwrap()));
    request.config.insert("max_tokens".to_string(), serde_json::Value::Number(serde_json::Number::from(2048)));
    request.config.insert("model_class".to_string(), serde_json::Value::String("CausalLM".to_string()));

    let processed = preprocess_create_model(request).unwrap();

    // Config should be preserved as-is
    assert_eq!(processed.config.len(), 3);
    assert!(processed.config.contains_key("temperature"));
    assert!(processed.config.contains_key("max_tokens"));
    assert!(processed.config.contains_key("model_class"));
}

#[test]
fn test_url_preprocessing() {
    let mut request = create_test_request();

    // Test various URL formats
    let test_urls = vec![
        "  https://example.com/model.bin  ".to_string(),
        "http://localhost:8080/model".to_string(),
        "https://huggingface.co/models/llama-7b/resolve/main/pytorch_model.bin".to_string(),
    ];

    for url in test_urls {
        request.download_url = Some(url.clone());
        let processed = preprocess_create_model(request.clone()).unwrap();
        assert_eq!(processed.download_url, Some(url.trim().to_string()));
    }
}

#[test]
fn test_preprocessing_preserves_required_fields() {
    let request = create_test_request();
    let processed = preprocess_create_model(request.clone()).unwrap();

    // Required fields should be preserved
    assert_eq!(processed.name.trim(), request.name.trim());
    assert_eq!(processed.version, request.version);
    assert_eq!(processed.model_type, request.model_type);
    assert_eq!(processed.file_size, request.file_size);
    assert_eq!(processed.is_official, request.is_official);
}

#[test]
fn test_preprocessing_with_none_optional_fields() {
    let mut request = create_test_request();
    request.description = None;
    request.license = None;
    request.file_path = None;
    request.download_url = None;

    let processed = preprocess_create_model(request).unwrap();

    assert!(processed.description.is_none());
    assert!(processed.license.is_none());
    assert!(processed.file_path.is_none());
    assert!(processed.download_url.is_none());
}

#[test]
fn test_preprocessing_with_empty_collections() {
    let mut request = create_test_request();
    request.tags = vec![];
    request.languages = vec![];

    let processed = preprocess_create_model(request).unwrap();

    assert!(processed.tags.is_empty());
    assert!(processed.languages.is_empty());
}

#[test]
fn test_preprocessing_performance() {
    let request = create_test_request();

    // Process many times to test performance
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _processed = preprocess_create_model(request.clone()).unwrap();
    }
    let duration = start.elapsed();

    // Should complete 1000 operations quickly
    assert!(duration.as_millis() < 1000); // Less than 1 second for 1000 operations
}

#[test]
fn test_whitespace_edge_cases() {
    let mut request = create_test_request();

    // Test various whitespace scenarios
    request.name = "\t\ntest-model\r\n  ".to_string();
    request.display_name = "  \t  Test Model  \n\r  ".to_string();
    request.provider = "\n\nTestProvider\t\t".to_string();

    let processed = preprocess_create_model(request).unwrap();

    assert_eq!(processed.name, "test-model");
    assert_eq!(processed.display_name, "Test Model");
    assert_eq!(processed.provider, "TestProvider");
}

#[test]
fn test_unicode_handling() {
    let mut request = create_test_request();

    // Test Unicode characters
    request.name = "模型-测试".to_string();
    request.display_name = "机器学习模型".to_string();
    request.description = Some("这是一个测试模型的描述".to_string());
    request.tags = vec!["中文".to_string(), "测试".to_string()];
    request.languages = vec!["中文".to_string(), "english".to_string()];

    let processed = preprocess_create_model(request).unwrap();

    assert_eq!(processed.name, "模型-测试");
    assert_eq!(processed.display_name, "机器学习模型");
    assert_eq!(processed.description, Some("这是一个测试模型的描述".to_string()));
    assert!(processed.tags.contains(&"中文".to_string()));
    assert!(processed.languages.contains(&"中文".to_string()));
    assert!(processed.languages.contains(&"English".to_string()));
}