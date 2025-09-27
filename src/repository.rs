use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use crate::Model;

/// 模型仓库信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRepository {
    /// 仓库ID
    pub id: Uuid,
    /// 仓库名称
    pub name: String,
    /// 仓库URL
    pub url: String,
    /// 仓库类型
    pub repo_type: RepositoryType,
    /// 是否启用
    pub enabled: bool,
    /// 认证信息
    pub auth: Option<RepositoryAuth>,
    /// 最后同步时间
    pub last_sync: Option<DateTime<Utc>>,
    /// 同步状态
    pub sync_status: SyncStatus,
    /// 仓库描述
    pub description: Option<String>,
    /// 仓库标签
    pub tags: Vec<String>,
    /// 优先级 (数字越小优先级越高)
    pub priority: u32,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

/// 仓库类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RepositoryType {
    /// 官方仓库
    Official,
    /// Hugging Face Hub
    HuggingFace,
    /// 本地文件系统
    Local,
    /// Git 仓库
    Git,
    /// HTTP/HTTPS 仓库
    Http,
    /// 第三方仓库
    ThirdParty,
}

/// 仓库认证信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryAuth {
    /// 认证类型
    pub auth_type: AuthType,
    /// 用户名
    pub username: Option<String>,
    /// 密码/令牌
    pub token: Option<String>,
    /// API 密钥
    pub api_key: Option<String>,
    /// 其他认证参数
    pub extra_params: HashMap<String, String>,
}

/// 认证类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthType {
    /// 无认证
    None,
    /// 基本认证
    Basic,
    /// 令牌认证
    Token,
    /// API 密钥
    ApiKey,
    /// OAuth
    OAuth,
}

/// 同步状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncStatus {
    /// 从未同步
    Never,
    /// 同步中
    Syncing,
    /// 同步成功
    Success,
    /// 同步失败
    Failed,
    /// 同步过期
    Outdated,
}

/// 模型仓库索引
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryIndex {
    /// 仓库ID
    pub repository_id: Uuid,
    /// 索引版本
    pub version: String,
    /// 索引更新时间
    pub updated_at: DateTime<Utc>,
    /// 模型列表
    pub models: Vec<RepositoryModel>,
    /// 索引校验和
    pub checksum: Option<String>,
    /// 索引元数据
    pub metadata: HashMap<String, serde_json::Value>,
}

/// 仓库中的模型信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryModel {
    /// 仓库中的模型ID
    pub repo_model_id: String,
    /// 模型信息
    #[serde(flatten)]
    pub model: Model,
    /// 仓库特定信息
    pub repository_info: RepositoryModelInfo,
}

/// 仓库特定的模型信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryModelInfo {
    /// 在仓库中的路径
    pub repo_path: String,
    /// 下载链接
    pub download_urls: Vec<DownloadUrl>,
    /// 文件列表
    pub files: Vec<ModelFile>,
    /// 依赖项
    pub dependencies: Vec<String>,
    /// 安装说明
    pub installation_notes: Option<String>,
    /// 使用示例
    pub usage_examples: Vec<String>,
    /// 许可证文本
    pub license_text: Option<String>,
    /// 模型卡片信息
    pub model_card: Option<String>,
}

/// 下载链接
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadUrl {
    /// 文件名
    pub filename: String,
    /// 下载URL
    pub url: String,
    /// 文件大小
    pub size: u64,
    /// 校验和
    pub checksum: Option<String>,
    /// 校验和算法
    pub checksum_algorithm: Option<String>,
    /// 是否为主文件
    pub is_primary: bool,
}

/// 模型文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelFile {
    /// 文件名
    pub filename: String,
    /// 文件大小
    pub size: u64,
    /// 文件类型
    pub file_type: ModelFileType,
    /// 校验和
    pub checksum: Option<String>,
    /// 是否必需
    pub required: bool,
    /// 文件描述
    pub description: Option<String>,
}

