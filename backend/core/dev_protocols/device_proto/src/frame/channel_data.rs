use crate::{frame::ChannelType, utils::ChannelList16};
use serde::{Deserialize, Serialize};

/// 通道数据类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChannelDataType {
    /// 开关量输入
    DIInput(DIInputData),
    /// 模拟量输入
    AIInput(AIInputData),
    /// 开关量输出
    DOOutput(DOOutputData),
    /// RS485端口、RS232端口
    RSPort(RSPortData),
    // TODO: 暂未实现
    // 这个字段目前用来 RS485 透传
    Unknown(Vec<u8>),
}

#[allow(non_snake_case)]
impl ChannelDataType {
    pub fn from_channel_type(channel_type: &ChannelType) -> Self {
        if channel_type.is_analog_sensor() {
            ChannelDataType::AIInput(AIInputData::new())
        } else if channel_type.is_distance_input() {
            ChannelDataType::DIInput(DIInputData::new())
        } else if channel_type.is_digital_output() {
            ChannelDataType::DOOutput(DOOutputData::new())
        } else {
            ChannelDataType::RSPort(RSPortData::new())
        }
    }

    pub fn as_di_input(&self) -> Option<&DIInputData> {
        match self {
            ChannelDataType::DIInput(data) => Some(data),
            _ => None,
        }
    }

    pub fn as_di_input_mut(&mut self) -> Option<&mut DIInputData> {
        match self {
            ChannelDataType::DIInput(data) => Some(data),
            _ => None,
        }
    }

    pub fn as_ai_input(&self) -> Option<&AIInputData> {
        match self {
            ChannelDataType::AIInput(data) => Some(data),
            _ => None,
        }
    }

    pub fn as_ai_input_mut(&mut self) -> Option<&mut AIInputData> {
        match self {
            ChannelDataType::AIInput(data) => Some(data),
            _ => None,
        }
    }

    pub fn as_do_output(&self) -> Option<&DOOutputData> {
        match self {
            ChannelDataType::DOOutput(data) => Some(data),
            _ => None,
        }
    }

    pub fn as_do_output_mut(&mut self) -> Option<&mut DOOutputData> {
        match self {
            ChannelDataType::DOOutput(data) => Some(data),
            _ => None,
        }
    }

    pub fn as_rs_port(&self) -> Option<&RSPortData> {
        match self {
            ChannelDataType::RSPort(data) => Some(data),
            _ => None,
        }
    }

    pub fn as_rs_port_mut(&mut self) -> Option<&mut RSPortData> {
        match self {
            ChannelDataType::RSPort(data) => Some(data),
            _ => None,
        }
    }

    /// 获取原始字节数据（对于 Unknown 类型）
    pub fn as_unknown_bytes(&self) -> Option<&[u8]> {
        match self {
            ChannelDataType::Unknown(data) => Some(data),
            _ => None,
        }
    }

    pub fn is_di_input(&self) -> bool {
        matches!(self, ChannelDataType::DIInput(_))
    }

    pub fn is_ai_input(&self) -> bool {
        matches!(self, ChannelDataType::AIInput(_))
    }

    pub fn is_do_output(&self) -> bool {
        matches!(self, ChannelDataType::DOOutput(_))
    }

