use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// 模型运行时配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// 最大上下文长度
    pub max_context_length: Option<u32>,
    /// 温度参数
    pub temperature: Option<f32>,
    /// Top-p 参数
    pub top_p: Option<f32>,
    /// Top-k 参数
    pub top_k: Option<u32>,
    /// 最大生成长度
    pub max_tokens: Option<u32>,
    /// 停止词
    pub stop_sequences: Vec<String>,
    /// 批处理大小
    pub batch_size: Option<u32>,
    /// 并发请求数
    pub max_concurrent_requests: Option<u32>,
    /// GPU 设备ID
    pub gpu_device_ids: Vec<u32>,
    /// 内存限制 (MB)
    pub memory_limit_mb: Option<u64>,
    /// 是否启用流式输出
    pub enable_streaming: bool,
    /// 自定义参数
    pub custom_params: HashMap<String, serde_json::Value>,
}

/// 模型运行时实例
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRuntime {
    /// 运行时ID
    pub id: Uuid,
    /// 模型ID
    pub model_id: Uuid,
    /// 运行时名称
    pub name: String,
    /// 绑定端口
    pub port: u16,
    /// 进程ID
    pub process_id: Option<u32>,
    /// 运行时配置
    pub config: RuntimeConfig,
    /// 启动时间
    pub started_at: Option<DateTime<Utc>>,
    /// 停止时间
    pub stopped_at: Option<DateTime<Utc>>,
    /// 运行状态
    pub status: crate::ModelStatus,
    /// 健康检查端点
    pub health_endpoint: String,
    /// API 端点
    pub api_endpoint: String,
    /// 日志文件路径
    pub log_file: Option<String>,
    /// 环境变量
    pub environment: HashMap<String, String>,
}

/// 运行时性能指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeMetrics {
    /// 运行时ID
    pub runtime_id: Uuid,
    /// 采样时间
    pub timestamp: DateTime<Utc>,
    /// CPU 使用率 (百分比)
    pub cpu_usage_percent: f32,
    /// 内存使用量 (MB)
    pub memory_usage_mb: u64,
    /// GPU 使用率 (百分比)
    pub gpu_usage_percent: Option<f32>,
    /// GPU 内存使用量 (MB)
    pub gpu_memory_usage_mb: Option<u64>,
    /// 活跃连接数
    pub active_connections: u32,
    /// 总请求数
    pub total_requests: u64,
    /// 成功请求数
    pub successful_requests: u64,
    /// 失败请求数
    pub failed_requests: u64,
    /// 平均响应时间 (毫秒)
    pub avg_response_time_ms: f32,
    /// 吞吐量 (请求/秒)
    pub throughput_rps: f32,
    /// 队列长度
    pub queue_length: u32,
}

/// 运行时事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeEvent {
    /// 事件ID
    pub id: Uuid,
    /// 运行时ID
    pub runtime_id: Uuid,
    /// 事件类型
    pub event_type: RuntimeEventType,
    /// 事件时间
    pub timestamp: DateTime<Utc>,
    /// 事件描述
    pub message: String,
    /// 事件详情
    pub details: Option<serde_json::Value>,
    /// 严重程度
    pub severity: EventSeverity,
}

/// 运行时事件类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RuntimeEventType {
    /// 启动
    Started,
    /// 停止
    Stopped,
    /// 重启
    Restarted,
    /// 配置更新
    ConfigUpdated,
    /// 健康检查失败
    HealthCheckFailed,
    /// 内存警告
    MemoryWarning,
    /// 错误
    Error,
    /// 请求处理
    RequestProcessed,
    /// 性能警告
    PerformanceWarning,
}

/// 事件严重程度
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventSeverity {
    /// 信息
    Info,
    /// 警告
    Warning,
    /// 错误
    Error,
    /// 严重错误
    Critical,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            max_context_length: Some(4096),
            temperature: Some(0.7),
            top_p: Some(0.9),
            top_k: Some(50),
            max_tokens: Some(2048),
            stop_sequences: vec!["</s>".to_string(), "<|endoftext|>".to_string()],
            batch_size: Some(1),
            max_concurrent_requests: Some(10),
            gpu_device_ids: vec![0],
            memory_limit_mb: None,
            enable_streaming: true,
            custom_params: HashMap::new(),
        }
    }
}

impl ModelRuntime {
    /// 创建新的运行时实例
    pub fn new(model_id: Uuid, name: String, port: u16) -> Self {
        Self {
            id: Uuid::new_v4(),
            model_id,
            name,
            port,
            process_id: None,
            config: RuntimeConfig::default(),
            started_at: None,
            stopped_at: None,
            status: crate::ModelStatus::Stopped,
            health_endpoint: format!("http://localhost:{}/health", port),
            api_endpoint: format!("http://localhost:{}/v1", port),
            log_file: None,
            environment: HashMap::new(),
        }
    }

    /// 标记为启动
    pub fn mark_started(&mut self, process_id: u32) {
        self.process_id = Some(process_id);
        self.started_at = Some(Utc::now());
        self.status = crate::ModelStatus::Running;
        self.stopped_at = None;
    }

    /// 标记为停止
    pub fn mark_stopped(&mut self) {
        self.process_id = None;
        self.stopped_at = Some(Utc::now());
        self.status = crate::ModelStatus::Stopped;
    }

    /// 获取运行时长 (秒)
    pub fn uptime_seconds(&self) -> Option<i64> {
        self.started_at.map(|start| {
            if self.status == crate::ModelStatus::Running {
                Utc::now().timestamp() - start.timestamp()
            } else {
                self.stopped_at.unwrap_or(Utc::now()).timestamp() - start.timestamp()
            }
        })
    }

    /// 检查是否健康
    pub fn is_healthy(&self) -> bool {
        self.status == crate::ModelStatus::Running && self.process_id.is_some()
    }
}

impl RuntimeEvent {
    /// 创建新事件
    pub fn new(
        runtime_id: Uuid,
        event_type: RuntimeEventType,
        message: String,
        severity: EventSeverity,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            runtime_id,
            event_type,
            timestamp: Utc::now(),
            message,
            details: None,
            severity,
        }
    }

    /// 带详情创建事件
    pub fn with_details(
        runtime_id: Uuid,
        event_type: RuntimeEventType,
        message: String,
        severity: EventSeverity,
        details: serde_json::Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            runtime_id,
            event_type,
            timestamp: Utc::now(),
            message,
            details: Some(details),
            severity,
        }
    }
}