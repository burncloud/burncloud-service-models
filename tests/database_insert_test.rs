//! 数据库插入测试
//!
//! 这个测试文件专门用于测试往数据库中插入数据的功能

use burncloud_service_models::{
    ModelsService, CreateModelRequest, ModelType
};
use burncloud_database::Database;
use std::sync::Arc;
use std::collections::HashMap;

/// 创建一个简单的模型请求用于测试
fn create_simple_model_request() -> CreateModelRequest {
    CreateModelRequest {
        name: "test-model".to_string(),
        display_name: "Test Model".to_string(),
        version: "1.0.0".to_string(),
        model_type: ModelType::Chat,
        provider: "TestProvider".to_string(),
        file_size: 5_000_000_000, // 5GB
        description: Some("This is a test model for database insertion".to_string()),
        license: Some("MIT".to_string()),
        tags: vec!["test".to_string(), "demo".to_string()],
        languages: vec!["English".to_string(), "Chinese".to_string()],
        file_path: None,
        download_url: Some("https://example.com/test-model".to_string()),
        config: HashMap::new(),
        is_official: false,
    }
}

#[tokio::test]
async fn test_insert_model_into_database() {
    // 1. 创建内存数据库
    let mut database = Database::new_in_memory();
    database.initialize().await.expect("Failed to initialize database");
    let database = Arc::new(database);

    // 2. 创建 ModelsService
    let service = ModelsService::new(database.clone())
        .await
        .expect("Failed to create ModelsService");

    // 3. 创建模型请求
    let request = create_simple_model_request();

    // 4. 插入数据到数据库
    let created_model = service.create_model(request)
        .await
        .expect("Failed to insert model into database");

    // 5. 验证插入的数据
    println!("✅ Model inserted successfully!");
    println!("   ID: {}", created_model.id);
    println!("   Name: {}", created_model.name);
    println!("   Display Name: {}", created_model.display_name);
    println!("   Type: {:?}", created_model.model_type);
    println!("   Provider: {}", created_model.provider);
    println!("   File Size: {} bytes", created_model.file_size);

    // 断言验证
    assert_eq!(created_model.name, "test-model");
    assert_eq!(created_model.display_name, "Test Model");
    assert_eq!(created_model.model_type, ModelType::Chat);
    assert_eq!(created_model.provider, "TestProvider");
    assert_eq!(created_model.file_size, 5_000_000_000);
    assert!(!created_model.is_official);

    // 6. 验证数据确实存在于数据库中
    let retrieved_model = service.get_model(created_model.id)
        .await
        .expect("Failed to retrieve model")
        .expect("Model not found in database");

    assert_eq!(retrieved_model.id, created_model.id);
    assert_eq!(retrieved_model.name, created_model.name);

    println!("✅ Model retrieved successfully from database!");
}