    pub fn is_rs_port(&self) -> bool {
        matches!(self, ChannelDataType::RSPort(_))
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, ChannelDataType::Unknown(_))
    }

    /// 统一访问器：获取模拟量值（仅对 AIInput 和 RSPort 有效）
    pub fn get_analog_value(&self) -> Option<f32> {
        match self {
            ChannelDataType::AIInput(data) => Some(data.get_analog_value()),
            ChannelDataType::RSPort(data) => Some(data.get_analog_value()),
            _ => None,
        }
    }

    /// 统一访问器：设置模拟量值（仅对 AIInput 和 RSPort 有效）
    pub fn set_analog_value(&mut self, value: f32) {
        match self {
            ChannelDataType::AIInput(data) => data.set_analog_value(value),
            ChannelDataType::RSPort(data) => data.set_analog_value(value),
            _ => {} // 静默忽略无效操作
        }
    }

    /// 统一访问器：获取指定索引的通道状态
    pub fn get_channel_state(&self, index: usize) -> Option<bool> {
        match self {
            ChannelDataType::DIInput(data) => Some(data.get_channel_state(index)),
            ChannelDataType::AIInput(data) => Some(data.get_channel_state(index)),
            ChannelDataType::DOOutput(data) => Some(data.get_channel_state(index)),
            ChannelDataType::RSPort(data) => Some(data.get_channel_state(index)),
            ChannelDataType::Unknown(_) => None,
        }
    }

    /// 统一访问器：设置指定索引的通道状态
    pub fn set_channel_state(&mut self, index: usize, is_open: bool) {
        match self {
            ChannelDataType::DIInput(data) => data.set_channel_state(index, is_open),
            ChannelDataType::AIInput(data) => data.set_channel_state(index, is_open),
            ChannelDataType::DOOutput(data) => data.set_channel_state(index, is_open),
            ChannelDataType::RSPort(data) => data.set_channel_state(index, is_open),
            ChannelDataType::Unknown(_) => {} // 静默忽略无效操作
        }
    }

    /// 统一访问器：获取通道状态的位掩码
    pub fn channels_mask(&self) -> Option<u16> {
        match self {
            ChannelDataType::DIInput(data) => Some(data.input_channels_mask()),
            ChannelDataType::AIInput(data) => Some(data.channels_mask()),
            ChannelDataType::DOOutput(data) => Some(data.output_channels_mask()),
            ChannelDataType::RSPort(data) => Some(data.channels_mask()),
            ChannelDataType::Unknown(_) => None,
        }
    }

    /// 使用字节流初始化数据
    pub fn init_by_bytes(mut self, bytes: &[u8]) -> Self {
        match self {
            ChannelDataType::DIInput(_) => {
                let temp_info = DIInputData::from_bytes(bytes);
                self = ChannelDataType::DIInput(temp_info);
            }
            ChannelDataType::AIInput(_) => {
                let temp_info = AIInputData::from_bytes(bytes);
                self = ChannelDataType::AIInput(temp_info);
            }
            ChannelDataType::DOOutput(_) => {
                let temp_info = DOOutputData::from_bytes(bytes);
                self = ChannelDataType::DOOutput(temp_info);
            }
            ChannelDataType::RSPort(_) => {
                let temp_info = RSPortData::from_bytes(bytes);
                self = ChannelDataType::RSPort(temp_info);
            }
            _ => {
                panic!("目前未实现!!!");
            }
        }
        self
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            ChannelDataType::AIInput(data) => data.to_bytes().to_vec(),
            ChannelDataType::DIInput(data) => data.to_bytes().to_vec(),
            ChannelDataType::DOOutput(data) => data.to_bytes().to_vec(),
            ChannelDataType::RSPort(data) => data.to_bytes().to_vec(),
            ChannelDataType::Unknown(data) => data.clone(),
        }
    }
}

/// 开关量输入数据类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DIInputData(ChannelList16, ChannelList16);

impl DIInputData {
    pub fn new() -> Self {
        Self(ChannelList16::new(), ChannelList16::new())
    }

    /// 获取指定端口的输入开关量状态
    pub fn get_input_channel_state(&self, index: usize) -> bool {
        assert!(index < 16, "Index out of bounds for DIInputData");

        self.0[index]
    }

    pub fn get_output_channel_state(&self, index: usize) -> bool {
        assert!(index < 16, "Index out of bounds for DOOutputData");
        self.1[index]
    }

    /// 设置指定端口的输入开关量状态
    pub fn set_input_channel_state(&mut self, index: usize, is_open: bool) {
        assert!(index < 16, "Index out of bounds for DIInputData");

        self.0.set(index, is_open);
    }

    pub fn set_output_channel_state(&mut self, index: usize, is_open: bool) {
        assert!(index < 16, "Index out of bounds for DOOutputData");

        self.1.set(index, is_open);
    }

    /// 统一访问器：获取通道状态（对于 DI 数据，统一返回输入状态）
    pub fn get_channel_state(&self, index: usize) -> bool {
        self.get_input_channel_state(index)
    }

