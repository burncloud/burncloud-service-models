use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// 系统性能指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// 采样时间
    pub timestamp: DateTime<Utc>,
    /// CPU 指标
    pub cpu: CpuMetrics,
    /// 内存指标
    pub memory: MemoryMetrics,
    /// 磁盘指标
    pub disk: DiskMetrics,
    /// 网络指标
    pub network: NetworkMetrics,
    /// GPU 指标
    pub gpu: Option<GpuMetrics>,
    /// 系统负载
    pub load_average: LoadAverage,
}

/// CPU 指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuMetrics {
    /// 总体使用率 (百分比)
    pub usage_percent: f32,
    /// 用户态使用率
    pub user_percent: f32,
    /// 系统态使用率
    pub system_percent: f32,
    /// 空闲率
    pub idle_percent: f32,
    /// 等待IO率
    pub iowait_percent: f32,
    /// 核心数
    pub core_count: u32,
    /// 每个核心的使用率
    pub per_core_usage: Vec<f32>,
    /// CPU 频率 (MHz)
    pub frequency_mhz: u32,
    /// CPU 温度 (摄氏度)
    pub temperature_celsius: Option<f32>,
}

/// 内存指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    /// 总内存 (字节)
    pub total_bytes: u64,
    /// 已使用内存 (字节)
    pub used_bytes: u64,
    /// 可用内存 (字节)
    pub available_bytes: u64,
    /// 使用率 (百分比)
    pub usage_percent: f32,
    /// 缓存大小 (字节)
    pub cached_bytes: u64,
    /// 缓冲区大小 (字节)
    pub buffer_bytes: u64,
    /// 交换分区总大小 (字节)
    pub swap_total_bytes: u64,
    /// 交换分区已使用 (字节)
    pub swap_used_bytes: u64,
    /// 交换分区使用率 (百分比)
    pub swap_usage_percent: f32,
}

/// 磁盘指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskMetrics {
    /// 各个磁盘分区的指标
    pub partitions: Vec<DiskPartitionMetrics>,
    /// 磁盘IO指标
    pub io: DiskIoMetrics,
}

/// 磁盘分区指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskPartitionMetrics {
    /// 挂载点
    pub mount_point: String,
    /// 设备名
    pub device: String,
    /// 文件系统类型
    pub filesystem: String,
    /// 总空间 (字节)
    pub total_bytes: u64,
    /// 已使用空间 (字节)
    pub used_bytes: u64,
    /// 可用空间 (字节)
    pub available_bytes: u64,
    /// 使用率 (百分比)
    pub usage_percent: f32,
    /// Inode 总数
    pub total_inodes: u64,
    /// 已使用 Inode
    pub used_inodes: u64,
    /// Inode 使用率 (百分比)
    pub inode_usage_percent: f32,
}

/// 磁盘IO指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskIoMetrics {
    /// 读取字节数/秒
    pub read_bytes_per_sec: u64,
    /// 写入字节数/秒
    pub write_bytes_per_sec: u64,
    /// 读取次数/秒
    pub read_ops_per_sec: u64,
    /// 写入次数/秒
    pub write_ops_per_sec: u64,
    /// 平均读取延迟 (毫秒)
    pub avg_read_latency_ms: f32,
    /// 平均写入延迟 (毫秒)
    pub avg_write_latency_ms: f32,
    /// 磁盘利用率 (百分比)
    pub utilization_percent: f32,
}

/// 网络指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    /// 各个网络接口的指标
    pub interfaces: Vec<NetworkInterfaceMetrics>,
    /// 总体统计
    pub total: NetworkTotalMetrics,
}

/// 网络接口指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceMetrics {
    /// 接口名称
    pub interface: String,
    /// 接收字节数/秒
    pub rx_bytes_per_sec: u64,
    /// 发送字节数/秒
    pub tx_bytes_per_sec: u64,
    /// 接收包数/秒
    pub rx_packets_per_sec: u64,
    /// 发送包数/秒
    pub tx_packets_per_sec: u64,
    /// 接收错误数
    pub rx_errors: u64,
    /// 发送错误数
    pub tx_errors: u64,
    /// 接收丢包数
    pub rx_dropped: u64,
    /// 发送丢包数
    pub tx_dropped: u64,
    /// 连接状态
    pub is_up: bool,
}

