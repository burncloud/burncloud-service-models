//! Integration tests for burncloud-service-models
//!
//! Tests the complete integration between service layer, database layer,
//! and core components. These tests verify end-to-end functionality.

use burncloud_service_models::*;
use burncloud_database_core::create_in_memory_database;
use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;

/// Setup test environment with fresh database
async fn setup_integration_test() -> ModelsService {
    let db = Arc::new(create_in_memory_database().await.unwrap());
    ModelsService::new(db).await.unwrap()
}

/// Create a complete test model request
fn create_comprehensive_model_request(name: &str) -> CreateModelRequest {
    let mut config = HashMap::new();
    config.insert("temperature".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.7).unwrap()));
    config.insert("max_tokens".to_string(), serde_json::Value::Number(serde_json::Number::from(2048)));

    CreateModelRequest {
        name: name.to_string(),
        display_name: format!("{} - Advanced AI Model", name),
        version: "1.2.3".to_string(),
        model_type: ModelType::Chat,
        provider: "IntegrationTestProvider".to_string(),
        file_size: 7_500_000_000, // 7.5GB
        description: Some("A comprehensive test model for integration testing".to_string()),
        license: Some("MIT".to_string()),
        tags: vec!["integration".to_string(), "test".to_string(), "ai".to_string()],
        languages: vec!["English".to_string(), "Spanish".to_string()],
        file_path: Some("/models/integration/test.bin".to_string()),
        download_url: Some("https://test.example.com/models/integration-test.bin".to_string()),
        config,
        is_official: false,
    }
}

#[tokio::test]
async fn test_complete_model_lifecycle() {
    let service = setup_integration_test().await;

    // 1. Create model
    let request = create_comprehensive_model_request("lifecycle-test");
    let created = service.create_model(request).await.unwrap();

    assert_eq!(created.name, "lifecycle-test");
    assert_eq!(created.size_category, SizeCategory::Medium);
    assert_eq!(created.download_count, 0);
    assert!(!created.is_official);

    // 2. Retrieve model
    let retrieved = service.get_model(created.id).await.unwrap().unwrap();
    assert_eq!(retrieved.id, created.id);
    assert_eq!(retrieved.display_name, created.display_name);

    // 3. Update model
    let update_request = UpdateModelRequest {
        display_name: Some("Updated Lifecycle Test Model".to_string()),
        description: Some("Updated description for integration test".to_string()),
        rating: Some(4.7),
        tags: Some(vec!["updated".to_string(), "integration".to_string(), "v2".to_string()]),
        ..Default::default()
    };

    let updated = service.update_model(created.id, update_request).await.unwrap();
    assert_eq!(updated.display_name, "Updated Lifecycle Test Model");
    assert_eq!(updated.rating, Some(4.7));
    assert!(updated.tags.contains(&"updated".to_string()));

    // 4. Install model
    let install_path = "/opt/integration-test/lifecycle".to_string();
    let installed = service.install_model(created.id, install_path.clone()).await.unwrap();

    assert_eq!(installed.model.id, created.id);
    assert_eq!(installed.install_path, install_path);
    assert_eq!(installed.status, ModelStatus::Stopped);

    // 5. Update status
    service.update_model_status(created.id, ModelStatus::Starting).await.unwrap();
    service.update_model_status(created.id, ModelStatus::Running).await.unwrap();

    let installed_models = service.get_installed_models().await.unwrap();
    assert_eq!(installed_models.len(), 1);
    assert_eq!(installed_models[0].status, ModelStatus::Running);

    // 6. Get statistics
    let stats = service.get_model_stats().await.unwrap();
    assert_eq!(stats.total_models, 1);
    assert_eq!(stats.installed_count, 1);
    assert_eq!(stats.running_count, 1);

    // 7. Delete model (should handle installed model appropriately)
    let delete_result = service.delete_model(created.id).await;
    // Behavior depends on business logic - document what happens
    println!("Delete result for installed model: {:?}", delete_result);
}