    /// 统一访问器：设置通道状态（对于 DI 数据，统一设置输入状态）
    pub fn set_channel_state(&mut self, index: usize, is_open: bool) {
        self.set_input_channel_state(index, is_open);
    }

    /// 获取所有输入通道状态的位掩码
    pub fn input_channels_mask(&self) -> u16 {
        self.0.to_u16()
    }

    /// 获取所有输出通道状态的位掩码
    pub fn output_channels_mask(&self) -> u16 {
        self.1.to_u16()
    }

    // TODO: 当前实现是：DI（高16位）+ DO（低16位）
    pub fn from_bytes(bytes: &[u8]) -> Self {
        // TODO: 设计缺陷
        // let mut output_value = 0u16;
        // let mut input_value = 0u16;
        // (0..4).for_each(|i| {
        //     (0..8).for_each(|j| {
        //         let bit_index = i * 8 + j;
        //         if bit_index < 16 {
        //             input_value |= (((input_bytes[i] >> j) & 0x01) as u16) << (bit_index % 16);
        //         } else {
        //             output_value |= (((input_bytes[i] >> j) & 0x01) as u16) << (bit_index % 16);
        //         }
        //     })
        // });

        // let input_value = u16::from_be_bytes([bytes[0], bytes[1]]);
        // let output_value = u16::from_be_bytes([bytes[2], bytes[3]]);

        // Self(
        //     ChannelList16::from_u16(input_value),
        //     ChannelList16::from_u16(output_value),
        // )

        assert!(
            bytes.len() >= 4,
            "Bytes length must be at least 4 for DIInputData"
        );
        Self(
            ChannelList16::from_bytes(&bytes[..2]),
            ChannelList16::from_bytes(&bytes[2..4]),
        )
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        let input_bytes = self.0.to_bytes();
        let output_bytes = self.1.to_bytes();

        let mut result = [0u8; 4];
        result[..2].copy_from_slice(&input_bytes);
        result[2..].copy_from_slice(&output_bytes);

        result
    }
}

/// 通道数据类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AIInputData(f32, ChannelList16);

#[allow(non_snake_case)]
impl AIInputData {
    pub fn new() -> Self {
        AIInputData(0.0, ChannelList16::new())
    }

    pub fn get_AI_value(&self) -> f32 {
        self.0
    }

    pub fn set_AI_value(&mut self, value: f32) {
        self.0 = value;
    }

    pub fn get_DO_channel_state(&self, index: usize) -> bool {
        assert!(index < 16, "Index out of bounds for AIInputData");

        self.1[index]
    }

    pub fn set_DO_channel_state(&mut self, index: usize, is_open: bool) {
        assert!(index < 16, "Index out of bounds for AIInputData");

        self.1.set(index, is_open);
    }

    /// 统一访问器：获取模拟量值
    pub fn get_analog_value(&self) -> f32 {
        self.get_AI_value()
    }

    /// 统一访问器：设置模拟量值
    pub fn set_analog_value(&mut self, value: f32) {
        self.set_AI_value(value);
    }

    /// 统一访问器：获取通道状态（对于 AI 数据，统一返回 DO 状态）
    pub fn get_channel_state(&self, index: usize) -> bool {
        self.get_DO_channel_state(index)
    }

    /// 统一访问器：设置通道状态（对于 AI 数据，统一设置 DO 状态）
    pub fn set_channel_state(&mut self, index: usize, is_open: bool) {
        self.set_DO_channel_state(index, is_open);
    }

    /// 获取所有 DO 通道状态的位掩码
    pub fn channels_mask(&self) -> u16 {
        self.1.to_u16()
    }

    pub fn set_AI_value_from_bytes(&mut self, bytes: &[u8]) {
        assert!(
            bytes.len() >= 4,
            "Byte slice too short to extract f32 value"
        );
        self.0 = f32::from_be_bytes(bytes[..4].try_into().unwrap());
    }

    pub fn get_AI_value_as_bytes(&self) -> [u8; 4] {
        self.0.to_be_bytes()
    }

