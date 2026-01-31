use super::error::{ParseError, ParseResult};
use crate::Codec;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read};
use std::net::Ipv4Addr;
use std::time::Duration;

/// 参数设置数据单元字节数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamSetUnitSize(u16);

impl ParamSetUnitSize {
    pub fn new(size: u16) -> ParamSetUnitSize {
        ParamSetUnitSize(size)
    }

    pub fn get_size(&self) -> u16 {
        self.0
    }

    fn from_bytes(bytes: &[u8]) -> ParseResult<Self> {
        if bytes.len() < 2 {
            Err(ParseError::InsufficientData {
                field: "ParamSetUnitSize".to_string(),
                expected: 2,
                actual: bytes.len(),
            })
        } else {
            Ok(ParamSetUnitSize::new(u16::from_be_bytes([
                bytes[0], bytes[1],
            ])))
        }
    }

    fn to_bytes(&self) -> [u8; 2] {
        self.get_size().to_be_bytes()
    }

    pub fn from_cursor_bytes(cursor: &mut Cursor<&[u8]>) -> ParseResult<Self> {
        cursor
            .read_u16::<BigEndian>()
            .map(ParamSetUnitSize::new)
            .map_err(|e| ParseError::InvalidDataFormat {
                data_type: 0,
                reason: format!("Failded to read parameter setting unit size: {}", e),
            })
    }
}

impl Codec<2> for ParamSetUnitSize {
    fn from_bytes(bytes: &[u8]) -> ParseResult<Self>
    where
        Self: Sized,
    {
        ParamSetUnitSize::from_bytes(bytes)
    }

    fn to_bytes_const(&self) -> [u8; 2] {
        ParamSetUnitSize::to_bytes(self)
    }
}

/// 参数集合
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, std::default::Default)]
pub struct ParameterSet {
    pub parameters: Vec<SettingParameter>,
}