#[tokio::test]
async fn test_multi_model_operations() {
    let service = setup_integration_test().await;

    // Create multiple models of different types
    let models_data = vec![
        ("chat-model-1", ModelType::Chat, 5_000_000_000),
        ("code-model-1", ModelType::Code, 3_000_000_000),
        ("text-model-1", ModelType::Text, 2_000_000_000),
        ("embedding-model-1", ModelType::Embedding, 1_000_000_000),
        ("image-model-1", ModelType::Image, 10_000_000_000),
    ];

    let mut created_models = vec![];

    for (name, model_type, file_size) in models_data {
        let mut request = create_comprehensive_model_request(name);
        request.model_type = model_type;
        request.file_size = file_size;
        request.is_official = name.contains("code") || name.contains("text");

        let created = service.create_model(request).await.unwrap();
        created_models.push(created);
    }

    // Test filtering
    let chat_models = service.list_models(ModelFilter {
        model_type: Some(ModelType::Chat),
        ..Default::default()
    }).await.unwrap();
    assert_eq!(chat_models.len(), 1);

    let official_models = service.list_models(ModelFilter {
        is_official: Some(true),
        ..Default::default()
    }).await.unwrap();
    assert_eq!(official_models.len(), 2);

    // Test search
    let search_results = service.list_models(ModelFilter {
        search: Some("model-1".to_string()),
        ..Default::default()
    }).await.unwrap();
    assert_eq!(search_results.len(), 5);

    // Install some models
    service.install_model(created_models[0].id, "/opt/chat-model".to_string()).await.unwrap();
    service.install_model(created_models[1].id, "/opt/code-model".to_string()).await.unwrap();
    service.install_model(created_models[2].id, "/opt/text-model".to_string()).await.unwrap();

    // Set different statuses
    service.update_model_status(created_models[0].id, ModelStatus::Running).await.unwrap();
    service.update_model_status(created_models[1].id, ModelStatus::Starting).await.unwrap();
    service.update_model_status(created_models[2].id, ModelStatus::Stopped).await.unwrap();

    // Test statistics
    let stats = service.get_model_stats().await.unwrap();
    assert_eq!(stats.total_models, 5);
    assert_eq!(stats.installed_count, 3);
    assert_eq!(stats.running_count, 1);
    assert_eq!(stats.official_count, 2);
    assert_eq!(stats.total_size_bytes, 21_000_000_000);

    // Check models by type
    assert_eq!(stats.models_by_type.get(&ModelType::Chat), Some(&1));
    assert_eq!(stats.models_by_type.get(&ModelType::Code), Some(&1));
    assert_eq!(stats.models_by_type.get(&ModelType::Text), Some(&1));
    assert_eq!(stats.models_by_type.get(&ModelType::Embedding), Some(&1));
    assert_eq!(stats.models_by_type.get(&ModelType::Image), Some(&1));
}

#[tokio::test]
async fn test_error_handling_integration() {
    let service = setup_integration_test().await;

    // Test duplicate model creation
    let request = create_comprehensive_model_request("duplicate-test");
    service.create_model(request.clone()).await.unwrap();

    // Second attempt should fail
    let duplicate_result = service.create_model(request).await;
    assert!(duplicate_result.is_err());

    // Test operations on non-existent models
    let fake_id = Uuid::new_v4();

    let get_result = service.get_model(fake_id).await.unwrap();
    assert!(get_result.is_none());

    let update_result = service.update_model(fake_id, UpdateModelRequest::default()).await;
    assert!(update_result.is_err());

    let install_result = service.install_model(fake_id, "/fake/path".to_string()).await;
    assert!(install_result.is_err());

    // Try to update status of non-existent model (might succeed or fail depending on implementation)
    let _status_result = service.update_model_status(fake_id, ModelStatus::Running).await;

    let delete_result = service.delete_model(fake_id).await.unwrap();
    assert!(!delete_result);
}

#[tokio::test]
async fn test_data_consistency_across_operations() {
    let service = setup_integration_test().await;

    // Create model
    let request = create_comprehensive_model_request("consistency-test");
    let original_file_size = request.file_size;
    let original_config = request.config.clone();

    let created = service.create_model(request).await.unwrap();

    // Install model
    let installed = service.install_model(created.id, "/opt/consistency".to_string()).await.unwrap();

    // Verify model data consistency
    assert_eq!(installed.model.id, created.id);
    assert_eq!(installed.model.name, created.name);
    assert_eq!(installed.model.file_size, original_file_size);
    assert_eq!(installed.model.config, original_config);

    // Update base model
    let update_request = UpdateModelRequest {
        description: Some("Updated consistency test description".to_string()),
        rating: Some(4.2),
        ..Default::default()
    };
    let updated = service.update_model(created.id, update_request).await.unwrap();

    // Get installed models and verify updates are reflected
    let installed_models = service.get_installed_models().await.unwrap();
    assert_eq!(installed_models.len(), 1);

    // The installed model should reflect base model updates
    // (behavior may vary depending on implementation)
    let installed_model = &installed_models[0];
    println!("Base model description: {:?}", updated.description);
    println!("Installed model description: {:?}", installed_model.model.description);
}

