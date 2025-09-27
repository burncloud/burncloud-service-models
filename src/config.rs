use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// 全局配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    /// 配置版本
    pub version: String,
    /// 服务配置
    pub service: ServiceConfig,
    /// 存储配置
    pub storage: StorageConfig,
    /// 网络配置
    pub network: NetworkConfig,
    /// 安全配置
    pub security: SecurityConfig,
    /// 监控配置
    pub monitoring: MonitoringConfig,
    /// 日志配置
    pub logging: LoggingConfig,
    /// 性能配置
    pub performance: PerformanceConfig,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

/// 服务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    /// 服务名称
    pub name: String,
    /// 服务版本
    pub version: String,
    /// 绑定地址
    pub bind_address: String,
    /// 绑定端口
    pub bind_port: u16,
    /// 工作目录
    pub work_dir: String,
    /// 模型存储目录
    pub models_dir: String,
    /// 日志目录
    pub logs_dir: String,
    /// 临时目录
    pub temp_dir: String,
    /// 最大并发连接数
    pub max_connections: u32,
    /// 请求超时时间 (秒)
    pub request_timeout_seconds: u32,
    /// 是否启用热重载
    pub enable_hot_reload: bool,
}

/// 存储配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// 存储类型
    pub storage_type: StorageType,
    /// 数据库URL
    pub database_url: Option<String>,
    /// 连接池大小
    pub connection_pool_size: u32,
    /// 连接超时 (秒)
    pub connection_timeout_seconds: u32,
    /// 自动备份
    pub auto_backup: bool,
    /// 备份间隔 (小时)
    pub backup_interval_hours: u32,
    /// 备份保留天数
    pub backup_retention_days: u32,
    /// 缓存配置
    pub cache: CacheConfig,
}

/// 存储类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StorageType {
    /// SQLite
    SQLite,
    /// PostgreSQL
    PostgreSQL,
    /// MySQL
    MySQL,
    /// 内存存储
    Memory,
}

/// 缓存配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// 是否启用缓存
    pub enabled: bool,
    /// 缓存类型
    pub cache_type: CacheType,
    /// 最大缓存大小 (MB)
    pub max_size_mb: u64,
    /// 缓存过期时间 (秒)
    pub ttl_seconds: u32,
    /// Redis URL (如果使用Redis)
    pub redis_url: Option<String>,
}

/// 缓存类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CacheType {
    /// 内存缓存
    Memory,
    /// Redis
    Redis,
    /// 文件缓存
    File,
}

/// 网络配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// 是否启用HTTPS
    pub enable_https: bool,
    /// SSL证书路径
    pub ssl_cert_path: Option<String>,
    /// SSL私钥路径
    pub ssl_key_path: Option<String>,
    /// 是否启用CORS
    pub enable_cors: bool,
    /// 允许的源
    pub allowed_origins: Vec<String>,
    /// 代理配置
    pub proxy: Option<ProxyConfig>,
    /// 下载配置
    pub download: DownloadConfig,
}

/// 代理配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    /// 代理URL
    pub url: String,
    /// 用户名
    pub username: Option<String>,
    /// 密码
    pub password: Option<String>,
    /// 不使用代理的域名
    pub no_proxy: Vec<String>,
}

/// 下载配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadConfig {
    /// 最大并发下载数
    pub max_concurrent_downloads: u32,
    /// 分块大小 (KB)
    pub chunk_size_kb: u32,
    /// 重试次数
    pub retry_attempts: u32,
    /// 重试间隔 (秒)
    pub retry_delay_seconds: u32,
    /// 下载超时 (秒)
    pub timeout_seconds: u32,
    /// 是否启用断点续传
    pub enable_resume: bool,
}

/// 安全配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// 是否启用认证
    pub enable_auth: bool,
    /// JWT密钥
    pub jwt_secret: Option<String>,
    /// Token过期时间 (小时)
    pub token_expiry_hours: u32,
    /// 是否启用API密钥
    pub enable_api_key: bool,
    /// API密钥
    pub api_keys: Vec<String>,
    /// 是否启用速率限制
    pub enable_rate_limiting: bool,
    /// 速率限制配置
    pub rate_limit: RateLimitConfig,
    /// 是否启用IP白名单
    pub enable_ip_whitelist: bool,
    /// IP白名单
    pub ip_whitelist: Vec<String>,
}

/// 速率限制配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// 每分钟请求数
    pub requests_per_minute: u32,
    /// 每小时请求数
    pub requests_per_hour: u32,
    /// 每天请求数
    pub requests_per_day: u32,
    /// 突发请求数
    pub burst_size: u32,
}

/// 监控配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// 是否启用监控
    pub enabled: bool,
    /// 监控端口
    pub port: u16,
    /// 指标收集间隔 (秒)
    pub metrics_interval_seconds: u32,
    /// 是否启用Prometheus
    pub enable_prometheus: bool,
    /// Prometheus端点
    pub prometheus_endpoint: String,
    /// 是否启用健康检查
    pub enable_health_check: bool,
    /// 健康检查间隔 (秒)
    pub health_check_interval_seconds: u32,
    /// 告警配置
    pub alerts: AlertConfig,
}

/// 告警配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    /// 是否启用告警
    pub enabled: bool,
    /// CPU使用率阈值
    pub cpu_threshold_percent: f32,
    /// 内存使用率阈值
    pub memory_threshold_percent: f32,
    /// 磁盘使用率阈值
    pub disk_threshold_percent: f32,
    /// 错误率阈值
    pub error_rate_threshold_percent: f32,
    /// 通知方式
    pub notification_methods: Vec<NotificationMethod>,
}