impl ParameterSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_parameter(&mut self, parameter: SettingParameter) {
        self.parameters.push(parameter);
    }

    pub fn get_parameter_len(&self) -> usize {
        self.parameters.len()
    }

    pub fn get_parameter(&self, index: usize) -> Option<&SettingParameter> {
        self.parameters.get(index)
    }

    pub fn from_bytes(bytes: &[u8], data_len: usize) -> ParseResult<Self> {
        if bytes.len() < data_len {
            return Err(ParseError::InsufficientData {
                field: "ParameterSet".to_string(),
                expected: data_len,
                actual: bytes.len(),
            });
        }

        let mut cursor = Cursor::new(bytes);
        let mut parameters = Vec::new();

        while (cursor.position() as usize) < data_len {
            if bytes.len() - (cursor.position() as usize) < 2 {
                return Err(ParseError::InsufficientData {
                    field: "Parameter header".to_string(),
                    expected: 2,
                    actual: data_len - cursor.position() as usize,
                });
            }

            let parameter_type = cursor.read_u8()?;
            let param_len = cursor.read_u8()?;

            let param_end = cursor.position() as usize + param_len as usize;
            if param_end > data_len {
                return Err(ParseError::InsufficientData {
                    field: "Parameter data".to_string(),
                    expected: param_len as usize,
                    actual: data_len - cursor.position() as usize,
                });
            }

            let parameter = SettingParameter::decode(
                (parameter_type, param_len),
                &bytes[cursor.position() as usize..],
            )?;
            parameters.push(parameter);

            cursor.set_position(param_end as u64);
        }

        if cursor.position() as usize != data_len {
            return Err(ParseError::InsufficientData {
                field: "ParameterSet".to_string(),
                expected: data_len,
                actual: cursor.position() as usize,
            });
        }

        Ok(Self { parameters })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        if self.parameters.is_empty() {
            return vec![];
        }

        self.parameters
            .iter()
            .flat_map(|param| {
                let mut data = vec![param.get_type(), param.get_value_len() as u8];
                let param_bytes = param.encode_value();
                if !param_bytes.is_empty() {
                    data.extend(param_bytes);
                }
                data
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SettingParameter {
    /// 清零
    /// 0 表示清零，>0 表示设置清零参数值
    Clear(u32),

    /// 设置高报警值
    HighAlarm(u32),

    /// 设置低报警值
    LowAlarm(u32),

    /// 设置数据记录间隔
    /// 范围：1s~9999s
    RecordInterval(Duration),

    /// 设置上发间隔
    /// 范围：3min~9999min
    UploadInterval(Duration),

    /// 设置 IP 地址
    IpAddress(Ipv4Addr),

    /// 设置端口号
    Port(u16),

    /// 设置采集间隔
    SamplingInterval(Duration),

    /// 设置波动报警值
    WaveAlarm(u32),

    /// 设置第二传感器的高报警值
    HighAlarmTwo(u32),

    /// 设置第二传感器的低报警值
    LowAlarmTwo(u32),

    /// 设置第二传感器的波动报警值
    WaveAlarmTwo(u32),

    /// 设置第二传感器的采集间隔
    SamplingIntervalTwo(Duration),

    /// 复位
    /// 读取次参数返回复位次数；写入此参数将复位设备
    Reset(u32),

    /// 客户自定义字符串
    CustomString(String),

    /// DO 控制
    ///
    /// 32 路 DO 控制，bit1 = 1 代表 DO1 开启，bit1 = 0 代表 D01 关闭，以此类推
    DoControl(u32),

    /// 电接点设置
    /// 高 16 位：控制时间（0=常闭，>0=闭合后延时断开时间）
    /// 低 16 位：输出控制位掩码
    ContactPoint(ContactPoint),

    /// 未知参数类型（保留原始数据）
    Unknown { type_id: u8, data: Vec<u8> },
}

impl SettingParameter {
    pub fn get_type(&self) -> u8 {
        match self {
            SettingParameter::Clear(_) => 1,
            SettingParameter::HighAlarm(_) => 2,
            SettingParameter::LowAlarm(_) => 3,
            SettingParameter::RecordInterval(_) => 4,
            SettingParameter::UploadInterval(_) => 5,
            SettingParameter::IpAddress(_) => 11,
            SettingParameter::Port(_) => 12,
            SettingParameter::SamplingInterval(_) => 13,
            SettingParameter::WaveAlarm(_) => 14,
            SettingParameter::HighAlarmTwo(_) => 15,
            SettingParameter::LowAlarmTwo(_) => 16,
            SettingParameter::WaveAlarmTwo(_) => 17,
            SettingParameter::SamplingIntervalTwo(_) => 18,
            SettingParameter::Reset(_) => 19,
            SettingParameter::CustomString(_) => 20,
            SettingParameter::DoControl(_) => 0x21,
            SettingParameter::ContactPoint(_) => 33,
            SettingParameter::Unknown { type_id, .. } => *type_id,
        }
    }

    pub fn get_value_len(&self) -> usize {
        match self {
            SettingParameter::Clear(_) => 4,
            SettingParameter::HighAlarm(_) => 4,
            SettingParameter::LowAlarm(_) => 4,
            SettingParameter::RecordInterval(_) => 4,
            SettingParameter::UploadInterval(_) => 4,
            SettingParameter::IpAddress(_) => 4,
            SettingParameter::Port(_) => 2,
            SettingParameter::SamplingInterval(_) => 4,
            SettingParameter::WaveAlarm(_) => 4,
            SettingParameter::HighAlarmTwo(_) => 4,
            SettingParameter::LowAlarmTwo(_) => 4,
            SettingParameter::WaveAlarmTwo(_) => 4,
            SettingParameter::SamplingIntervalTwo(_) => 4,
            SettingParameter::Reset(_) => 4,
            SettingParameter::CustomString(str) => str.len(),
            SettingParameter::ContactPoint(_) => 4,
            SettingParameter::DoControl(_) => 4,
            SettingParameter::Unknown { data, .. } => data.len(),
        }
    }

    /// 返回完整的参数编码（包括类型、长度和数据）
    pub fn encode_full(&self) -> Vec<u8> {
        let mut buf = vec![self.get_type(), self.get_value_len() as u8];
        buf.extend(self.encode_value());
        buf
    }

    pub fn encode_value(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.get_value_len());

        match self {
            SettingParameter::Clear(v)
            | SettingParameter::LowAlarm(v)
            | SettingParameter::HighAlarm(v)
            | SettingParameter::HighAlarmTwo(v)
            | SettingParameter::LowAlarmTwo(v)
            | SettingParameter::WaveAlarm(v)
            | SettingParameter::WaveAlarmTwo(v)
            | SettingParameter::DoControl(v)
            | SettingParameter::Reset(v) => {
                let _ = buf.write_u32::<BigEndian>(*v);
            }
            SettingParameter::RecordInterval(d)
            | SettingParameter::SamplingInterval(d)
            | SettingParameter::SamplingIntervalTwo(d) => {
                let secs = d.as_secs() as u32;
                let _ = buf.write_u32::<BigEndian>(secs);
            }
            SettingParameter::UploadInterval(d) => {
                let mins = (d.as_secs() / 60) as u32;
                let _ = buf.write_u32::<BigEndian>(mins);
            }
            SettingParameter::IpAddress(ip) => {
                buf.extend_from_slice(&ip.octets());
            }
            SettingParameter::Port(p) => {
                let _ = buf.write_u16::<BigEndian>(*p);
            }
            SettingParameter::CustomString(str) => {
                buf.extend_from_slice(str.as_bytes());
            }
            SettingParameter::ContactPoint(cp) => {
                let _ = buf.extend_from_slice(&cp.to_bytes_const());
            }
            SettingParameter::Unknown { data, .. } => {
                buf.extend_from_slice(data);
            }
        }
        buf
    }

    pub fn decode((type_id, data_len): (u8, u8), data: &[u8]) -> ParseResult<Self> {
        if data.len() < data_len as usize {
            return Err(ParseError::InsufficientData {
                field: "Parameter".to_string(),
                expected: data_len as usize,
                actual: data.len(),
            });
        }
        let mut cursor = Cursor::new(data);
        match type_id {
            1 => Ok(SettingParameter::Clear(cursor.read_u32::<BigEndian>()?)),
            2 => Ok(SettingParameter::HighAlarm(cursor.read_u32::<BigEndian>()?)),
            3 => Ok(SettingParameter::LowAlarm(cursor.read_u32::<BigEndian>()?)),
            4 => Ok(SettingParameter::RecordInterval(Duration::from_secs(
                cursor.read_u32::<BigEndian>()? as u64,
            ))),
            5 => Ok(SettingParameter::UploadInterval(Duration::from_secs(
                cursor.read_u32::<BigEndian>()? as u64 * 60,
            ))),
            11 => {
                let mut octets = [0u8; 4];
                cursor.read_exact(&mut octets)?;
                Ok(SettingParameter::IpAddress(Ipv4Addr::from(octets)))
            }
            12 => Ok(SettingParameter::Port(cursor.read_u16::<BigEndian>()?)),
            13 => Ok(SettingParameter::SamplingInterval(Duration::from_secs(
                cursor.read_u32::<BigEndian>()? as u64,
            ))),
            14 => Ok(SettingParameter::WaveAlarm(cursor.read_u32::<BigEndian>()?)),
            15 => Ok(SettingParameter::HighAlarmTwo(
                cursor.read_u32::<BigEndian>()?,
            )),
            16 => Ok(SettingParameter::LowAlarmTwo(
                cursor.read_u32::<BigEndian>()?,
            )),
            17 => Ok(SettingParameter::WaveAlarmTwo(
                cursor.read_u32::<BigEndian>()?,
            )),
            18 => Ok(SettingParameter::SamplingIntervalTwo(Duration::from_secs(
                cursor.read_u32::<BigEndian>()? as u64,
            ))),
            19 => Ok(SettingParameter::Reset(cursor.read_u32::<BigEndian>()?)),
            20 => Ok(SettingParameter::CustomString(
                String::from_utf8_lossy(&data[..(data_len as usize)]).to_string(),
            )),
            33 => Ok(SettingParameter::ContactPoint(<ContactPoint as Codec<
                4,
            >>::from_bytes(
                data
            )?)),
            _ => Ok(SettingParameter::Unknown {
                type_id,
                data: data[..(data_len as usize)].to_vec(),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContactPoint {
    /// 脉冲模式持续时间。如果为 None，表示常闭（或手动控制）
    pub pulse_duration: Option<Duration>,
    /// 输出控制位掩码（1 = 闭合，0 = 断开）
    pub output_mask: u16,
}

impl ContactPoint {
    pub fn new(mask: u16) -> Self {
        ContactPoint {
            pulse_duration: None,
            output_mask: mask,
        }
    }

    pub fn with_pulse_duration(mask: u16, duration: Duration) -> Self {
        if duration.is_zero() {
            ContactPoint {
                pulse_duration: None,
                output_mask: mask,
            }
        } else {
            ContactPoint {
                pulse_duration: Some(duration),
                output_mask: mask,
            }
        }
    }

    fn from_bytes(bytes: &[u8]) -> ParseResult<Self> {
        if bytes.len() < 4 {
            return Err(ParseError::InsufficientData {
                field: "ContactPoint".to_string(),
                expected: 4,
                actual: bytes.len(),
            });
        }

        let high = u16::from_be_bytes([bytes[0], bytes[1]]);
        let low = u16::from_be_bytes([bytes[2], bytes[3]]);

        let pulse_duration = (high != 0).then(|| Duration::from_secs(high as u64));

        Ok(ContactPoint {
            pulse_duration,
            output_mask: low,
        })
    }

    fn to_bytes(&self) -> [u8; 4] {
        let pulse = self
            .pulse_duration
            .map(|d| d.as_secs() as u16)
            .unwrap_or(0)
            .to_be_bytes();

        let mask = self.output_mask.to_be_bytes();

        [pulse[0], pulse[1], mask[0], mask[1]]
    }

    pub fn reset_pulse_duration(&mut self, duration: Duration) {
        self.pulse_duration = (!duration.is_zero()).then_some(duration)
    }

    pub fn set_channel(&mut self, index: u8, enabled: bool) {
        assert!(index < 16, "Channel index out of range");
        let bit = 1u16 << index;
        if enabled {
            self.output_mask |= bit;
        } else {
            self.output_mask &= !bit;
        }
    }

    pub fn is_channel_enabled(&self, index: u8) -> bool {
        assert!(index < 16, "Channel index out of range");
        (self.output_mask & (1u16 << index)) != 0
    }

    pub fn enabled_channels(&self) -> impl Iterator<Item = u8> + '_ {
        (0..16).filter(|&i| self.is_channel_enabled(i))
    }
}

impl Codec<4> for ContactPoint {
    fn to_bytes_const(&self) -> [u8; 4] {
        ContactPoint::to_bytes(self)
    }

    fn from_bytes(bytes: &[u8]) -> ParseResult<Self>
    where
        Self: Sized,
    {
        ContactPoint::from_bytes(bytes)
    }
}

/// 流式 API 辅助方法
impl ParameterSet {
    pub fn add_clear(mut self, value: u32) -> Self {
        self.add_parameter(SettingParameter::Clear(value));
        self
    }

    pub fn add_high_alarm_1(mut self, value: u32) -> Self {
        self.add_parameter(SettingParameter::HighAlarm(value));
        self
    }

    pub fn add_high_alarm_2(mut self, value: u32) -> Self {
        self.add_parameter(SettingParameter::HighAlarmTwo(value));
        self
    }

    pub fn add_low_alarm_1(mut self, value: u32) -> Self {
        self.add_parameter(SettingParameter::LowAlarm(value));
        self
    }

    pub fn add_record_interval(mut self, duration: Duration) -> Self {
        self.add_parameter(SettingParameter::RecordInterval(duration));
        self
    }

    pub fn add_low_alarm_2(mut self, value: u32) -> Self {
        self.add_parameter(SettingParameter::LowAlarmTwo(value));
        self
    }

    pub fn add_upload_interval(mut self, duration: Duration) -> Self {
        self.add_parameter(SettingParameter::UploadInterval(duration));
        self
    }

    pub fn add_ip(mut self, ip: Ipv4Addr) -> Self {
        self.add_parameter(SettingParameter::IpAddress(ip));
        self
    }

    pub fn add_port(mut self, port: u16) -> Self {
        self.add_parameter(SettingParameter::Port(port));
        self
    }

    pub fn add_sampling_interval_1(mut self, duration: Duration) -> Self {
        self.add_parameter(SettingParameter::SamplingInterval(duration));
        self
    }

    pub fn add_sampling_interval_2(mut self, duration: Duration) -> Self {
        self.add_parameter(SettingParameter::SamplingIntervalTwo(duration));
        self
    }

    pub fn add_contact_point(mut self, mask: u16, pulse: Option<Duration>) -> Self {
        self.add_parameter(SettingParameter::ContactPoint(ContactPoint {
            pulse_duration: pulse,
            output_mask: mask,
        }));
        self
    }

    pub fn add_wave_alarm_1(mut self, value: u32) -> Self {
        self.add_parameter(SettingParameter::WaveAlarm(value));
        self
    }

    pub fn add_wave_alarm_2(mut self, value: u32) -> Self {
        self.add_parameter(SettingParameter::WaveAlarmTwo(value));
        self
    }
}