#[tokio::test]
async fn test_concurrent_operations_integration() {
    let service = Arc::new(setup_integration_test().await);

    // Concurrent model creation
    let mut create_handles = vec![];
    for i in 0..10 {
        let service_clone = service.clone();
        let handle = tokio::spawn(async move {
            let request = create_comprehensive_model_request(&format!("concurrent-{}", i));
            service_clone.create_model(request).await
        });
        create_handles.push(handle);
    }

    // Wait for all creations
    let mut created_models = vec![];
    for handle in create_handles {
        let result = handle.await.unwrap();
        if let Ok(model) = result {
            created_models.push(model);
        }
    }

    assert_eq!(created_models.len(), 10);

    // Concurrent installations
    let mut install_handles = vec![];
    for (i, model) in created_models.iter().enumerate() {
        let service_clone = service.clone();
        let model_id = model.id;
        let handle = tokio::spawn(async move {
            let path = format!("/opt/concurrent-{}", i);
            service_clone.install_model(model_id, path).await
        });
        install_handles.push(handle);
    }

    // Wait for installations
    let mut install_count = 0;
    for handle in install_handles {
        if handle.await.unwrap().is_ok() {
            install_count += 1;
        }
    }

    assert_eq!(install_count, 10);

    // Concurrent status updates
    let mut status_handles = vec![];
    for model in &created_models {
        let service_clone = service.clone();
        let model_id = model.id;
        let handle = tokio::spawn(async move {
            service_clone.update_model_status(model_id, ModelStatus::Running).await
        });
        status_handles.push(handle);
    }

    // Wait for status updates
    for handle in status_handles {
        handle.await.unwrap().unwrap();
    }

    // Verify final state
    let stats = service.get_model_stats().await.unwrap();
    assert_eq!(stats.total_models, 10);
    assert_eq!(stats.installed_count, 10);
    assert_eq!(stats.running_count, 10);
}

#[tokio::test]
async fn test_large_scale_operations() {
    let service = setup_integration_test().await;

    // Create a substantial number of models
    const MODEL_COUNT: usize = 50;
    let model_types = [
        ModelType::Chat,
        ModelType::Code,
        ModelType::Text,
        ModelType::Embedding,
        ModelType::Image,
    ];

    let start_time = std::time::Instant::now();

    for i in 0..MODEL_COUNT {
        let mut request = create_comprehensive_model_request(&format!("scale-test-{:03}", i));
        request.model_type = model_types[i % model_types.len()];
        request.file_size = 1_000_000_000 + (i as u64 * 100_000_000); // Varying sizes
        request.is_official = i % 5 == 0; // Every 5th model is official

        service.create_model(request).await.unwrap();
    }

    let creation_time = start_time.elapsed();
    println!("Created {} models in {:?}", MODEL_COUNT, creation_time);

    // Test bulk operations
    let all_models = service.list_models(ModelFilter::default()).await.unwrap();
    assert_eq!(all_models.len(), MODEL_COUNT);

    // Test filtering performance
    let filter_start = std::time::Instant::now();
    let chat_models = service.list_models(ModelFilter {
        model_type: Some(ModelType::Chat),
        ..Default::default()
    }).await.unwrap();
    let filter_time = filter_start.elapsed();

    println!("Filtered {} chat models in {:?}", chat_models.len(), filter_time);

    // Test search performance
    let search_start = std::time::Instant::now();
    let search_results = service.list_models(ModelFilter {
        search: Some("scale-test".to_string()),
        limit: Some(20),
        ..Default::default()
    }).await.unwrap();
    let search_time = search_start.elapsed();

    println!("Search found {} models in {:?}", search_results.len(), search_time);

    // Test statistics calculation performance
    let stats_start = std::time::Instant::now();
    let stats = service.get_model_stats().await.unwrap();
    let stats_time = stats_start.elapsed();

    println!("Calculated statistics in {:?}", stats_time);
    assert_eq!(stats.total_models, MODEL_COUNT);

    // Performance assertions (loose bounds for CI)
    assert!(creation_time.as_millis() < 10000); // 10 seconds
    assert!(filter_time.as_millis() < 1000);    // 1 second
    assert!(search_time.as_millis() < 1000);    // 1 second
    assert!(stats_time.as_millis() < 1000);     // 1 second
}

#[tokio::test]
async fn test_service_recovery_and_persistence() {
    let db = Arc::new(create_in_memory_database().await.unwrap());

    // Create service and add data
    let created_model_id = {
        let service = ModelsService::new(db.clone()).await.unwrap();
        let request = create_comprehensive_model_request("persistence-test");
        let created = service.create_model(request).await.unwrap();
        service.install_model(created.id, "/opt/persistence".to_string()).await.unwrap();
        service.update_model_status(created.id, ModelStatus::Running).await.unwrap();
        created.id
    };

    // Create new service instance with same database
    let service2 = ModelsService::new(db.clone()).await.unwrap();

    // Verify data persisted
    let models = service2.list_models(ModelFilter::default()).await.unwrap();
    assert_eq!(models.len(), 1);
    assert_eq!(models[0].name, "persistence-test");

    let installed = service2.get_installed_models().await.unwrap();
    assert_eq!(installed.len(), 1);
    assert_eq!(installed[0].status, ModelStatus::Running);

    let stats = service2.get_model_stats().await.unwrap();
    assert_eq!(stats.total_models, 1);
    assert_eq!(stats.installed_count, 1);
    assert_eq!(stats.running_count, 1);

    // Test operations on recovered data
    let update_request = UpdateModelRequest {
        description: Some("Updated after recovery".to_string()),
        ..Default::default()
    };
    let updated = service2.update_model(created_model_id, update_request).await.unwrap();
    assert_eq!(updated.description, Some("Updated after recovery".to_string()));
}