/// 网络总体指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTotalMetrics {
    /// 总接收字节数/秒
    pub total_rx_bytes_per_sec: u64,
    /// 总发送字节数/秒
    pub total_tx_bytes_per_sec: u64,
    /// 活跃连接数
    pub active_connections: u32,
    /// TCP 连接数
    pub tcp_connections: u32,
    /// UDP 连接数
    pub udp_connections: u32,
}

/// GPU 指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuMetrics {
    /// GPU 设备列表
    pub devices: Vec<GpuDeviceMetrics>,
}

/// GPU 设备指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuDeviceMetrics {
    /// 设备ID
    pub device_id: u32,
    /// 设备名称
    pub name: String,
    /// GPU 使用率 (百分比)
    pub usage_percent: f32,
    /// 内存总量 (字节)
    pub memory_total_bytes: u64,
    /// 内存已使用 (字节)
    pub memory_used_bytes: u64,
    /// 内存使用率 (百分比)
    pub memory_usage_percent: f32,
    /// 温度 (摄氏度)
    pub temperature_celsius: f32,
    /// 功耗 (瓦特)
    pub power_usage_watts: f32,
    /// 风扇速度 (百分比)
    pub fan_speed_percent: f32,
    /// 时钟频率 (MHz)
    pub clock_speed_mhz: u32,
    /// 内存时钟频率 (MHz)
    pub memory_clock_mhz: u32,
}

/// 系统负载
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadAverage {
    /// 1分钟负载
    pub load_1m: f32,
    /// 5分钟负载
    pub load_5m: f32,
    /// 15分钟负载
    pub load_15m: f32,
    /// 运行中的进程数
    pub running_processes: u32,
    /// 总进程数
    pub total_processes: u32,
}

/// 应用程序指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationMetrics {
    /// 采样时间
    pub timestamp: DateTime<Utc>,
    /// 服务指标
    pub service: ServiceMetrics,
    /// 模型指标
    pub models: Vec<ModelMetrics>,
    /// 数据库指标
    pub database: DatabaseMetrics,
    /// 缓存指标
    pub cache: CacheMetrics,
    /// 队列指标
    pub queues: Vec<QueueMetrics>,
}

/// 服务指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetrics {
    /// 服务启动时间
    pub uptime_seconds: u64,
    /// 总请求数
    pub total_requests: u64,
    /// 成功请求数
    pub successful_requests: u64,
    /// 失败请求数
    pub failed_requests: u64,
    /// 活跃连接数
    pub active_connections: u32,
    /// 平均响应时间 (毫秒)
    pub avg_response_time_ms: f32,
    /// 95百分位响应时间 (毫秒)
    pub p95_response_time_ms: f32,
    /// 99百分位响应时间 (毫秒)
    pub p99_response_time_ms: f32,
    /// 当前QPS
    pub current_qps: f32,
    /// 峰值QPS
    pub peak_qps: f32,
    /// 错误率 (百分比)
    pub error_rate_percent: f32,
    /// 服务健康状态
    pub health_status: HealthStatus,
}

/// 健康状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    /// 健康
    Healthy,
    /// 警告
    Warning,
    /// 不健康
    Unhealthy,
    /// 严重
    Critical,
}

/// 模型指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    /// 模型ID
    pub model_id: Uuid,
    /// 模型名称
    pub model_name: String,
    /// 运行时ID
    pub runtime_id: Option<Uuid>,
    /// 状态
    pub status: crate::ModelStatus,
    /// 总请求数
    pub total_requests: u64,
    /// 成功请求数
    pub successful_requests: u64,
    /// 失败请求数
    pub failed_requests: u64,
    /// 平均推理时间 (毫秒)
    pub avg_inference_time_ms: f32,
    /// Token 生成速度 (tokens/秒)
    pub tokens_per_second: f32,
    /// 内存使用量 (字节)
    pub memory_usage_bytes: u64,
    /// GPU 内存使用量 (字节)
    pub gpu_memory_usage_bytes: Option<u64>,
    /// CPU 使用率 (百分比)
    pub cpu_usage_percent: f32,
    /// GPU 使用率 (百分比)
    pub gpu_usage_percent: Option<f32>,
    /// 队列长度
    pub queue_length: u32,
    /// 最后请求时间
    pub last_request_time: Option<DateTime<Utc>>,
}

