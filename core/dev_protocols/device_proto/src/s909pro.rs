use crate::constants::{FRAME_END, FRAME_START, command};
use crate::frame;
use crate::utils::crc16;
use byteorder::{BigEndian, ReadBytesExt};
use frame::error::{ParseError, ParseResult};
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S909UP {
    /// 帧起始标记
    pub frame_start: [u8; 2],
    /// 命令类型
    pub command_type: u8,
    /// 帧总长度
    pub frame_length: frame::FrameLength,
    /// 设备编号
    pub device_id: frame::DeviceId,
    /// 协议版本
    pub protocol_version: frame::ProtocolVersion,
    /// 电量状态
    pub power_status: frame::PowerStatus,
    /// 信号强度
    pub signal_strength: frame::SignalStrength,
    /// 通道号
    pub channel_id: frame::ChannelId,
    /// 设备类型
    pub device_type: frame::DeviceType,
    /// 设备报警状态
    pub device_alarm_status: frame::AlarmStatus,
    /// UTC 时间戳
    pub timestamp: frame::Timestamp,
    /// 通道类型
    pub channel_type: frame::ChannelType,
    /// 通道数据长度
    pub channel_data_length: frame::ChannelDataLength,
    /// 通道数据
    pub channel_data: frame::ChannelDataType,
    /// 单位长度
    pub unit_length: frame::UnitLength,
    /// 单位值
    pub unit_value: frame::UnitContent,
    /// CRC 校验值
    pub crc: u16,
    /// 帧结尾标记
    pub frame_end: [u8; 2],
}

