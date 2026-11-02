//! 设备数据模块 - 定义解析后的设备数据结构

use super::device_config::{ParameterValue, SingleParameterConfig};
use super::device_type::DeviceType;
use super::error::{ParseError, ParseResult};
use crate::utils::crc16;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub parameter_name: String,
    pub raw_value: ParameterValue,
    pub physical_value: f64,
    pub unit: String,
}

impl Parameter {
    pub fn new(
        parameter_name: String,
        raw_value: ParameterValue,
        physical_value: f64,
        unit: String,
    ) -> Self {
        Self {
            parameter_name,
            raw_value,
            physical_value,
            unit,
        }
    }

    pub fn from_config(config: &SingleParameterConfig, raw_value: ParameterValue) -> Self {
        let physical_value = raw_value.to_physical_value(config.lsb);
        Self {
            parameter_name: config.name.to_string(),
            raw_value,
            physical_value,
            unit: config.unit.to_string(),
        }
    }

    pub fn raw_data_len(&self) -> usize {
        self.raw_value.get_len()
    }
}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.unit.is_empty() {
            write!(f, "{}: {:.3}", self.parameter_name, self.physical_value)
        } else {
            write!(
                f,
                "{}: {:.3} {}",
                self.parameter_name, self.physical_value, self.unit
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DeviceData {
    device_type: DeviceType,
    parameters: Vec<Parameter>,
}

impl DeviceData {
    pub fn new(device_type: DeviceType, parameters: Vec<Parameter>) -> Self {
        Self {
            device_type,
            parameters,
        }
    }

    pub fn device_type(&self) -> DeviceType {
        self.device_type
    }

    pub fn from_bytes(device_type: DeviceType, data: &[u8]) -> ParseResult<Self> {
        let parameters_info = device_type.info();
        let mut parameters = Vec::with_capacity(parameters_info.parameters.len());
        let mut offset = 0;

        if parameters_info.is_variable_length() {
            if device_type != DeviceType::ModbusTerminal {
                return Err(ParseError::ModbusError {
                    reason: "Unsupported device type for variable length data".to_string(),
                });
            }
            // TODO: 实现在 parser.rs
        } else {
            let datalength = parameters_info.get_fixed_data_length().unwrap();
            parameters_info.parameters.iter().try_for_each(
                |single_parameter_config| -> ParseResult<()> {
                    let value = single_parameter_config
                        .data_type
                        .parse_from_bytes(data, offset)?;
                    parameters.push(Parameter::from_config(single_parameter_config, value));
                    offset += value.get_len();
                    Ok(())
                },
            )?;
            if datalength != offset {
                return Err(ParseError::InvalidDataFormat {
                    data_type: 0,
                    reason: format!(
                        "Alarm data parsing failed validation: offset[{}] != data_length[{}]",
                        offset, datalength
                    ),
                });
            }
        }

        Ok(Self {
            device_type,
            parameters,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let byte_length = self.get_data_length();

        let mut bytes = Vec::with_capacity(byte_length);

        self.parameters.iter().for_each(|p| {
            bytes.extend_from_slice(&p.raw_value.to_bytes());
        });

        bytes
    }

    pub fn get_data_length(&self) -> usize {
        self.parameters
            .iter()
            .map(|p| p.raw_data_len())
            .sum::<usize>()
    }

    pub fn get_parameters(&self) -> &Vec<Parameter> {
        &self.parameters
    }

    pub fn get_parameter_by_index(&self, index: usize) -> Option<&Parameter> {
        self.parameters.get(index)
    }

    /// 通过名称获取参数
    pub fn get_parameter_by_name(&self, name: &str) -> Option<&Parameter> {
        self.parameters.iter().find(|p| p.parameter_name == name)
    }

    pub fn get_physical_value(&self, name: &str) -> ParseResult<f64> {
        self.get_parameter_by_name(name)
            .map(|p| p.physical_value)
            .ok_or_else(|| ParseError::ParameterNotFound {
                name: name.to_string(),
                device_type: self.device_type.to_u8(),
            })
    }

    pub fn get_raw_value(&self, name: &str) -> ParseResult<&ParameterValue> {
        self.get_parameter_by_name(name)
            .map(|p| &p.raw_value)
            .ok_or_else(|| ParseError::ParameterNotFound {
                name: name.to_string(),
                device_type: self.device_type.to_u8(),
            })
    }

    pub fn get_description(&self) -> String {
        let mut result = format!(
            "设备类型: {} ({})\n",
            self.device_type.name(),
            self.device_type.to_u8()
        );
        result.push_str(" 参数:\n");
        for param in &self.parameters {
            result.push_str(&format!("  - {}\n", param.to_string()));
        }
        result
    }

    /// 转换为JSON格式的字符串（手动构建，不依赖serde）
    pub fn to_json_string(&self) -> String {
        let mut json = format!(
            r#"{{"device_type":{},"device_name":"{}","parameters":["#,
            self.device_type.to_u8(),
            self.device_type.name(),
        );

        for (i, param) in self.parameters.iter().enumerate() {
            if i > 0 {
                json.push(',');
            }
            json.push_str(&format!(
                r#"{{"name":"{}","value":{},"unit":"{}"}}"#,
                param.parameter_name, param.physical_value, param.unit
            ));
        }

        json.push_str("]}");
        json
    }
}

impl std::fmt::Display for DeviceData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_description())
    }
}

/// Modbus采集终端数据
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ModbusData {
    pub address: u8,
    pub function_code: u8,
    pub byte_count: u8,
    pub payload: Vec<u8>,
    pub crc: u16,
}

impl ModbusData {
    pub fn new(address: u8, function_code: u8, byte_count: u8, payload: Vec<u8>, crc: u16) -> Self {
        assert!(
            byte_count as usize == payload.len(),
            "The number of bytes does not match the data length"
        );
        Self {
            address,
            function_code,
            byte_count,
            payload,
            crc,
        }
    }

    pub(crate) fn verify_crc(&self) -> ParseResult<()> {
        let mut bytes = vec![self.address, self.function_code, self.byte_count];
        bytes.extend_from_slice(&self.payload);
        let calculated_crc = crc16(&bytes);
        if calculated_crc != self.crc {
            Err(ParseError::CrcCheckFailed {
                expected: calculated_crc,
                actual: self.crc,
            })
        } else {
            Ok(())
        }
    }

    pub fn get_description(&self) -> String {
        format!(
            "Modbus数据:\n  地址: 0x{:02X}\n  功能码: 0x{:02X}\n  字节数: {}\n  数据: {:02X?}\n  CRC: 0x{:04X}",
            self.address, self.function_code, self.byte_count, self.payload, self.crc
        )
    }

    pub fn from_bytes(data: &[u8]) -> ParseResult<Self> {
        if data.len() < 5 {
            return Err(ParseError::InsufficientData {
                field: "Modbus data".to_string(),
                expected: 5,
                actual: data.len(),
            });
        }
        let address = data[0];
        let function_code = data[1];
        let byte_count = data[2];
        if data.len() < 5 + byte_count as usize {
            return Err(ParseError::InsufficientData {
                field: "Modbus data".to_string(),
                expected: 5 + byte_count as usize,
                actual: data.len(),
            });
        }
        let payload = data[3..(3 + byte_count as usize)].to_vec();
        let crc_index = 3 + byte_count as usize;

        // TODO: CRC校验值目前使用大端解析
        let crc = u16::from_be_bytes([data[crc_index], data[crc_index + 1]]);

        let self_temp = Self::new(address, function_code, byte_count, payload, crc);
        self_temp.verify_crc()?;
        Ok(self_temp)
    }

    pub fn data_length(&self) -> usize {
        self.payload.len() + 5
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.data_length());
        bytes.push(self.address);
        bytes.push(self.function_code);
        bytes.push(self.byte_count);
        bytes.extend_from_slice(&self.payload);
        bytes.extend_from_slice(&self.crc.to_be_bytes());
        bytes
    }
}