/// 数据库指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseMetrics {
    /// 连接池大小
    pub pool_size: u32,
    /// 活跃连接数
    pub active_connections: u32,
    /// 空闲连接数
    pub idle_connections: u32,
    /// 总查询数
    pub total_queries: u64,
    /// 成功查询数
    pub successful_queries: u64,
    /// 失败查询数
    pub failed_queries: u64,
    /// 平均查询时间 (毫秒)
    pub avg_query_time_ms: f32,
    /// 慢查询数 (> 1秒)
    pub slow_queries: u64,
    /// 数据库大小 (字节)
    pub database_size_bytes: u64,
    /// 最后备份时间
    pub last_backup_time: Option<DateTime<Utc>>,
}

/// 缓存指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetrics {
    /// 缓存类型
    pub cache_type: crate::CacheType,
    /// 总条目数
    pub total_entries: u64,
    /// 已使用内存 (字节)
    pub used_memory_bytes: u64,
    /// 最大内存 (字节)
    pub max_memory_bytes: u64,
    /// 内存使用率 (百分比)
    pub memory_usage_percent: f32,
    /// 缓存命中数
    pub cache_hits: u64,
    /// 缓存未命中数
    pub cache_misses: u64,
    /// 缓存命中率 (百分比)
    pub hit_rate_percent: f32,
    /// 过期条目数
    pub expired_entries: u64,
    /// 驱逐条目数
    pub evicted_entries: u64,
}

/// 队列指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueMetrics {
    /// 队列名称
    pub queue_name: String,
    /// 队列长度
    pub queue_length: u32,
    /// 等待中的任务数
    pub pending_tasks: u32,
    /// 正在处理的任务数
    pub processing_tasks: u32,
    /// 已完成的任务数
    pub completed_tasks: u64,
    /// 失败的任务数
    pub failed_tasks: u64,
    /// 平均处理时间 (毫秒)
    pub avg_processing_time_ms: f32,
    /// 平均等待时间 (毫秒)
    pub avg_waiting_time_ms: f32,
    /// 吞吐量 (任务/秒)
    pub throughput_tps: f32,
}

/// 告警事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertEvent {
    /// 告警ID
    pub id: Uuid,
    /// 告警类型
    pub alert_type: AlertType,
    /// 严重程度
    pub severity: AlertSeverity,
    /// 告警标题
    pub title: String,
    /// 告警描述
    pub description: String,
    /// 触发时间
    pub triggered_at: DateTime<Utc>,
    /// 解决时间
    pub resolved_at: Option<DateTime<Utc>>,
    /// 告警状态
    pub status: AlertStatus,
    /// 关联资源
    pub resource: AlertResource,
    /// 告警值
    pub value: f32,
    /// 阈值
    pub threshold: f32,
    /// 标签
    pub labels: HashMap<String, String>,
}

/// 告警类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertType {
    /// CPU使用率高
    HighCpuUsage,
    /// 内存使用率高
    HighMemoryUsage,
    /// 磁盘使用率高
    HighDiskUsage,
    /// 网络错误率高
    HighNetworkErrors,
    /// 服务响应时间长
    HighResponseTime,
    /// 错误率高
    HighErrorRate,
    /// 队列积压
    QueueBacklog,
    /// 数据库连接池满
    DatabasePoolFull,
    /// 缓存命中率低
    LowCacheHitRate,
    /// 模型推理失败
    ModelInferenceFailed,
    /// 服务不可用
    ServiceUnavailable,
    /// GPU 温度过高
    HighGpuTemperature,
    /// 自定义告警
    Custom(String),
}

