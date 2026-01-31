use super::frame::error::{ParseError, ParseResult};
use crate::Codec;
use crate::constants::{FRAME_END, FRAME_START, command::UPLOAD_DATA_DEVICE};
use crate::frame::{
    AlarmParameters, AlarmStatus, DeviceData, DeviceDataParser, DeviceId, DeviceType, FrameLength,
    ModbusData, PowerStatus, SignalStrength, Timestamp,
};
use crate::utils::crc16;
use byteorder::{BigEndian, ReadBytesExt};
use either::Either;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read};

/// 数据记录间隔
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct DataRecordInterval(u16);

impl DataRecordInterval {
    pub fn new(interval: u16) -> Self {
        DataRecordInterval(interval)
    }

    pub fn get_interval(&self) -> u16 {
        self.0
    }

    fn from_bytes(bytes: &[u8]) -> ParseResult<Self> {
        if bytes.len() < 2 {
            Err(ParseError::InsufficientData {
                field: "DataRecordInterval".to_string(),
                expected: 2,
                actual: bytes.len(),
            })
        } else {
            Ok(DataRecordInterval(u16::from_be_bytes([bytes[0], bytes[1]])))
        }
    }
}

impl Codec<2> for DataRecordInterval {
    fn from_bytes(bytes: &[u8]) -> ParseResult<Self> {
        Self::from_bytes(bytes)
    }

    fn to_bytes_const(&self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
}

/// 数据记录条数
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct DataRecordCount(u16);

impl DataRecordCount {
    pub fn new(count: u16) -> Self {
        DataRecordCount(count)
    }

    pub fn get_count(&self) -> u16 {
        self.0
    }

    fn from_bytes(bytes: &[u8]) -> ParseResult<Self> {
        if bytes.len() < 2 {
            Err(ParseError::InsufficientData {
                field: "DataRecordCount".to_string(),
                expected: 2,
                actual: bytes.len(),
            })
        } else {
            Ok(DataRecordCount(u16::from_be_bytes([bytes[0], bytes[1]])))
        }
    }
}

impl Codec<2> for DataRecordCount {
    fn from_bytes(bytes: &[u8]) -> ParseResult<Self> {
        Self::from_bytes(bytes)
    }

