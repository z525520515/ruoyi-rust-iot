/// 帧起始标记
pub const FRAME_START: [u8; 2] = [0xA5, 0x5A];

/// 帧结束标记
pub const FRAME_END: [u8; 2] = [0x55, 0xAA];

/// 命令类型
pub mod command {
    /// 上传数据命令类型（网关）
    pub const UPLOAD_DATA: u8 = 0x30;

    /// 上传数据命令类型（设备）
    pub const UPLOAD_DATA_DEVICE: u8 = 0x00;

    /// 下行数据命令类型
    pub const DOWNLINK_DATA: u8 = 0xff;

    /// 设备上传设置报文命令类型
    pub const UPLOAD_SETTING: u8 = 0x04;
}
