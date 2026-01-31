use super::device_type::DeviceType;
use serde::{Deserialize, Serialize};

/// 报警状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct AlarmStatus {
    bits: u8,
}

impl AlarmStatus {
    pub fn from_u8(value: u8) -> Self {
        Self { bits: value }
    }

    pub fn to_u8(self) -> u8 {
        self.bits
    }

    pub fn is_normal(&self, bit: u8) -> bool {
        assert!(bit < 8, "Bit index out of range");
        (self.bits & (1 << bit)) != 0
    }

    pub fn set_bit(&mut self, bit: u8, is_alarm: bool) {
        assert!(bit < 8, "Bit index out of range");
        if is_alarm {
            self.bits |= 1 << bit;
        } else {
            self.bits &= !(1 << bit);
        }
    }

    pub fn has_any_alarm(&self) -> bool {
        self.bits != 0
    }

    pub fn clear_all_alarm(&mut self) {
        self.bits = 0;
    }

    pub fn interpret(&self, device_type: DeviceType) -> Vec<String> {
        let mut alarms = Vec::new();
        for bit in 0..8 {
            if self.is_normal(bit) {
                alarms.push(get_bit_description(device_type, bit));
            }
        }
        alarms
    }

    pub fn get_active_bits(&self) -> Vec<u8> {
        (0..8).filter(|&bit| self.is_normal(bit)).collect()
    }
}

impl Default for AlarmStatus {
    fn default() -> Self {
        Self::from_u8(0)
    }
}

impl From<u8> for AlarmStatus {
    fn from(value: u8) -> Self {
        Self::from_u8(value)
    }
}

impl From<AlarmStatus> for u8 {
    fn from(status: AlarmStatus) -> Self {
        status.bits
    }
}

// impl std::fmt::Display for AlarmStatus {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "AlarmStatus(0b{:08b})", self.bits)
//     }
// }

fn get_bit_description(device_type: DeviceType, bit: u8) -> String {
    match device_type {
        DeviceType::FireHydrant => match bit {
            0 => "压力低报警",
            1 => "压力高报警",
            5 => "撞击报警",
            6 => "倾斜报警",
            7 => "放水报警",
            _ => default_bit_description(bit),
        }
        .to_string(),

        DeviceType::FireHydrantPressure => match bit {
            7 => "放水报警",
            _ => default_bit_description(bit),
        }
        .to_string(),

        DeviceType::UltrasonicLevelGauge => match bit {
            0 => "液位超高报警",
            1 => "液位高报警",
            _ => default_bit_description(bit),
        }
        .to_string(),

        DeviceType::FireHydrantCover => match bit {
            0 => "闷盖浸水报警",
            5 => "撞击报警",
            6 => "开盖报警",
            _ => default_bit_description(bit),
        }
        .to_string(),

        DeviceType::ManholeCoverLevel => match bit {
            7 => "井盖开启报警",
            _ => default_bit_description(bit),
        }
        .to_string(),

        DeviceType::TemperatureAbnormal => match bit {
            5 => "异动报警",
            _ => default_bit_description(bit),
        }
        .to_string(),

        DeviceType::FireHydrantFlow => match bit {
            0 => "压力低报警",
            1 => "压力高报警",
            5 => "撞击报警",
            6 => "倾斜报警",
            7 => "放水报警",
            _ => default_bit_description(bit),
        }
        .to_string(),

        DeviceType::SmartManholeCoverMonitor => match bit {
            0 => "浸水报警",
            5 => "异动报警",
            6 => "倾斜报警",
            _ => default_bit_description(bit),
        }
        .to_string(),

        _ => default_bit_description(bit).to_string(),
    }
}

fn default_bit_description(bit: u8) -> &'static str {
    assert!(
        bit < 8,
        "Bit index out of bounds for default_bit_description(未知报警状态)"
    );

    match bit {
        0 => "低报警",
        1 => "高报警",
        2 => "电量报警",
        3 => "传感器异常",
        4 => "硬件异常",
        5 => "第一传感器报警",
        6 => "第二传感器报警",
        7 => "第三传感器报警",
        _ => "未知报警",
    }
}