impl S909UP {
    /// 从完整数据中解析消息
    pub fn from_bytes(bytes: &[u8]) -> ParseResult<S909UP> {
        let bytes_len = bytes.len();
        if bytes_len < 36 {
            return Err(ParseError::InsufficientData {
                field: "上发数据帧".to_string(),
                expected: 36,
                actual: bytes.len(),
            });
        }

        let mut data_append = Vec::<u8>::with_capacity(36);
        let mut cursor = Cursor::new(bytes);

        let mut frame_start = [0u8; 2];
        cursor
            .read_exact(&mut frame_start)
            .map_err(|e| ParseError::InvalidDataFormat {
                data_type: 0,
                reason: format!("Failed to read frame start: {}", e),
            })?;

        if frame_start != FRAME_START {
            return Err(ParseError::InvalidDataFormat {
                data_type: 1,
                reason: format!(
                    "Invalid frame start: [{:02X} {:02X}]",
                    frame_start[0], frame_start[1]
                ),
            });
        }
        data_append.extend_from_slice(&frame_start);

        let command_type = cursor
            .read_u8()
            .map_err(|e| ParseError::InvalidDataFormat {
                data_type: 0,
                reason: format!("Failed to read command type: {}", e),
            })?;

        if command_type != command::UPLOAD_DATA {
            return Err(ParseError::InvalidDataFormat {
                data_type: 2,
                reason: format!("Invalid command type: {}", command_type),
            });
        }
        data_append.push(command_type);

        let mut frame_length = [0u8; 2];
        cursor
            .read_exact(&mut frame_length)
            .map_err(|e| ParseError::InvalidDataFormat {
                data_type: 0,
                reason: format!("Failed to read frame length: {}", e),
            })?;

        let frame_length = frame::FrameLength::from_slices(&frame_length);

        if frame_length.len() as usize > bytes_len {
            return Err(ParseError::InsufficientData {
                field: "上发数据帧".to_string(),
                expected: frame_length.len() as usize,
                actual: bytes_len,
            });
        }
        data_append.extend_from_slice(&frame_length.to_bytes());

        let mut device_id = [0u8; 12];
        cursor
            .read_exact(&mut device_id)
            .map_err(|e| ParseError::InvalidDataFormat {
                data_type: 0,
                reason: format!("Faild to read device ID: {}", e),
            })?;

        let device_id = frame::DeviceId::from_slices(&device_id);
        data_append.extend_from_slice(&device_id.to_bytes());

        let version = cursor
            .read_u8()
            .map_err(|e| ParseError::InvalidDataFormat {
                data_type: 0,
                reason: format!("Failed to read version: {}", e),
            })?;

        let protocol_version = frame::ProtocolVersion::from_u8(version);
        data_append.push(protocol_version.to_u8());

        let power_status = frame::PowerStatus::from_u8(cursor.read_u8().map_err(|e| {
            ParseError::InvalidDataFormat {
                data_type: 0,
                reason: format!("Faild to read power status: {}", e),
            }
        })?);
        data_append.push(power_status.to_u8());

        let signal_strength = frame::SignalStrength::from_u8(cursor.read_u8().map_err(|e| {
            ParseError::InvalidDataFormat {
                data_type: 0,
                reason: format!("Faild to read signal strength: {}", e),
            }
        })?);
        data_append.push(signal_strength.to_byte());

        let channel_id = frame::ChannelId::from_u8(cursor.read_u8().map_err(|e| {
            ParseError::InvalidDataFormat {
                data_type: 0,
                reason: format!("Faild to read channel ID: {}", e),
            }
        })?);
        data_append.push(channel_id.to_u8());

        let device_type_raw = cursor
            .read_u8()
            .map_err(|e| ParseError::InvalidDataFormat {
                data_type: 0,
                reason: format!("Faild to read device type: {}", e),
            })?;
        let device_type = frame::DeviceType::from_u8(device_type_raw)?;
        data_append.push(device_type.to_u8());

        let device_alarm_value = frame::AlarmStatus::from_u8(cursor.read_u8().map_err(|e| {
            ParseError::InvalidDataFormat {
                data_type: 0,
                reason: format!("Faild to read device alarm value: {}", e),
            }
        })?);
        data_append.push(device_alarm_value.to_u8());

        let mut timestamp = [0u8; 4];
        cursor
            .read_exact(&mut timestamp)
            .map_err(|e| ParseError::InvalidDataFormat {
                data_type: 0,
                reason: format!("Failed to read timestamp: {}", e),
            })?;
        let timestamp = frame::Timestamp::from_bytes(timestamp);
        data_append.extend_from_slice(&timestamp.to_bytes());

        let channel_type = frame::ChannelType::from_u8(cursor.read_u8().map_err(|e| {
            ParseError::InvalidDataFormat {
                data_type: 0,
                reason: format!("Failed to read channel type: {}", e),
            }
        })?);
        data_append.push(channel_type.to_u8());

        let mut channel_data_length = [0u8; 2];
        cursor
            .read_exact(&mut channel_data_length)
            .map_err(|e| ParseError::InvalidDataFormat {
                data_type: 0,
                reason: format!("Failed to read channel data length: {}", e),
            })?;
        let channel_data_length = frame::ChannelDataLength::from_bytes(&channel_data_length);
        data_append.extend_from_slice(&channel_data_length.to_bytes());

        // TODO: channel_data_length 可能为 4 或 6，但实际上可能存在其他长度的数据（透传未实现）
        let mut channel_data = vec![0u8; channel_data_length.to_u16() as usize];
        cursor
            .read_exact(&mut channel_data)
            .map_err(|e| ParseError::InvalidDataFormat {
                data_type: 0,
                reason: format!("Failed to read channel data: {}", e),
            })?;
        let channel_data =
            frame::ChannelDataType::from_channel_type(&channel_type).init_by_bytes(&channel_data);
        data_append.extend_from_slice(&channel_data.to_bytes());

        let unit_length: frame::UnitLength =
            cursor
                .read_u8()
                .map_err(|e| ParseError::InvalidDataFormat {
                    data_type: 0,
                    reason: format!("Failed to read unit length: {}", e),
                })?;
        data_append.push(unit_length);

        let mut unit_value = vec![0u8; unit_length as usize];
        cursor
            .read_exact(&mut unit_value)
            .map_err(|e| ParseError::InvalidDataFormat {
                data_type: 0,
                reason: format!("Failed to read unit value: {}", e),
            })?;
        let unit_value = frame::UnitContent::from_bytes(&unit_value, channel_type)?;
        data_append.extend_from_slice(&unit_value.to_bytes());

        // crc 校验
        let crc = cursor
            .read_u16::<BigEndian>()
            .map_err(|e| ParseError::InvalidDataFormat {
                data_type: 0,
                reason: format!("Failed to read crc: {}", e),
            })?;

        let expected_crc = crc16(&data_append[..data_append.len()]);
        if expected_crc != crc {
            return Err(ParseError::CrcCheckFailed {
                expected: expected_crc,
                actual: crc,
            });
        }
        // data_append.extend_from_slice(&crc.to_be_bytes());

        let mut frame_end = [0u8; 2];
        cursor
            .read_exact(&mut frame_end)
            .map_err(|e| ParseError::InvalidDataFormat {
                data_type: 0,
                reason: format!("Failed to read frame end: {}", e),
            })?;
        if frame_end != FRAME_END {
            return Err(ParseError::InvalidDataType {
                field: "Frame End".to_string(),
                value: 0,
            });
        }

        Ok(S909UP {
            frame_start,
            command_type,
            frame_length,
            device_id,
            protocol_version,
            power_status,
            signal_strength,
            channel_id,
            device_type,
            device_alarm_status: device_alarm_value,
            timestamp,
            channel_type,
            channel_data_length,
            channel_data,
            unit_length,
            unit_value,
            crc,
            frame_end,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut data_append = Vec::new();
        data_append.extend_from_slice(&self.frame_start);
        data_append.push(self.command_type);
        data_append.extend_from_slice(&self.frame_length.to_bytes());
        data_append.extend_from_slice(&self.device_id.to_bytes());
        data_append.push(self.protocol_version.to_u8());
        data_append.push(self.power_status.to_u8());
        data_append.push(self.signal_strength.to_byte());
        data_append.push(self.channel_id.to_u8());
        data_append.push(self.device_type.to_u8());
        data_append.push(self.device_alarm_status.to_u8());
        data_append.extend_from_slice(&self.timestamp.to_bytes());
        data_append.push(self.channel_type.to_u8());
        data_append.extend_from_slice(&self.channel_data_length.to_bytes());
        data_append.extend_from_slice(&self.channel_data.to_bytes());
        data_append.push(self.unit_length);
        data_append.extend_from_slice(&self.unit_value.to_bytes());
        data_append.extend_from_slice(&self.crc.to_be_bytes());
        data_append.extend_from_slice(&self.frame_end);

        data_append
    }
}
