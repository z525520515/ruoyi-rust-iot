use DEVICE_PROTO::dev_frame::DevFrame;

fn main() {
    let mut datas: Vec<Vec<u8>> = vec![];
    // datas.push(
    //     hex_from_str(
    //         "a55a00003b53484d4b313930363030313334c8003c000a0102006401f4021a5d1966b60219021a0216021802180213021e021c021d021d7e0355aa",
    //     )
    //     .unwrap(),
    // );

    // 压力表数据
    "A55A00002953485250323530353030303833C4000000010100000009C4016E697688FA016E47E655AA
    A55A00002953485250323530393032333034D0000000010100000009C401C2697688E701C2390655AA
    A55A00002953485250323530393033343134BA000000010100000009C402666976890602661D1D55AA
    A55A00002953485250323530393033333834C6000000010100000009C4028E69768908028E7BF255AA
    A55A00002953485250323530393032303434CC000000010100000009C400F86976891100F8B8DC55AA
    A55A00002953485250323530393033323334D0000000010100000009C400E26976892C00E21EB855AA
    A55A00002953485250323530393031343534D0000000010100000009C402EB6976893102EB92E855AA
    A55A00002953485250323530393032383234BE000000010100000009C4044F6976893F044F4CF455AA
    A55A00002953485250323530393031323634BE000000010100000009C400C76976894500C7D6E555AA
    A55A00002953485250323530393031313934C8000000010100000009C40360697689490360E9AC55AA
    A55A00002953485250323530393031333434B6000000010100000009C400D86976894F00D8927B55AA
    A55A00002953485250323530393032363634BE000000010100000009C401F86976895501F84ABA55AA
    A55A00002953485250323530393030313434C6000000010100000009C4044169768960044185B455AA
    A55A00002953485250323530393031353634D0000000010800000001F400536976899B00536F4F55AA
    A55A000029534852503235303930323038349C000000010100000009C400CF6976899000CF459655AA
    A55A00002953485250323530393032313534CA000000010100000009C4017E697689A9017E3EB255AA
    A55A00002953485250323530393032323534D0000000010100000009C402DA697689AA02DAA5CA55AA
    A55A00002953485250323530393032313734C6000000010100000009C400DD697689A900DDA29155AA
    A55A00002953485250323530393030373134D0000000010100000009C403E2697689C903E2340B55AA
    A55A00002953485250323530393031393734C2000000010100000009C400D3697689E600D302E255AA
    A55A00002953485250323530393032313134C6000000010100000009C400F469768A0100F47C0F55AA
    A55A00002953485250323530393033323534C2000000010100000009C4027569768A290275985455AA
    A55A00002953485250323530393030393134C2000000010100000009C4028669768A2A028600CC55AA
    A55A00002953485250323530393030383034D0000000010100000009C4021B69768A3F021BC22D55AA
    A55A00002953485250323530393032333934B8000000010100000009C401B669768A4A01B6B04755AA
    A55A00002953485250323530393031353334D0000000010100000009C4010D69768AA3010D30B955AA
    A55A00002953485250323530353030303234D0000000010100000009C400CC69768AA900CC2FFF55AA
    A55A00002953485250323530393033353134C0000000010100000009C4032869768ACD0328159355AA
    A55A00002953485250323530393033363334C0000000010100000009C4026469768AE00264861F55AA
    A55A00002953485250323530393031313234B6000000010100000009C400F969768AE300F9528F55AA
    A55A00002953485250323530393032343234B2000000010100000009C401E169768B5E01E1377655AA
    A55A00002953485250323530393030323934C2000000010100000009C402E769768B6102E73F7F55AA
    A55A00002953485250323530393033373434CC000000010100000009C401C869768B6501C82B9655AA
    A55A00002953485250323530393032343634D0000000010100000009C401B969768B8501B9077C55AA
    A55A00002953485250323530393032373434CC000000010100000009C4029F69768BA0029F546C55AA
    A55A00002953485250323530393030383734D0000000010100000009C403F169768BEE03F1AD0055AA
    A55A00002953485250323530393031303634C6000000010100000009C4027669768BF902763FDF55AA
    A55A00002953485250323530393031393834D0000000010100000009C400D369768C1600D379A055AA
    A55A00002953485250323531303030303134D0000000010100000009C4044169768C440441DC3B55AA
    A55A00002953485250323530393032333434C4000000010100000009C401E269768C5401E283A655AA
    A55A00002953485250323530393031323834B4000000010100000009C400D169768C5300D1268655AA
    A55A00002953485250323530393032363134BA000000010100000009C4043969768C5A043931DF55AA
    A55A00002953485250323530393030313034CA000000010100000009C4029269768C760292822355AA
    A55A00002953485250323530393031343034BE000000010100000009C400F569768CA700F5765755AA
    A55A00002953485250323530393031393934C4000000010100000009C4000F69768CB0000F2C7955AA
    A55A00002953485250323530393033323734B4000000010100000009C4029169768CB702911A6055AA
    A55A00002953485250323530393033353634D0000000010100000009C4026569768CBD02655FBD55AA
    A55A00002953485250323530393031333934C2000000010100000009C400D369768CFC00D3580655AA
    A55A00002953485250323530393033363234CC000000010100000009C402BC69768D3502BCDD5455AA
    A55A00002953485250323530393032303134D0000000010100000009C400E569768D3400E58C4355AA
    A55A00002953485250323530393030373634CC000000010100000009C403DA69768D4C03DAD90255AA
    A55A00002953485250323530393031313134D0000000010100000009C4030269768D770302D0A155AA"
        .split('\n')
        .for_each(|data_raw| datas.push(hex_from_str(data_raw).unwrap()));

    println!("datas len: {}", datas.len());

    datas.iter().for_each(|data| {
        let frame = DevFrame::from_bytes(&data).unwrap();
        // println!("{}", serde_json::to_string_pretty(&frame).unwrap());
        println!(
            "\n[{:X}]\tcommand_type: {}",
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
            "[{:04X}]data_record_interval: {}",
            frame.data_record_interval.get_interval(),
            frame.data_record_interval.get_interval(),
        );
        println!(
            "[{:04X}]data_record_count: {}",
            frame.data_record_count.get_count(),
            frame.data_record_count.get_count(),
        );
        println!(
            "[{:X}]device_type: {}",
            frame.device_type.to_u8(),
            frame.device_type.info().name
        );
        println!(
            "[{:b}]alarm_status: {:?}",
            frame.alarm_status.to_u8(),
            frame.alarm_status.interpret(frame.device_type.clone())
        );
        print!(
            "[{:X?}]alarm_parameters: - alarm_group: {} ",
            frame.alarm_parameters.to_bytes(),
            frame.alarm_parameters.get_alarm_group(),
        );
        if frame.alarm_parameters.get_alarm_group() == 1 {
            println!(
                " - alarm_lower_limit: {:?} - alarm_upper_limit: {:?}",
                frame.alarm_parameters.get_alarm_lower_limit(0).unwrap(),
                frame.alarm_parameters.get_alarm_upper_limit(0).unwrap()
            );
        } else {
            println!(
                " - alarm_lower_limit: {:?} {:?} - alarm_upper_limit: {:?} {:?}",
                frame.alarm_parameters.get_alarm_lower_limit(0).unwrap(),
                frame.alarm_parameters.get_alarm_lower_limit(1).unwrap(),
                frame.alarm_parameters.get_alarm_upper_limit(0).unwrap(),
                frame.alarm_parameters.get_alarm_upper_limit(1).unwrap()
            )
        }
        println!(
            "[{:?}]current_device_data: {}",
            get_either_data_bytes(&frame.current_device_data),
            get_either_data_description(&frame.current_device_data)
        );
        println!(
            "[{:?}]timestamp: {}",
            frame.timestamp.to_bytes(),
            frame.timestamp.to_formatted_string()
        );
        frame
            .cache_device_data
            .iter()
            .zip(&frame.cache_timestamp)
            .for_each(|(device_data, timestamp)| {
                println!("{{");
                println!(
                    "\t[{:?}]device_data: {}",
                    get_either_data_bytes(device_data),
                    get_either_data_description(device_data)
                );
                println!(
                    "\t[{:?}]timestamp: {}",
                    timestamp.to_bytes(),
                    timestamp.to_formatted_string()
                );
                println!("}}");
            });
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

fn get_either_data_description(
    device_data: &either::Either<
        DEVICE_PROTO::frame::device_data::DeviceData,
        DEVICE_PROTO::frame::device_data::ModbusData,
    >,
) -> String {
    match device_data {
        either::Either::Left(device_data) => device_data.get_description(),
        either::Either::Right(modbus_data) => modbus_data.get_description(),
    }
}

fn get_either_data_bytes(
    device_data: &either::Either<
        DEVICE_PROTO::frame::device_data::DeviceData,
        DEVICE_PROTO::frame::device_data::ModbusData,
    >,
) -> Vec<u8> {
    match device_data {
        either::Either::Left(device_data) => device_data.to_bytes(),
        either::Either::Right(modbus_data) => modbus_data.to_bytes(),
    }
}
