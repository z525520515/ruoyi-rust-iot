//! 设备数据解析错误类型定义

use std::fmt;
use std::io;

/// 设备数据解析错误
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// 数据长度不足
    InsufficientData {
        /// 字段
        field: String,
        /// 期望的最小字节数
        expected: usize,
        /// 实际的字节数
        actual: usize,
    },

    /// 数据越界
    OutOfBounds {
        /// 尝试访问的偏移量
        offset: usize,
        /// 尝试读取的字节数
        length: usize,
        /// 数据总长度
        total: usize,
    },

    /// 无效的数据类型
    InvalidDataType {
        field: String,
        /// 无效类型值
        value: u8,
    },

    /// 无效的参数索引
    InvalidParameterIndex {
        /// 参数索引
        index: usize,
        /// 设备类型
        device_type: u8,
    },

    /// 无效的数据格式
    InvalidDataFormat {
        ///
        data_type: u8,
        /// 错误描述
        reason: String,
    },

    /// Modbus特定错误
    ModbusError {
        /// 错误描述
        reason: String,
    },

    /// CRC校验失败
    CrcCheckFailed {
        /// 期望的CRC值
        expected: u16,
        /// 实际计算的CRC值
        actual: u16,
    },

    /// 参数未找到
    ParameterNotFound {
        /// 参数名称
        name: String,
        /// 设备类型
        device_type: u8,
    },

    /// 未初始化
    Uninitialized { field: &'static str, reason: String },

    /// 字符串解析错误
    StringParseError(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InsufficientData {
                field,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "{} 数据长度不足: 期望至少 {} 字节，实际只有 {} 字节",
                    field, expected, actual
                )
            }
            ParseError::OutOfBounds {
                offset,
                length,
                total,
            } => {
                write!(
                    f,
                    "数据访问越界: 尝试在偏移 {} 处读取 {} 字节，但数据总长度只有 {} 字节",
                    offset, length, total
                )
            }
            ParseError::InvalidDataType { field, value } => {
                write!(f, "无效的数据类型: {} : {}", field, value)
            }
            ParseError::InvalidParameterIndex { index, device_type } => {
                write!(
                    f,
                    "无效的参数索引: 设备类型 {} 不存在索引为 {} 的参数",
                    device_type, index
                )
            }
            ParseError::InvalidDataFormat { data_type, reason } => {
                write!(f, "数据类型 {} 的数据格式无效: {}", data_type, reason)
            }
            ParseError::ModbusError { reason } => {
                write!(f, "Modbus解析错误: {}", reason)
            }
            ParseError::CrcCheckFailed { expected, actual } => {
                write!(
                    f,
                    "CRC校验失败: 期望 0x{:04X}，实际 0x{:04X}",
                    expected, actual
                )
            }
            ParseError::ParameterNotFound { name, device_type } => {
                write!(
                    f,
                    "参数未找到: 设备类型 {} 中不存在名为 '{}' 的参数",
                    device_type, name
                )
            }
            ParseError::Uninitialized { field, reason } => {
                write!(f, "字段 {} 未初始化: {}", field, reason)
            }
            ParseError::StringParseError(msg) => {
                write!(f, "字符串解析错误: {}", msg)
            }
        }
    }
}

impl std::error::Error for ParseError {}

/// 解析结果类型别名
pub type ParseResult<T> = Result<T, ParseError>;

impl From<io::Error> for ParseError {
    fn from(value: std::io::Error) -> Self {
        ParseError::InvalidDataFormat {
            data_type: 0,
            reason: format!("IO ERROR: {}", value),
        }
    }
}
