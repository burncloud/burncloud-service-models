use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// 模型类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ModelType {
    /// 对话模型
    Chat,
    /// 代码生成模型
    Code,
    /// 文本生成模型
    Text,
    /// 嵌入模型
    Embedding,
    /// 多模态模型
    Multimodal,
    /// 图像生成模型
    ImageGeneration,
    /// 语音模型
    Speech,
}

/// 模型大小分类
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelSize {
    /// 小型 (< 3B)
    Small,
    /// 中型 (3B - 8B)
    Medium,
    /// 大型 (8B - 30B)
    Large,
    /// 超大型 (> 30B)
    XLarge,
}

/// 模型运行状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelStatus {
    /// 运行中
    Running,
    /// 已停止
    Stopped,
    /// 启动中
    Starting,
    /// 停止中
    Stopping,
    /// 错误状态
    Error,
    /// 下载中
    Downloading,
    /// 安装中
    Installing,
}

/// 模型信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Model {
    /// 模型ID
    pub id: Uuid,
    /// 模型名称
    pub name: String,
    /// 模型显示名称
    pub display_name: String,
    /// 模型描述
    pub description: Option<String>,
    /// 模型版本
    pub version: String,
    /// 模型类型
    pub model_type: ModelType,
    /// 模型大小分类
    pub size_category: ModelSize,
    /// 模型文件大小 (字节)
    pub file_size: u64,
    /// 模型提供商
    pub provider: String,
    /// 模型许可证
    pub license: Option<String>,
    /// 模型标签
    pub tags: Vec<String>,
    /// 支持的语言
    pub languages: Vec<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
    /// 模型文件路径
    pub file_path: Option<String>,
    /// 模型检验和
    pub checksum: Option<String>,
    /// 下载URL
    pub download_url: Option<String>,
    /// 模型配置参数
    pub config: HashMap<String, serde_json::Value>,
    /// 模型评分
    pub rating: Option<f32>,
    /// 下载次数
    pub download_count: u64,
    /// 是否为官方模型
    pub is_official: bool,
}

/// 已安装的模型实例
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InstalledModel {
    /// 基础模型信息
    #[serde(flatten)]
    pub model: Model,
    /// 安装路径
    pub install_path: String,
    /// 安装时间
    pub installed_at: DateTime<Utc>,
    /// 当前状态
    pub status: ModelStatus,
    /// 运行端口
    pub port: Option<u16>,
    /// 进程ID
    pub process_id: Option<u32>,
    /// 最后使用时间
    pub last_used: Option<DateTime<Utc>>,
    /// 使用次数
    pub usage_count: u64,
}

/// 可下载的模型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AvailableModel {
    /// 基础模型信息
    #[serde(flatten)]
    pub model: Model,
    /// 是否已安装
    pub is_installed: bool,
    /// 发布时间
    pub published_at: DateTime<Utc>,
    /// 最后更新时间
    pub last_updated: DateTime<Utc>,
    /// 系统要求
    pub system_requirements: SystemRequirements,
}

/// 系统要求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemRequirements {
    /// 最小内存 (GB)
    pub min_memory_gb: f32,
    /// 推荐内存 (GB)
    pub recommended_memory_gb: f32,
    /// 最小磁盘空间 (GB)
    pub min_disk_space_gb: f32,
    /// 是否需要GPU
    pub requires_gpu: bool,
    /// 支持的操作系统
    pub supported_os: Vec<String>,
    /// 支持的架构
    pub supported_architectures: Vec<String>,
}

impl Model {
    /// 创建新模型
    pub fn new(
        name: String,
        display_name: String,
        version: String,
        model_type: ModelType,
        provider: String,
        file_size: u64,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            display_name,
            description: None,
            version,
            model_type,
            size_category: Self::calculate_size_category(file_size),
            file_size,
            provider,
            license: None,
            tags: Vec::new(),
            languages: Vec::new(),
            created_at: now,
            updated_at: now,
            file_path: None,
            checksum: None,
            download_url: None,
            config: HashMap::new(),
            rating: None,
            download_count: 0,
            is_official: false,
        }
    }

    /// 根据文件大小计算模型大小分类
    fn calculate_size_category(file_size: u64) -> ModelSize {
        let size_gb = file_size as f64 / 1024.0 / 1024.0 / 1024.0;
        match size_gb {
            s if s < 3.0 => ModelSize::Small,
            s if s < 8.0 => ModelSize::Medium,
            s if s < 30.0 => ModelSize::Large,
            _ => ModelSize::XLarge,
        }
    }

    /// 格式化文件大小
    pub fn formatted_size(&self) -> String {
        let size_gb = self.file_size as f64 / 1024.0 / 1024.0 / 1024.0;
        format!("{:.1}GB", size_gb)
    }
}

impl InstalledModel {
    /// 从模型创建已安装实例
    pub fn from_model(model: Model, install_path: String) -> Self {
        Self {
            model,
            install_path,
            installed_at: Utc::now(),
            status: ModelStatus::Stopped,
            port: None,
            process_id: None,
            last_used: None,
            usage_count: 0,
        }
    }

    /// 标记为已使用
    pub fn mark_used(&mut self) {
        self.last_used = Some(Utc::now());
        self.usage_count += 1;
    }

    /// 检查是否正在运行
    pub fn is_running(&self) -> bool {
        matches!(self.status, ModelStatus::Running)
    }
}

impl AvailableModel {
    /// 从模型创建可下载实例
    pub fn from_model(model: Model, system_requirements: SystemRequirements) -> Self {
        Self {
            is_installed: false,
            published_at: model.created_at,
            last_updated: model.updated_at,
            system_requirements,
            model,
        }
    }

    /// 检查系统兼容性
    pub fn is_compatible(&self, available_memory_gb: f32, os: &str, arch: &str) -> bool {
        available_memory_gb >= self.system_requirements.min_memory_gb
            && self.system_requirements.supported_os.iter().any(|supported_os| supported_os == os)
            && self.system_requirements.supported_architectures.iter().any(|supported_arch| supported_arch == arch)
    }
}