/// 模型文件类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelFileType {
    /// 模型权重
    Weights,
    /// 配置文件
    Config,
    /// 词汇表
    Tokenizer,
    /// 文档
    Documentation,
    /// 示例
    Example,
    /// 其他
    Other,
}

/// 同步结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    /// 仓库ID
    pub repository_id: Uuid,
    /// 同步开始时间
    pub started_at: DateTime<Utc>,
    /// 同步结束时间
    pub completed_at: Option<DateTime<Utc>>,
    /// 同步状态
    pub status: SyncStatus,
    /// 新增模型数量
    pub models_added: u32,
    /// 更新模型数量
    pub models_updated: u32,
    /// 删除模型数量
    pub models_removed: u32,
    /// 错误信息
    pub error_message: Option<String>,
    /// 同步日志
    pub log_entries: Vec<String>,
}

impl ModelRepository {
    /// 创建新的模型仓库
    pub fn new(name: String, url: String, repo_type: RepositoryType) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            url,
            repo_type,
            enabled: true,
            auth: None,
            last_sync: None,
            sync_status: SyncStatus::Never,
            description: None,
            tags: Vec::new(),
            priority: 100,
            created_at: now,
            updated_at: now,
        }
    }

    /// 设置认证信息
    pub fn with_auth(mut self, auth: RepositoryAuth) -> Self {
        self.auth = Some(auth);
        self
    }

    /// 设置描述
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置优先级
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    /// 检查是否需要同步
    pub fn needs_sync(&self, max_age_hours: u32) -> bool {
        match self.last_sync {
            None => true,
            Some(last_sync) => {
                let age = Utc::now().signed_duration_since(last_sync);
                age.num_hours() as u32 > max_age_hours
            }
        }
    }

    /// 标记同步开始
    pub fn mark_sync_started(&mut self) {
        self.sync_status = SyncStatus::Syncing;
        self.updated_at = Utc::now();
    }

    /// 标记同步完成
    pub fn mark_sync_completed(&mut self, success: bool) {
        self.sync_status = if success { SyncStatus::Success } else { SyncStatus::Failed };
        self.last_sync = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// 是否为官方仓库
    pub fn is_official(&self) -> bool {
        self.repo_type == RepositoryType::Official
    }
}

impl RepositoryAuth {
    /// 创建基本认证
    pub fn basic(username: String, password: String) -> Self {
        Self {
            auth_type: AuthType::Basic,
            username: Some(username),
            token: Some(password),
            api_key: None,
            extra_params: HashMap::new(),
        }
    }

    /// 创建令牌认证
    pub fn token(token: String) -> Self {
        Self {
            auth_type: AuthType::Token,
            username: None,
            token: Some(token),
            api_key: None,
            extra_params: HashMap::new(),
        }
    }

    /// 创建API密钥认证
    pub fn api_key(api_key: String) -> Self {
        Self {
            auth_type: AuthType::ApiKey,
            username: None,
            token: None,
            api_key: Some(api_key),
            extra_params: HashMap::new(),
        }
    }
}

impl SyncResult {
    /// 创建新的同步结果
    pub fn new(repository_id: Uuid) -> Self {
        Self {
            repository_id,
            started_at: Utc::now(),
            completed_at: None,
            status: SyncStatus::Syncing,
            models_added: 0,
            models_updated: 0,
            models_removed: 0,
            error_message: None,
            log_entries: Vec::new(),
        }
    }

    /// 标记完成
    pub fn mark_completed(&mut self, success: bool) {
        self.completed_at = Some(Utc::now());
        self.status = if success { SyncStatus::Success } else { SyncStatus::Failed };
    }

    /// 添加日志条目
    pub fn add_log(&mut self, message: String) {
        self.log_entries.push(format!("[{}] {}", Utc::now().format("%H:%M:%S"), message));
    }

    /// 设置错误
    pub fn set_error(&mut self, error: String) {
        self.error_message = Some(error.clone());
        self.add_log(format!("ERROR: {}", error));
        self.mark_completed(false);
    }
}