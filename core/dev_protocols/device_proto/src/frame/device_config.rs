//! 设备配置模块 - 定义设备类型的元数据和参数配置

use super::error::{ParseError, ParseResult};
use serde::{Deserialize, Serialize};

/// 参数数据类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataType {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    F32,
    F64,
}

impl DataType {
    pub const fn byte_size(&self) -> usize {
        match self {
            DataType::I8 | DataType::U8 => 1,
            DataType::I16 | DataType::U16 => 2,
            DataType::I32 | DataType::U32 | DataType::F32 => 4,
            DataType::F64 => 8,
        }
    }

    /// 从字节数组解析数据（大端字节序）
    pub fn parse_from_bytes(&self, data: &[u8], offset: usize) -> ParseResult<ParameterValue> {
        let size = self.byte_size();
        if offset + size > data.len() {
            return Err(ParseError::OutOfBounds {
                offset,
                length: size,
                total: data.len(),
            });
        }

        match self {
            DataType::I8 => Ok(ParameterValue::I8(data[offset] as i8)),
            DataType::U8 => Ok(ParameterValue::U8(data[offset])),
            DataType::I16 => {
                let bytes = [data[offset], data[offset + 1]];
                Ok(ParameterValue::I16(i16::from_be_bytes(bytes)))
            }
            DataType::U16 => {
                let bytes = [data[offset], data[offset + 1]];
                Ok(ParameterValue::U16(u16::from_be_bytes(bytes)))
            }
            DataType::I32 => {
                let bytes = [
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ];
                Ok(ParameterValue::I32(i32::from_be_bytes(bytes)))
            }
            DataType::U32 => {
                let bytes = [
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ];
                Ok(ParameterValue::U32(u32::from_be_bytes(bytes)))
            }
            DataType::F32 => {
                let bytes = [
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ];
                Ok(ParameterValue::F32(f32::from_be_bytes(bytes)))
            }
            DataType::F64 => {
                let bytes = [
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                    data[offset + 4],
                    data[offset + 5],
                    data[offset + 6],
                    data[offset + 7],
                ];
                Ok(ParameterValue::F64(f64::from_be_bytes(bytes)))
            }
        }
    }
}

/// 解析后的参数值
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Copy)]
pub enum ParameterValue {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    F32(f32),
    F64(f64),
}

impl ParameterValue {
    pub fn to_physical_value(&self, lsb: f64) -> f64 {
        self.to_f64() * lsb
    }

    pub fn to_i64(&self) -> Option<i64> {
        match self {
            ParameterValue::I8(v) => Some(*v as i64),
            ParameterValue::U8(v) => Some(*v as i64),
            ParameterValue::I16(v) => Some(*v as i64),
            ParameterValue::U16(v) => Some(*v as i64),
            ParameterValue::I32(v) => Some(*v as i64),
            ParameterValue::U32(v) => Some(*v as i64),
            _ => None,
        }
    }

    pub fn to_u64(&self) -> Option<u64> {
        match self {
            ParameterValue::U8(v) => Some(*v as u64),
            ParameterValue::I8(v) => Some(*v as u64),
            ParameterValue::U16(v) => Some(*v as u64),
            ParameterValue::U32(v) => Some(*v as u64),
            ParameterValue::I16(v) if *v >= 0 => Some(*v as u64),
            ParameterValue::I32(v) if *v >= 0 => Some(*v as u64),
            _ => None,
        }
    }

    pub fn to_f64(&self) -> f64 {
        match self {
            ParameterValue::I8(v) => *v as f64,
            ParameterValue::U8(v) => *v as f64,
            ParameterValue::I16(v) => *v as f64,
            ParameterValue::U16(v) => *v as f64,
            ParameterValue::I32(v) => *v as f64,
            ParameterValue::U32(v) => *v as f64,
            ParameterValue::F32(v) => *v as f64,
            ParameterValue::F64(v) => *v,
        }
    }