#[tokio::test]
async fn test_insert_model_into_file_database() {
    // 使用 burncloud-database 提供的默认数据库路径
    // Windows: %USERPROFILE%\AppData\Local\BurnCloud\data.db
    // Linux: ~/.burncloud/data.db

    // 注意: 此测试会使用真实的数据库文件
    // 为了避免与已有数据冲突，我们使用唯一的模型名称

    {
        // 1. 创建默认位置的数据库
        let database = Database::new_default_initialized()
            .await
            .expect("Failed to create default database");
        let database = Arc::new(database);

        println!("✅ Database initialized at default location");

        // 2. 创建 ModelsService
        let service = ModelsService::new(database.clone())
            .await
            .expect("Failed to create ModelsService");

        // 3. 使用带时间戳的唯一模型名称，避免冲突
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut request1 = create_simple_model_request();
        request1.name = format!("qwen-7b-test-{}", timestamp);
        request1.display_name = "Qwen 7B Chat Test".to_string();
        request1.model_type = ModelType::Chat;

        let model1 = service.create_model(request1)
            .await
            .expect("Failed to insert first model");

        println!("✅ Inserted model 1: {} (ID: {})", model1.name, model1.id);

        // 4. 插入第二个模型
        let mut request2 = create_simple_model_request();
        request2.name = format!("deepseek-coder-test-{}", timestamp);
        request2.display_name = "DeepSeek Coder Test".to_string();
        request2.model_type = ModelType::Code;

        let model2 = service.create_model(request2)
            .await
            .expect("Failed to insert second model");

        println!("✅ Inserted model 2: {} (ID: {})", model2.name, model2.id);

        // 5. 安装第一个模型
        let install_path = "/opt/models/qwen-7b".to_string();
        let installed = service.install_model(model1.id, install_path.clone())
            .await
            .expect("Failed to install model");

        println!("✅ Installed model at: {}", installed.install_path);

        // 6. 列出此测试创建的模型（通过 ID）
        let created_models = vec![model1.id, model2.id];
        println!("✅ Created {} test models in this test run", created_models.len());

        // 验证可以查询到创建的模型
        for model_id in &created_models {
            let found = service.get_model(*model_id).await.expect("Failed to query model");
            assert!(found.is_some(), "Model should exist in database");
        }

        let installed_models = service.get_installed_models()
            .await
            .expect("Failed to get installed models");

        // 过滤出本测试安装的模型
        let test_installed: Vec<_> = installed_models.iter()
            .filter(|im| im.model.id == model1.id)
            .collect();

        println!("✅ Verified installed model from this test");
        assert_eq!(test_installed.len(), 1);
    }

    // 7. 打印数据库位置信息
    println!("\n🎉 Test completed!");
    println!("💡 Database is stored at the default location managed by burncloud-database");
    println!("   Windows: %USERPROFILE%\\AppData\\Local\\BurnCloud\\data.db");
    println!("   Linux: ~/.burncloud/data.db");
}

#[tokio::test]
async fn test_insert_multiple_models() {
    // 创建数据库和服务
    let mut database = Database::new_in_memory();
    database.initialize().await.expect("Failed to initialize database");
    let database = Arc::new(database);
    let service = ModelsService::new(database).await.expect("Failed to create service");

    // 插入多个模型
    let model_names = vec!["model-1", "model-2", "model-3"];
    let mut inserted_ids = Vec::new();

    for name in &model_names {
        let mut request = create_simple_model_request();
        request.name = name.to_string();
        request.display_name = format!("{} Display", name);

        let created = service.create_model(request)
            .await
            .expect(&format!("Failed to insert {}", name));

        inserted_ids.push(created.id);
        println!("✅ Inserted model: {}", name);
    }

    // 验证所有模型都在数据库中
    let all_models = service.list_models(Default::default())
        .await
        .expect("Failed to list models");

    assert_eq!(all_models.len(), 3, "Should have 3 models in database");

    // 验证所有模型名称都存在（不依赖顺序）
    let retrieved_names: Vec<String> = all_models.iter().map(|m| m.name.clone()).collect();
    for name in &model_names {
        assert!(retrieved_names.contains(&name.to_string()), "Model {} should be in database", name);
        println!("✅ Verified model in database: {}", name);
    }

    println!("✅ All {} models inserted and verified successfully!", model_names.len());
}

#[tokio::test]
async fn test_insert_and_install_model() {
    // 创建数据库和服务
    let mut database = Database::new_in_memory();
    database.initialize().await.expect("Failed to initialize database");
    let database = Arc::new(database);
    let service = ModelsService::new(database).await.expect("Failed to create service");

    // 1. 插入模型
    let request = create_simple_model_request();
    let created_model = service.create_model(request)
        .await
        .expect("Failed to insert model");

    println!("✅ Model created: {}", created_model.name);

    // 2. 安装模型
    let install_path = "/opt/models/test-model".to_string();
    let installed_model = service.install_model(created_model.id, install_path.clone())
        .await
        .expect("Failed to install model");

    println!("✅ Model installed at: {}", installed_model.install_path);

    // 3. 验证安装
    assert_eq!(installed_model.model.id, created_model.id);
    assert_eq!(installed_model.install_path, install_path);
    assert_eq!(installed_model.status, burncloud_service_models::ModelStatus::Stopped);

    // 4. 获取已安装模型列表
    let installed_models = service.get_installed_models()
        .await
        .expect("Failed to get installed models");

    assert_eq!(installed_models.len(), 1);
    assert_eq!(installed_models[0].model.name, "test-model");

    println!("✅ Model installation verified successfully!");
}