impl DeviceType {
    pub fn interpret_alarm_status(&self, status: AlarmStatus) -> Vec<String> {
        status.interpret(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alarm_status_basic() {
        let status = AlarmStatus::from(0b0000_0001);
        assert!(status.is_normal(0));
        assert!(!status.is_normal(1));
        assert!(status.has_any_alarm());

        let empty = AlarmStatus::from(0);
        assert!(!empty.has_any_alarm());
    }

    #[test]
    fn test_alarm_status_set_bit() {
        let mut status = AlarmStatus::from(0);
        assert!(!status.is_normal(0));

        status.set_bit(0, true);
        assert!(status.is_normal(0));
        assert_eq!(status.to_u8(), 0b0000_0001);

        status.set_bit(0, false);
        assert!(!status.is_normal(0));
        assert_eq!(status.to_u8(), 0);
    }

    #[test]
    fn test_alarm_status_clear_all() {
        let mut status = AlarmStatus::from_u8(0b1111_1111);
        assert!(status.has_any_alarm());

        status.clear_all_alarm();
        assert!(!status.has_any_alarm());
        assert_eq!(status.to_u8(), 0);
    }

    #[test]
    fn test_default_interpretation() {
        let status = AlarmStatus::from_u8(0b0000_0011); // BIT0和BIT1
        let device_type = DeviceType::PressureGaugeMpa;
        let alarms = status.interpret(device_type);

        assert_eq!(alarms.len(), 2);
        assert!(alarms.contains(&"低报警".to_string()));
        assert!(alarms.contains(&"高报警".to_string()));
    }

    #[test]
    fn test_fire_hydrant_interpretation() {
        let status = AlarmStatus::from_u8(0b1110_0011); // BIT7,6,5,1,0
        let device_type = DeviceType::FireHydrant;
        let alarms = status.interpret(device_type);

        assert_eq!(alarms.len(), 5);
        assert!(alarms.contains(&"压力低报警".to_string()));
        assert!(alarms.contains(&"压力高报警".to_string()));
        assert!(alarms.contains(&"撞击报警".to_string()));
        assert!(alarms.contains(&"倾斜报警".to_string()));
        assert!(alarms.contains(&"放水报警".to_string()));
    }

    #[test]
    fn test_fire_hydrant_pressure_interpretation() {
        let status = AlarmStatus::from_u8(0b1000_0011); // BIT7,1,0
        let device_type = DeviceType::FireHydrantPressure;
        let alarms = status.interpret(device_type);

        assert_eq!(alarms.len(), 3);
        assert!(alarms.contains(&"低报警".to_string()));
        assert!(alarms.contains(&"高报警".to_string()));
        assert!(alarms.contains(&"放水报警".to_string()));
    }

    #[test]
    fn test_ultrasonic_level_interpretation() {
        let status = AlarmStatus::from_u8(0b0000_0011); // BIT1,0
        let device_type = DeviceType::UltrasonicLevelGauge;
        let alarms = status.interpret(device_type);

        assert_eq!(alarms.len(), 2);
        assert!(alarms.contains(&"液位超高报警".to_string()));
        assert!(alarms.contains(&"液位高报警".to_string()));
    }

    #[test]
    fn test_fire_hydrant_cover_interpretation() {
        let status = AlarmStatus::from_u8(0b0110_0001); // BIT6,5,0
        let device_type = DeviceType::FireHydrantCover;
        let alarms = status.interpret(device_type);

        assert_eq!(alarms.len(), 3);
        assert!(alarms.contains(&"闷盖浸水报警".to_string()));
        assert!(alarms.contains(&"撞击报警".to_string()));
        assert!(alarms.contains(&"开盖报警".to_string()));
    }

    #[test]
    fn test_manhole_cover_level_interpretation() {
        let status = AlarmStatus::from_u8(0b1000_0000); // BIT7
        let device_type = DeviceType::ManholeCoverLevel;
        let alarms = status.interpret(device_type);

        assert_eq!(alarms.len(), 1);
        assert!(alarms.contains(&"井盖开启报警".to_string()));
    }

    #[test]
    fn test_temperature_abnormal_interpretation() {
        let status = AlarmStatus::from_u8(0b0010_0011); // BIT5,1,0
        let device_type = DeviceType::TemperatureAbnormal;
        let alarms = status.interpret(device_type);

        assert_eq!(alarms.len(), 3);
        assert!(alarms.contains(&"低报警".to_string()));
        assert!(alarms.contains(&"高报警".to_string()));
        assert!(alarms.contains(&"异动报警".to_string()));
    }

    #[test]
    fn test_fire_hydrant_flow_interpretation() {
        let status = AlarmStatus::from_u8(0b1110_0011); // BIT7,6,5,1,0
        let device_type = DeviceType::FireHydrantFlow;
        let alarms = status.interpret(device_type);

        assert_eq!(alarms.len(), 5);
        assert!(alarms.contains(&"压力低报警".to_string()));
        assert!(alarms.contains(&"压力高报警".to_string()));
        assert!(alarms.contains(&"撞击报警".to_string()));
        assert!(alarms.contains(&"倾斜报警".to_string()));
        assert!(alarms.contains(&"放水报警".to_string()));
    }

    #[test]
    fn test_smart_manhole_cover_interpretation() {
        let status = AlarmStatus::from_u8(0b0110_0001); // BIT6,5,0
        let device_type = DeviceType::SmartManholeCoverMonitor;
        let alarms = status.interpret(device_type);

        assert_eq!(alarms.len(), 3);
        assert!(alarms.contains(&"浸水报警".to_string()));
        assert!(alarms.contains(&"异动报警".to_string()));
        assert!(alarms.contains(&"倾斜报警".to_string()));
    }

    #[test]
    fn test_alarm_status_conversion() {
        let status = AlarmStatus::from(0b1010_1010);
        assert_eq!(status.to_u8(), 0b1010_1010);
        assert_eq!(u8::from(status), 0b1010_1010);
    }

    #[test]
    fn test_get_active_bits() {
        let status = AlarmStatus::from_u8(0b1010_0101);
        let active_bits = status.get_active_bits();

        assert_eq!(active_bits.len(), 4);
        assert!(active_bits.contains(&0));
        assert!(active_bits.contains(&2));
        assert!(active_bits.contains(&5));
        assert!(active_bits.contains(&7));
    }

    #[test]
    fn test_device_type_interpret_method() {
        let status = AlarmStatus::from_u8(0b1110_0011);
        let device_type = DeviceType::FireHydrant;
        let alarms = device_type.interpret_alarm_status(status);

        assert_eq!(alarms.len(), 5);
    }

    // #[test]
    // fn test_display() {
    //     let status = AlarmStatus::from_u8(0b1010_1010);
    //     let display = format!("{}", status);
    //     assert_eq!(display, "AlarmStatus(0b10101010)");
    // }

    #[test]
    fn test_boundary_bits() {
        let mut status = AlarmStatus::from_u8(0);

        // 测试边界位
        status.set_bit(0, true);
        assert!(status.is_normal(0));

        status.set_bit(7, true);
        assert!(status.is_normal(7));
    }
}