    pub fn set_DO_channel_state_from_bytes(&mut self, bytes: &[u8]) {
        self.1 = ChannelList16::from_bytes(&bytes[..2]);
    }

    pub fn get_DO_channel_state_as_bytes(&self) -> [u8; 2] {
        self.1.to_bytes()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert!(
            bytes.len() >= 6,
            "Bytes length must be at least 6 for AIInputData"
        );

        let mut result = Self::new();
        result.set_AI_value_from_bytes(&bytes[..4]);
        result.set_DO_channel_state_from_bytes(&bytes[4..]);

        result
    }

    pub fn to_bytes(&self) -> [u8; 6] {
        let mut result = [0u8; 6];
        result[..4].copy_from_slice(&self.get_AI_value_as_bytes());
        result[4..].copy_from_slice(&self.get_DO_channel_state_as_bytes());

        result
    }
}

/// 开关量输出数据类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DOOutputData(DIInputData);

impl DOOutputData {
    pub fn new() -> Self {
        Self(DIInputData::new())
    }

    /// 获取指定端口的输入开关量状态
    pub fn get_input_channel_state(&self, index: usize) -> bool {
        self.0.get_input_channel_state(index)
    }

    pub fn get_output_channel_state(&self, index: usize) -> bool {
        self.0.get_output_channel_state(index)
    }

    /// 设置指定端口的输入开关量状态
    pub fn set_input_channel_state(&mut self, index: usize, is_open: bool) {
        self.0.set_input_channel_state(index, is_open);
    }

    pub fn set_output_channel_state(&mut self, index: usize, is_open: bool) {
        self.0.set_output_channel_state(index, is_open);
    }

    /// 统一访问器：获取通道状态（对于 DO 数据，统一返回输出状态）
    pub fn get_channel_state(&self, index: usize) -> bool {
        self.get_output_channel_state(index)
    }

    /// 统一访问器：设置通道状态（对于 DO 数据，统一设置输出状态）
    pub fn set_channel_state(&mut self, index: usize, is_open: bool) {
        self.set_output_channel_state(index, is_open);
    }

    /// 获取所有输入通道状态的位掩码
    pub fn input_channels_mask(&self) -> u16 {
        self.0.input_channels_mask()
    }

    /// 获取所有输出通道状态的位掩码
    pub fn output_channels_mask(&self) -> u16 {
        self.0.output_channels_mask()
    }

    // TODO: 当前实现是：DI（高16位）+ DO（低16位）
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(DIInputData::from_bytes(bytes))
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        self.0.to_bytes()
    }
}

/// RS* 端口数据类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RSPortData(AIInputData);

#[allow(non_snake_case)]
impl RSPortData {
    pub fn new() -> Self {
        Self(AIInputData(0.0, ChannelList16::new()))
    }

    pub fn get_AI_value(&self) -> f32 {
        self.0.0
    }

    pub fn set_AI_value(&mut self, value: f32) {
        self.0.0 = value;
    }

    pub fn get_DO_channel_state(&self, index: usize) -> bool {
        self.0.1[index]
    }

    pub fn set_DO_channel_state(&mut self, index: usize, is_open: bool) {
        self.0.1.set(index, is_open);
    }

    /// 统一访问器：获取模拟量值
    pub fn get_analog_value(&self) -> f32 {
        self.get_AI_value()
    }

    /// 统一访问器：设置模拟量值
    pub fn set_analog_value(&mut self, value: f32) {
        self.set_AI_value(value);
    }

    /// 统一访问器：获取通道状态
    pub fn get_channel_state(&self, index: usize) -> bool {
        self.get_DO_channel_state(index)
    }

    /// 统一访问器：设置通道状态
    pub fn set_channel_state(&mut self, index: usize, is_open: bool) {
        self.set_DO_channel_state(index, is_open);
    }

    /// 获取所有通道状态的位掩码
    pub fn channels_mask(&self) -> u16 {
        self.0.1.to_u16()
    }

    pub fn set_AI_value_from_bytes(&mut self, bytes: &[u8]) {
        self.0.0 = f32::from_be_bytes(bytes[..4].try_into().unwrap());
    }