    pub fn get_len(&self) -> usize {
        match self {
            ParameterValue::I8(_) | ParameterValue::U8(_) => 1,
            ParameterValue::I16(_) | ParameterValue::U16(_) => 2,
            ParameterValue::I32(_) | ParameterValue::U32(_) => 4,
            ParameterValue::F32(_) | ParameterValue::F64(_) => 8,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            ParameterValue::I8(v) => v.to_be_bytes().to_vec(),
            ParameterValue::U8(v) => v.to_be_bytes().to_vec(),
            ParameterValue::I16(v) => v.to_be_bytes().to_vec(),
            ParameterValue::U16(v) => v.to_be_bytes().to_vec(),
            ParameterValue::I32(v) => v.to_be_bytes().to_vec(),
            ParameterValue::U32(v) => v.to_be_bytes().to_vec(),
            ParameterValue::F32(v) => v.to_be_bytes().to_vec(),
            ParameterValue::F64(v) => v.to_be_bytes().to_vec(),
        }
    }
}

/// 单个参数的配置信息
#[derive(Debug, Clone)]
pub struct SingleParameterConfig {
    pub(crate) name: &'static str,
    pub(crate) data_type: DataType,
    pub(crate) unit: &'static str,
    pub(crate) lsb: f64,
    pub(crate) offset: usize,
}

impl SingleParameterConfig {
    pub const fn new(
        name: &'static str,
        data_type: DataType,
        unit: &'static str,
        lsb: f64,
        offset: usize,
    ) -> Self {
        Self {
            name,
            data_type,
            unit,
            lsb,
            offset,
        }
    }
}

/// 数据长度类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataLength {
    Fixed(usize),
    /// 不定长
    Variable, // 目前唯一对应Modbus采集终端数据
}

impl DataLength {
    pub(crate) fn fixed_value(&self) -> Option<usize> {
        match self {
            DataLength::Fixed(len) => Some(*len),
            DataLength::Variable => None,
        }
    }

    pub(crate) fn is_variable(&self) -> bool {
        matches!(self, DataLength::Variable)
    }
}

/// 设备类型元数据
#[derive(Debug, Clone)]
pub struct DeviceTypeInfo {
    pub type_id: u8,
    pub name: &'static str,
    pub alarm_groups: u8,
    pub(crate) data_length: DataLength,
    pub(crate) parameters: &'static [SingleParameterConfig],
}

impl DeviceTypeInfo {
    pub const fn new(
        type_id: u8,
        name: &'static str,
        alarm_groups: u8,
        data_length: DataLength,
        parameters: &'static [SingleParameterConfig],
    ) -> Self {
        Self {
            type_id,
            name,
            alarm_groups,
            data_length,
            parameters,
        }
    }

    pub fn get_fixed_data_length(&self) -> Option<usize> {
        self.data_length.fixed_value()
    }

    pub fn is_variable_length(&self) -> bool {
        self.data_length.is_variable()
    }

    pub fn calculate_data_length(&self, data: &[u8]) -> ParseResult<usize> {
        match self.data_length {
            DataLength::Fixed(len) => Ok(len),
            DataLength::Variable => {
                // 主要针对 Modbus 采集数据
                if self.type_id == 45 {
                    if data.len() < 3 {
                        return Err(ParseError::InsufficientData {
                            expected: 3,
                            actual: data.len(),
                            field: "Modbus data length".to_string(),
                        });
                    }
                    let n = data[2] as usize;
                    Ok(1 + 1 + 1 + n + 2)
                } else {
                    Err(ParseError::InvalidDataFormat {
                        data_type: self.type_id,
                        reason: "不支持的变长数据类型".to_string(),
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_type_byte_size() {
        assert_eq!(DataType::I16.byte_size(), 2);
        assert_eq!(DataType::U16.byte_size(), 2);
        assert_eq!(DataType::I32.byte_size(), 4);
        assert_eq!(DataType::U32.byte_size(), 4);
        assert_eq!(DataType::F32.byte_size(), 4);
        assert_eq!(DataType::F64.byte_size(), 8);
    }

    #[test]
    fn test_parse_i16() {
        let data = [0x06, 0x1B]; // 1563 in big-endian
        let value = DataType::I16.parse_from_bytes(&data, 0).unwrap();
        assert_eq!(value, ParameterValue::I16(1563));
    }

    #[test]
    fn test_parameter_value_to_physical() {
        let value = ParameterValue::I16(1563);
        let physical = value.to_physical_value(0.001);
        assert!((physical - 1.563).abs() < 0.0001);
    }

    #[test]
    fn test_data_length() {
        let fixed = DataLength::Fixed(10);
        assert_eq!(fixed.fixed_value(), Some(10));
        assert!(!fixed.is_variable());

        let variable = DataLength::Variable;
        assert_eq!(variable.fixed_value(), None);
        assert!(variable.is_variable());
    }
}