/// 告警严重程度
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    /// 信息
    Info,
    /// 警告
    Warning,
    /// 严重
    Critical,
    /// 紧急
    Emergency,
}

/// 告警状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertStatus {
    /// 触发
    Triggered,
    /// 确认
    Acknowledged,
    /// 解决
    Resolved,
    /// 抑制
    Suppressed,
}

/// 告警资源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertResource {
    /// 资源类型
    pub resource_type: String,
    /// 资源ID
    pub resource_id: String,
    /// 资源名称
    pub resource_name: String,
    /// 额外信息
    pub metadata: HashMap<String, String>,
}

impl SystemMetrics {
    /// 创建新的系统指标
    pub fn new() -> Self {
        Self {
            timestamp: Utc::now(),
            cpu: CpuMetrics::default(),
            memory: MemoryMetrics::default(),
            disk: DiskMetrics::default(),
            network: NetworkMetrics::default(),
            gpu: None,
            load_average: LoadAverage::default(),
        }
    }
}

impl Default for CpuMetrics {
    fn default() -> Self {
        Self {
            usage_percent: 0.0,
            user_percent: 0.0,
            system_percent: 0.0,
            idle_percent: 100.0,
            iowait_percent: 0.0,
            core_count: 1,
            per_core_usage: Vec::new(),
            frequency_mhz: 0,
            temperature_celsius: None,
        }
    }
}

impl Default for MemoryMetrics {
    fn default() -> Self {
        Self {
            total_bytes: 0,
            used_bytes: 0,
            available_bytes: 0,
            usage_percent: 0.0,
            cached_bytes: 0,
            buffer_bytes: 0,
            swap_total_bytes: 0,
            swap_used_bytes: 0,
            swap_usage_percent: 0.0,
        }
    }
}

impl Default for DiskMetrics {
    fn default() -> Self {
        Self {
            partitions: Vec::new(),
            io: DiskIoMetrics::default(),
        }
    }
}

impl Default for DiskIoMetrics {
    fn default() -> Self {
        Self {
            read_bytes_per_sec: 0,
            write_bytes_per_sec: 0,
            read_ops_per_sec: 0,
            write_ops_per_sec: 0,
            avg_read_latency_ms: 0.0,
            avg_write_latency_ms: 0.0,
            utilization_percent: 0.0,
        }
    }
}

impl Default for NetworkMetrics {
    fn default() -> Self {
        Self {
            interfaces: Vec::new(),
            total: NetworkTotalMetrics::default(),
        }
    }
}

impl Default for NetworkTotalMetrics {
    fn default() -> Self {
        Self {
            total_rx_bytes_per_sec: 0,
            total_tx_bytes_per_sec: 0,
            active_connections: 0,
            tcp_connections: 0,
            udp_connections: 0,
        }
    }
}

impl Default for LoadAverage {
    fn default() -> Self {
        Self {
            load_1m: 0.0,
            load_5m: 0.0,
            load_15m: 0.0,
            running_processes: 0,
            total_processes: 0,
        }
    }
}

impl AlertEvent {
    /// 创建新的告警事件
    pub fn new(
        alert_type: AlertType,
        severity: AlertSeverity,
        title: String,
        description: String,
        resource: AlertResource,
        value: f32,
        threshold: f32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            alert_type,
            severity,
            title,
            description,
            triggered_at: Utc::now(),
            resolved_at: None,
            status: AlertStatus::Triggered,
            resource,
            value,
            threshold,
            labels: HashMap::new(),
        }
    }

    /// 解决告警
    pub fn resolve(&mut self) {
        self.status = AlertStatus::Resolved;
        self.resolved_at = Some(Utc::now());
    }

    /// 确认告警
    pub fn acknowledge(&mut self) {
        self.status = AlertStatus::Acknowledged;
    }

    /// 获取告警持续时间 (秒)
    pub fn duration_seconds(&self) -> i64 {
        let end_time = self.resolved_at.unwrap_or(Utc::now());
        end_time.timestamp() - self.triggered_at.timestamp()
    }
}