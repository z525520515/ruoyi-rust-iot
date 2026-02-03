// S909 示例测试通过（不包含透传）
use DEVICE_PROTO::s909pro::S909UP;

fn main() {
    let mut datas: Vec<Vec<u8>> = vec![];
    // datas.push(hex_from_str(
    //     "A55A30003753484D4B323430333939393300D4C7001C0038894108110004010000001000000000000000000000000000000000144C55AA"
    // ).unwrap());
    // datas.push(hex_from_str(
    //     "A55A30003753484D4B323430313939393300D4C4011C0065B212681100043005000510000000000000000000000000040302010C4055AA"
    // ).unwrap());
    // datas.push(hex_from_str(
    //     "A55A30002C53484D4B323430313939393300D4C403010065B2183F1600063EBE66660000034D5041CE4655AA",
    // ).unwrap());
    // datas.push(hex_from_str(
    //     "A55A30002C53484D4B323430313939393300D4C803010065B21F271E0006424C00000000034D5041249555AA"
    // ).unwrap());
    // datas.push(hex_from_str(
    //     "A55A30002C53484D4B323430313939393300D4C403010065B222AB1E0006459BDF5C0000034B5041821255AA"
    // ).unwrap());

    // 现场数据
    datas.push(hex_from_str(
        "A55A30003753485250323530353030313600D4CE001C006976B6E21100040909000010200011001E00100120001100260014011FF855AA"
    ).unwrap());

    datas.push(hex_from_str(
        "A55A30003753485250323530393031363600D4C6001C006976B73411000451410000100022201E121110010028260908151401C5D355AA"
    ).unwrap());

    datas.push(hex_from_str(
        "A55A30003753485250323530393031373900D4BC001C006976BAA1110004212100001000000000000000000000000000000000A53C55AA"
    ).unwrap());

    datas.iter().for_each(|data| {
        let mut frame = S909UP::from_bytes(&data).unwrap();
        println!("{}", serde_json::to_string_pretty(&frame).unwrap());
        println!(
            "[{:X}]\tcommand_type: {}",
            frame.command_type, frame.command_type
        );
        println!(
            "[{:X}]\tframe_length: {}",
            frame.frame_length.len(),
            frame.frame_length.len()
        );
        println!(
            "[{:?}]\tdevice_id: {}",
            frame.device_id.to_bytes(),
            frame.device_id.to_string()
        );
        println!(
            "[{:X}]\tprotocol_version: {}",
            frame.protocol_version.to_u8(),
            frame.protocol_version.to_u8()
        );
        println!(
            "[{:b}]\tpower_status: level({}%) {:?} is_external_power: {:?}",
            frame.power_status.to_u8(),
            frame.power_status.level.get_level(),
            frame.power_status.source,
            frame.power_status.is_external_power
        );
        println!(
            "[{:X}]signal_strength: {}{}",
            frame.signal_strength.get_dbm() as u8,
            frame.signal_strength.get_dbm(),
            frame.signal_strength.get_unit()
        );
        println!(
            "[{:X}]channel_id: {}",
            frame.channel_id.to_u8(),
            frame.channel_id.to_u8(),
        );
        println!(
            "[{:X}]device_type: {}",
            frame.device_type.to_u8(),
            frame.device_type.info().name
        );
        println!(
            "[{:b}]alarm_status: {:?}",
            frame.device_alarm_status.to_u8(),
            frame
                .device_alarm_status
                .interpret(frame.device_type.clone())
        );
        println!(
            "[{:?}]timestamp: {}",
            frame.timestamp.to_bytes(),
            frame.timestamp.to_formatted_string()
        );
        println!(
            "[{:X}]channel_type: {:?}",
            frame.channel_type.to_u8(),
            frame.channel_type
        );
        println!(
            "[{:?}]channel_data_length: {}",
            frame.channel_data_length.to_bytes(),
            frame.channel_data_length.to_u16()
        );
        println!(
            "[{:?}]channel_data: {:?}",
            frame.channel_data.to_bytes(),
            frame.channel_data
        );
        println!(
            "[{:X}]unit_length: {}",
            frame.unit_length, frame.unit_length
        );
        println!(
            "[{:?}]unit_value: {:?}",
            frame.unit_value.to_bytes(),
            frame.unit_value.get_res()
        );
        println!("[{:?}]crc: {}", frame.crc.to_be_bytes(), frame.crc);
    });
}

fn hex_from_str(input: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    input
        .trim()
        .as_bytes()
        .chunks(2)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .map(|chunk| u8::from_str_radix(chunk, 16))
        .collect()
}