    fn to_bytes_const(&self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DevFrame {
    pub frame_start: [u8; 2],
    pub command_type: u8,
    pub frame_length: FrameLength,
    pub device_id: DeviceId,
    pub power_status: PowerStatus,
    pub signal_strength: SignalStrength,
    pub data_record_interval: DataRecordInterval,
    pub data_record_count: DataRecordCount,
    pub device_type: DeviceType,
    pub alarm_status: AlarmStatus,
    pub alarm_parameters: AlarmParameters,
    pub current_device_data: Either<DeviceData, ModbusData>,
    pub timestamp: Timestamp,
    pub cache_device_data: Vec<Either<DeviceData, ModbusData>>,
    pub cache_timestamp: Vec<Timestamp>,
    pub crc: u16,
    pub frame_end: [u8; 2],
}

impl DevFrame {
    pub fn from_bytes(bytes: &[u8]) -> ParseResult<Self> {
        // TODO: 当前阶段使用 data_remaining 验证编码正确性
        let bytes_len = bytes.len();
        if bytes_len < 33 {
            return Err(ParseError::InsufficientData {
                field: "上发数据帧".to_string(),
                expected: 33,
                actual: bytes.len(),
            });
        }

        let mut data_append = Vec::<u8>::with_capacity(35);
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
        if command_type != UPLOAD_DATA_DEVICE {
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

        let frame_length = FrameLength::from_slices(&frame_length);

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

        let device_id = DeviceId::from_slices(&device_id);
        data_append.extend_from_slice(&device_id.to_bytes());

        let power_status =
            PowerStatus::from_u8(
                cursor
                    .read_u8()
                    .map_err(|e| ParseError::InvalidDataFormat {
                        data_type: 0,
                        reason: format!("Faild to read power status: {}", e),
                    })?,
            );
        data_append.push(power_status.to_u8());

        let signal_strength = SignalStrength::from_u8(cursor.read_u8().map_err(|e| {
            ParseError::InvalidDataFormat {
                data_type: 0,
                reason: format!("Faild to read signal strength: {}", e),
            }
        })?);
        data_append.push(signal_strength.to_byte());

        let data_record_interval =
            DataRecordInterval::new(cursor.read_u16::<BigEndian>().map_err(|e| {
                ParseError::InvalidDataFormat {
                    data_type: 0,
                    reason: format!("Failed to read data record interval: {}", e),
                }
            })?);
        data_append.extend_from_slice(&data_record_interval.to_bytes_const());

        let data_record_count =
            DataRecordCount::new(cursor.read_u16::<BigEndian>().map_err(|e| {
                ParseError::InvalidDataFormat {
                    data_type: 0,
                    reason: format!("Failed to read data record count: {}", e),
                }
            })?);
        data_append.extend_from_slice(&data_record_count.to_bytes_const());

        let device_type =
            DeviceType::from_u8(
                cursor
                    .read_u8()
                    .map_err(|e| ParseError::InvalidDataFormat {
                        data_type: 0,
                        reason: format!("Failed to read device type: {}", e),
                    })?,
            )?;
        data_append.push(device_type.to_u8());

        let alarm_status =
            AlarmStatus::from_u8(
                cursor
                    .read_u8()
                    .map_err(|e| ParseError::InvalidDataFormat {
                        data_type: 0,
                        reason: format!("Faild to read device alarm value: {}", e),
                    })?,
            );
        data_append.push(alarm_status.to_u8());

        let mut alarm_parameters = AlarmParameters::from_device_type(device_type)?;
        let mut alarm_parameters_buf = vec![0u8; alarm_parameters.get_data_len()];
        cursor.read_exact(&mut alarm_parameters_buf).map_err(|e| {
            ParseError::InvalidDataFormat {
                data_type: 0,
                reason: format!("Failed to read alarm parameters: {}", e),
            }
        })?;
        alarm_parameters.init_with_data(&alarm_parameters_buf)?;
        data_append.extend_from_slice(&alarm_parameters.to_bytes());

        let current_device_data =
            Self::get_device_data(&mut cursor, device_type, &mut data_append)?;

        let mut timestamp = [0u8; 4];
        cursor
            .read_exact(&mut timestamp)
            .map_err(|e| ParseError::InvalidDataFormat {
                data_type: 0,
                reason: format!("Failed to read timestamp: {}", e),
            })?;
        let timestamp = Timestamp::from_bytes(timestamp);
        data_append.extend_from_slice(&timestamp.to_bytes());

        let remaining_data_length = frame_length.len() - cursor.position() as u16 - 4;
        let mut cache_device_data = Vec::<Either<DeviceData, ModbusData>>::new();

        match &current_device_data {
            Either::Left(device_data) => {
                let signal_device_data_length = device_data.get_data_length();
                if remaining_data_length % signal_device_data_length as u16 != 0 {
                    return Err(ParseError::InvalidDataFormat {
                        data_type: 0,
                        reason: format!(
                            "Device Data Length Miss Match {} - {}",
                            remaining_data_length, signal_device_data_length
                        ),
                    });
                }

                (0..(remaining_data_length / signal_device_data_length as u16)).try_for_each(
                    |_| -> ParseResult<()> {
                        cache_device_data.push(Self::get_device_data(
                            &mut cursor,
                            device_type,
                            &mut data_append,
                        )?);

                        Ok(())
                    },
                )?;
            }
            Either::Right(_) => {
                // 边界条件：正常应该是 cursor.position() == remaining_data_length + 第一次循环cursor.position()
                // 如果不是，说明数据格式错误
                let expected_position = remaining_data_length as u64 + cursor.position();
                while cursor.position() < expected_position {
                    cache_device_data.push(Self::get_device_data(
                        &mut cursor,
                        device_type,
                        &mut data_append,
                    )?);
                }
                // 检查边界条件
                if expected_position != cursor.position()
                    || cache_device_data.len() + 1 != data_record_count.get_count() as usize
                {
                    return Err(ParseError::InvalidDataFormat {
                        data_type: 0,
                        reason: format!(
                            "Modbus Device Data Length Miss Match {} - {}",
                            cursor.position(),
                            expected_position
                        ),
                    });
                }
            }
        }

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

        let cache_timestamp = Self::match_timestamps(
            timestamp,
            data_record_interval.get_interval(),
            data_record_count.get_count(),
        );

        Ok(DevFrame {
            frame_start,
            command_type,
            frame_length,
            device_id,
            power_status,
            signal_strength,
            data_record_interval,
            data_record_count,
            device_type,
            alarm_status,
            alarm_parameters,
            current_device_data,
            timestamp,
            cache_device_data,
            cache_timestamp,
            crc,
            frame_end,
        })
    }

    fn match_timestamps(
        from: Timestamp,
        record_interval: u16,
        record_count: u16,
    ) -> Vec<Timestamp> {
        let mut timestamps = Vec::with_capacity(record_count as usize - 1);
        let mut timestamp = from;
        for _ in 0..(record_count - 1) {
            timestamp = Timestamp::new(timestamp.seconds - record_interval as u32);
            timestamps.push(timestamp);
        }
        timestamps.reverse();

        timestamps
    }

    fn get_device_data(
        cursor: &mut Cursor<&[u8]>,
        device_type: DeviceType,
        data_append: &mut Vec<u8>,
    ) -> ParseResult<Either<DeviceData, ModbusData>> {
        let current_device_data = DeviceDataParser::parse_with_type_id(
            device_type.to_u8(),
            &cursor.get_ref()[cursor.position() as usize..],
        )?;

        match &current_device_data {
            Either::Left(device_data) => {
                let data_length = device_data.get_data_length();
                cursor.set_position(cursor.position() + data_length as u64);
                data_append.extend_from_slice(&device_data.to_bytes());
            }
            Either::Right(modbus_data) => {
                let data_length = modbus_data.data_length();
                cursor.set_position(cursor.position() + data_length as u64);
                data_append.extend_from_slice(&modbus_data.to_bytes());
            }
        };

        Ok(current_device_data)
    }
}