/// 通知方式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationMethod {
    /// 邮件
    Email { to: String, smtp_server: String },
    /// Webhook
    Webhook { url: String },
    /// Slack
    Slack { webhook_url: String },
    /// 钉钉
    DingTalk { webhook_url: String },
}

/// 日志配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// 日志级别
    pub level: LogLevel,
    /// 日志格式
    pub format: LogFormat,
    /// 是否输出到控制台
    pub console: bool,
    /// 是否输出到文件
    pub file: bool,
    /// 日志文件路径
    pub file_path: Option<String>,
    /// 日志文件最大大小 (MB)
    pub max_file_size_mb: u32,
    /// 日志文件保留数量
    pub max_files: u32,
    /// 是否压缩旧日志
    pub compress: bool,
    /// 结构化日志字段
    pub structured_fields: HashMap<String, serde_json::Value>,
}

/// 日志级别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LogLevel {
    /// 跟踪
    Trace,
    /// 调试
    Debug,
    /// 信息
    Info,
    /// 警告
    Warn,
    /// 错误
    Error,
}

/// 日志格式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LogFormat {
    /// 纯文本
    Text,
    /// JSON
    Json,
    /// 结构化
    Structured,
}

/// 性能配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// 工作线程数
    pub worker_threads: Option<u32>,
    /// 是否启用线程池
    pub enable_thread_pool: bool,
    /// 线程池大小
    pub thread_pool_size: u32,
    /// 内存池大小 (MB)
    pub memory_pool_size_mb: u64,
    /// 是否启用预加载
    pub enable_preloading: bool,
    /// 预加载模型列表
    pub preload_models: Vec<String>,
    /// 垃圾回收配置
    pub gc: GcConfig,
}

/// 垃圾回收配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcConfig {
    /// 是否启用自动GC
    pub auto_gc: bool,
    /// GC间隔 (秒)
    pub gc_interval_seconds: u32,
    /// 内存阈值 (MB)
    pub memory_threshold_mb: u64,
    /// 是否强制GC
    pub force_gc: bool,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            version: "1.0.0".to_string(),
            service: ServiceConfig::default(),
            storage: StorageConfig::default(),
            network: NetworkConfig::default(),
            security: SecurityConfig::default(),
            monitoring: MonitoringConfig::default(),
            logging: LoggingConfig::default(),
            performance: PerformanceConfig::default(),
            created_at: now,
            updated_at: now,
        }
    }
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            name: "burncloud-service".to_string(),
            version: "0.1.0".to_string(),
            bind_address: "127.0.0.1".to_string(),
            bind_port: 8080,
            work_dir: "./data".to_string(),
            models_dir: "./data/models".to_string(),
            logs_dir: "./logs".to_string(),
            temp_dir: "./tmp".to_string(),
            max_connections: 1000,
            request_timeout_seconds: 30,
            enable_hot_reload: false,
        }
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            storage_type: StorageType::SQLite,
            database_url: Some("sqlite:./data/burncloud.db".to_string()),
            connection_pool_size: 10,
            connection_timeout_seconds: 30,
            auto_backup: true,
            backup_interval_hours: 24,
            backup_retention_days: 7,
            cache: CacheConfig::default(),
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cache_type: CacheType::Memory,
            max_size_mb: 256,
            ttl_seconds: 3600,
            redis_url: None,
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            enable_https: false,
            ssl_cert_path: None,
            ssl_key_path: None,
            enable_cors: true,
            allowed_origins: vec!["*".to_string()],
            proxy: None,
            download: DownloadConfig::default(),
        }
    }
}

impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            max_concurrent_downloads: 3,
            chunk_size_kb: 1024,
            retry_attempts: 3,
            retry_delay_seconds: 5,
            timeout_seconds: 300,
            enable_resume: true,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_auth: false,
            jwt_secret: None,
            token_expiry_hours: 24,
            enable_api_key: false,
            api_keys: Vec::new(),
            enable_rate_limiting: true,
            rate_limit: RateLimitConfig::default(),
            enable_ip_whitelist: false,
            ip_whitelist: Vec::new(),
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            requests_per_hour: 1000,
            requests_per_day: 10000,
            burst_size: 10,
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            port: 9090,
            metrics_interval_seconds: 30,
            enable_prometheus: true,
            prometheus_endpoint: "/metrics".to_string(),
            enable_health_check: true,
            health_check_interval_seconds: 30,
            alerts: AlertConfig::default(),
        }
    }
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cpu_threshold_percent: 80.0,
            memory_threshold_percent: 85.0,
            disk_threshold_percent: 90.0,
            error_rate_threshold_percent: 5.0,
            notification_methods: Vec::new(),
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            format: LogFormat::Text,
            console: true,
            file: true,
            file_path: Some("./logs/burncloud.log".to_string()),
            max_file_size_mb: 100,
            max_files: 10,
            compress: true,
            structured_fields: HashMap::new(),
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            worker_threads: None, // 使用系统默认
            enable_thread_pool: true,
            thread_pool_size: num_cpus::get() as u32,
            memory_pool_size_mb: 512,
            enable_preloading: false,
            preload_models: Vec::new(),
            gc: GcConfig::default(),
        }
    }
}

impl Default for GcConfig {
    fn default() -> Self {
        Self {
            auto_gc: true,
            gc_interval_seconds: 300, // 5分钟
            memory_threshold_mb: 1024, // 1GB
            force_gc: false,
        }
    }
}

// 为了避免依赖 num_cpus，我们提供一个简单的实现
mod num_cpus {
    pub fn get() -> usize {
        std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4)
    }
}