    pub fn get_AI_value_as_bytes(&self) -> [u8; 4] {
        self.0.0.to_be_bytes()
    }

    pub fn set_DO_channel_state_from_bytes(&mut self, bytes: &[u8]) {
        self.0.1 = ChannelList16::from_bytes(&bytes[..2]);
    }

    pub fn get_DO_channel_state_as_bytes(&self) -> [u8; 2] {
        self.0.1.to_bytes()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(AIInputData::from_bytes(bytes))
    }

    pub fn to_bytes(&self) -> [u8; 6] {
        self.0.to_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_data_type_accessors() {
        // 测试统一访问器API
        let mut di_data = ChannelDataType::DIInput(DIInputData::new());
        let mut ai_data = ChannelDataType::AIInput(AIInputData::new());
        let mut do_data = ChannelDataType::DOOutput(DOOutputData::new());
        let mut rs_data = ChannelDataType::RSPort(RSPortData::new());

        // 测试类型检查方法
        assert!(di_data.is_di_input());
        assert!(ai_data.is_ai_input());
        assert!(do_data.is_do_output());
        assert!(rs_data.is_rs_port());

        // 测试模拟量值访问器（只对 AIInput 和 RSPort 有效）
        assert_eq!(ai_data.get_analog_value(), Some(0.0));
        assert_eq!(rs_data.get_analog_value(), Some(0.0));
        assert_eq!(di_data.get_analog_value(), None);
        assert_eq!(do_data.get_analog_value(), None);

        // 设置模拟量值
        ai_data.set_analog_value(3.14);
        rs_data.set_analog_value(2.71);

        assert_eq!(ai_data.get_analog_value(), Some(3.14));
        assert_eq!(rs_data.get_analog_value(), Some(2.71));

        // 测试通道状态访问器
        di_data.set_channel_state(0, true);
        ai_data.set_channel_state(1, true);
        do_data.set_channel_state(2, true);
        rs_data.set_channel_state(3, true);

        assert_eq!(di_data.get_channel_state(0), Some(true));
        assert_eq!(ai_data.get_channel_state(1), Some(true));
        assert_eq!(do_data.get_channel_state(2), Some(true));
        assert_eq!(rs_data.get_channel_state(3), Some(true));

        // 测试通道掩码访问器
        assert_eq!(di_data.channels_mask(), Some(0b0000000000000001)); // DI 输入通道
        assert_eq!(ai_data.channels_mask(), Some(0b0000000000000010)); // AI DO 通道
        assert_eq!(do_data.channels_mask(), Some(0b0000000000000100)); // DO 输出通道
        assert_eq!(rs_data.channels_mask(), Some(0b0000000000001000)); // RS DO 通道
    }

    #[test]
    fn test_specific_data_type_accessors() {
        let mut di_data = DIInputData::new();
        let mut ai_data = AIInputData::new();
        let mut do_data = DOOutputData::new();
        let mut rs_data = RSPortData::new();

        // 测试 DI 数据
        di_data.set_input_channel_state(0, true);
        di_data.set_output_channel_state(1, true);
        assert_eq!(di_data.input_channels_mask(), 0b0000000000000001);
        assert_eq!(di_data.output_channels_mask(), 0b0000000000000010);

        // 测试 AI 数据
        ai_data.set_AI_value(42.0);
        ai_data.set_DO_channel_state(2, true);
        assert_eq!(ai_data.get_analog_value(), 42.0);
        assert_eq!(ai_data.channels_mask(), 0b0000000000000100);

        // 测试 DO 数据
        do_data.set_input_channel_state(3, true);
        do_data.set_output_channel_state(4, true);
        assert_eq!(do_data.input_channels_mask(), 0b0000000000001000);
        assert_eq!(do_data.output_channels_mask(), 0b0000000000010000);

        // 测试 RS 数据
        rs_data.set_AI_value(24.0);
        rs_data.set_DO_channel_state(5, true);
        assert_eq!(rs_data.get_analog_value(), 24.0);
        assert_eq!(rs_data.channels_mask(), 0b0000000000100000);
    }
}