impl std::fmt::Display for ModbusData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_description())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_creation() {
        let param = Parameter::new(
            "压力".to_string(),
            ParameterValue::I16(1563),
            1.563,
            "MPa".to_string(),
        );

        assert_eq!(param.parameter_name, "压力");
        assert_eq!(param.physical_value, 1.563);
        assert_eq!(param.unit, "MPa");
    }

    #[test]
    fn test_parameter_display() {
        let param = Parameter::new(
            "压力".to_string(),
            ParameterValue::I16(1563),
            1.563,
            "MPa".to_string(),
        );

        assert_eq!(param.to_string(), "压力: 1.563 MPa");
    }

    #[test]
    fn test_device_data_get_value() {
        let device_type = DeviceType::from_u8(1).unwrap();
        let params = vec![Parameter::new(
            "压力".to_string(),
            ParameterValue::I16(1563),
            1.563,
            "MPa".to_string(),
        )];

        let device_data = DeviceData::new(device_type, params);

        assert_eq!(device_data.get_physical_value("压力").unwrap(), 1.563);
        assert!(device_data.get_physical_value("温度").is_err());
    }

    #[test]
    fn test_modbus_crc() {
        // 测试数据: 地址=0x01, 功能码=0x03, 字节数=0x04, 数据=[0x00, 0x01, 0x00, 0x02]
        let data = vec![0x01, 0x03, 0x04, 0x00, 0x01, 0x00, 0x02];
        let crc = crc16(&data);

        // CRC应该是确定的值
        assert!(crc != 0);
    }

    #[test]
    fn test_modbus_data_verify() {
        let data = vec![0x00, 0x01, 0x00, 0x02];
        let calculated_crc = crc16(&[0x01, 0x03, 0x04, 0x00, 0x01, 0x00, 0x02]);

        let modbus_data = ModbusData::new(0x01, 0x03, 0x04, data, calculated_crc);
        assert!(modbus_data.verify_crc().is_ok());
    }

    #[test]
    fn test_device_data_json() {
        let device_type = DeviceType::from_u8(1).unwrap();
        let params = vec![Parameter::new(
            "压力".to_string(),
            ParameterValue::I16(1563),
            1.563,
            "MPa".to_string(),
        )];

        let device_data = DeviceData::new(device_type, params);
        let json = device_data.to_json_string();

        assert!(json.contains("device_type"));
        assert!(json.contains("压力"));
        assert!(json.contains("1.563"));
    }
}
