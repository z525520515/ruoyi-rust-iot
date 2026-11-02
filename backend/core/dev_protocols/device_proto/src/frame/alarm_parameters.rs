use super::error::{ParseError, ParseResult};
use super::{DataType, DeviceType, ParameterValue};
use serde::{Deserialize, Serialize};

/// 报警参数
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AlarmParameters {
    alarm_group: u8,
    parameters_type: Vec<DataType>,
    data: Vec<u8>,
    result: Vec<ParameterValue>,
}

impl AlarmParameters {
    pub fn from_device_type(device_type: DeviceType) -> ParseResult<Self> {
        let alarm_group = device_type.alarm_groups();
        let parameters_info = device_type.get_parameters_info();
        let parameters_type = match alarm_group {
            1 => {
                if parameters_info.len() < 1 {
                    return Err(ParseError::InsufficientData {
                        field: format!("Parameters number of {}", device_type.name()),
                        expected: 1,
                        actual: parameters_info.len(),
                    });
                }
                let mut temp_data = vec![];
                temp_data.push(parameters_info[0].data_type);
                temp_data.push(parameters_info[0].data_type);
                temp_data
            }
            2 => {
                if parameters_info.len() < 2 {
                    return Err(ParseError::InsufficientData {
                        field: format!("Parameters number of {}", device_type.name()),
                        expected: 2,
                        actual: parameters_info.len(),
                    });
                }
                let mut temp_data = vec![];
                temp_data.push(parameters_info[0].data_type);
                temp_data.push(parameters_info[1].data_type);
                temp_data.push(parameters_info[0].data_type);
                temp_data.push(parameters_info[1].data_type);
                temp_data
            }
            n => {
                return Err(ParseError::InvalidDataType {
                    field: format!("Parameters number of {}", device_type.name()),
                    value: n,
                });
            }
        };

        let data_len = parameters_type.iter().map(|t| t.byte_size()).sum::<usize>();

        Ok(AlarmParameters {
            alarm_group,
            parameters_type,
            data: Vec::with_capacity(data_len),
            result: Vec::with_capacity(if alarm_group == 1 { 2 } else { 4 }),
        })
    }

    // TODO: 添加函数调用检查约束
    pub fn init_with_data(&mut self, data: &[u8]) -> Result<(), ParseError> {
        let expected_len = self.get_data_len();
        if data.len() < expected_len {
            return Err(ParseError::InsufficientData {
                field: "AlarmParameters data".to_string(),
                expected: expected_len,
                actual: data.len(),
            });
        }
        self.data.extend_from_slice(&data[..expected_len]);

        let mut offset = 0;
        self.parameters_type
            .iter()
            .try_for_each(|data_type| -> ParseResult<()> {
                let value = data_type.parse_from_bytes(&self.data, offset)?;
                offset += value.get_len();
                self.result.push(value);
                Ok(())
            })
    }

    pub fn get_data_len(&self) -> usize {
        self.parameters_type.iter().map(|p| p.byte_size()).sum()
    }

    pub fn get_result(&self) -> &[ParameterValue] {
        &self.result
    }

    pub fn get_alarm_group(&self) -> u8 {
        self.alarm_group
    }

    // DEBUG
    pub fn get_raw_data(&self) -> &[u8] {
        &self.data
    }

    pub fn get_alarm_upper_limit(&self, index_group: usize) -> Option<ParameterValue> {
        // assert!(index_group < 2, "Index out of range");
        if index_group >= self.get_alarm_group() as usize {
            None
        } else {
            Some(self.result[index_group * 2 + 1])
        }
    }

    pub fn get_alarm_lower_limit(&self, index_group: usize) -> Option<ParameterValue> {
        // assert!(index_group < 2, "Index out of range");
        if index_group >= self.get_alarm_group() as usize {
            None
        } else {
            Some(self.result[index_group * 2])
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }
}
