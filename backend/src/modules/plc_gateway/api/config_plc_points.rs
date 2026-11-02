use crate::modules::plc_gateway::api::config_util::call_device;
use crate::modules::plc_gateway::unified::response::error_response;
use axum::http::{StatusCode, header};
use axum::{
    extract::{Multipart, Path},
    response::{IntoResponse, Json},
};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Value, json};
use std::fmt;
use std::fs;
#[cfg(feature = "plc_gateway_xlsx")]
use xlsxwriter::{Format, Workbook, Worksheet};

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlcModbusNodePointReq {
    pub id: Option<u16>,
    pub name: String,
    pub addr: String,
    pub datatype: String,
    pub protocol: String,
    /* ===== 选填，全部用 Option ===== */
    pub reg_num: Option<u16>,
    pub reg_swap: Option<String>,
    pub param_k: Option<u16>,
    pub multiples: Option<u16>,
    pub precision: Option<u16>,
    pub effective: Option<u16>,
    pub bit_offset: Option<u16>,
    pub table: Option<String>,
    pub field: Option<String>,
    pub remark: Option<String>,
    // extra 字段
    pub trigger: Option<String>,
    pub condition: Option<String>,
    pub value: Option<u16>,
}

/// 提供添加分组后测点的配置，由于缺乏数据，目前只测试了协议类型为"modbus_tcp"的添加测试，后续如果有了其他协议类型，需要更新测试这里的函数
/// gid 是 pcl的uuid
/// 请求示例：/plc/points/10.0.0.5/7clxbl7h
pub async fn add_plc_config_group_points(
    Path((ip, uuid)): Path<(String, String)>,
    Json(params): Json<PlcModbusNodePointReq>,
) -> Json<Value> {
    handle_plc_point(ip, uuid, params, true).await
}

/// 更新测点配置
pub async fn update_plc_config_group_points(
    Path((ip, uuid)): Path<(String, String)>,
    Json(params): Json<PlcModbusNodePointReq>,
) -> Json<Value> {
    handle_plc_point(ip, uuid, params, false).await
}

/// 获取分组下的所有测点, gid 是group的uuid
pub async fn get_plc_config_group_points(Path((ip, uuid)): Path<(String, String)>) -> Json<Value> {
    let url = format!("http://{}:80/plc/api/v1/group/{}/node", &ip, uuid);
    call_device(url, json!([]), Some("GET")).await
}

pub async fn del_plc_config_group_points(
    Path((ip, uuid, id)): Path<(String, String, String)>,
) -> Json<Value> {
    let url = format!("http://{}:80/plc/api/v1/group/{}/node/{}", &ip, uuid, id);
    call_device(url, json!([]), Some("DELETE")).await
}

/// 统一处理 add / update 测点
async fn handle_plc_point(
    ip: String,
    uuid: String,
    params: PlcModbusNodePointReq,
    is_add: bool,
) -> Json<Value> {
    log::info!(
        "{} plc config group points: ip={}, uuid={}, params={:#?}",
        if is_add { "add" } else { "update" },
        ip,
        uuid,
        params
    );

    // === 1. id 校验（update 必填）===
    let id = if is_add {
        params.id.unwrap_or(0) // add 时可无，设为 0
    } else {
        match params.id {
            Some(v) => v,
            None => return error_response("id is required".to_string(), Some(400)),
        }
    };

    // === 2. extra 字段构建 ===
    let extra = json!({
        "trigger": params.trigger.as_deref().unwrap_or(""),
        "condition": params.condition.as_deref().unwrap_or(""),
        "value": params.value.unwrap_or(0),
    });

    // === 3. trigger / condition 通用校验 ===
    if let Some(trigger) = &params.trigger {
        if !["onChange", "onValue"].contains(&trigger.as_str()) {
            return error_response(
                "trigger must be 'onChange' or 'onValue'".to_string(),
                Some(400),
            );
        }
    }
    if let Some(condition) = &params.condition {
        if !["eq", "ne", "ge", "gt", "le", "lt"].contains(&condition.as_str()) {
            return error_response(
                "condition must be one of: eq, ne, ge, gt, le, lt".to_string(),
                Some(400),
            );
        }
    }

    // === 4. datatype 校验 ===
    let datatype_upper = params.datatype.to_uppercase();
    let alt_types = ["UINT16", "INT16", "UINT32", "INT32", "FLOAT"];
    let is_alt_type = alt_types.contains(&datatype_upper.as_str());

    if !matches!(
        datatype_upper.as_str(),
        "BIT" | "BOOL" | "UINT16" | "INT16" | "UINT32" | "INT32" | "FLOAT" | "STRING"
    ) {
        return error_response("unknown datatype".to_string(), Some(400));
    }

    // === 5. 按 datatype 校验必填字段 ===
    macro_rules! require {
        ($opt:expr, $msg:expr) => {
            if $opt.is_none() {
                return error_response($msg.to_string(), Some(400));
            }
        };
    }

    match datatype_upper.as_str() {
        "BIT" => {
            require!(params.reg_swap, "regSwap is required for BIT");
            require!(params.bit_offset, "bitOffset is required for BIT");
        }
        "BOOL" => {}
        "UINT16" | "INT16" | "UINT32" | "INT32" | "FLOAT" => {
            require!(params.reg_swap, "regSwap is required");
            require!(params.multiples, "multiples is required");
            if params.precision.is_some() && params.effective.is_some() {
                return error_response(
                    "cannot have both precision and effective".to_string(),
                    Some(400),
                );
            }
            if params.precision.is_none() && params.effective.is_none() {
                return error_response(
                    "must provide either precision or effective".to_string(),
                    Some(400),
                );
            }
        }
        "STRING" => {
            require!(params.reg_swap, "regSwap is required for STRING");
            require!(params.reg_num, "regNum is required for STRING");
        }
        _ => unreachable!(),
    }

    // === 6. 构建 point 对象 ===
    let mut point = json!({
        "id": id,
        "name": params.name,
        "datatype": params.datatype,
        "addr": params.addr,
        "regSwap": params.reg_swap.as_deref().unwrap_or("AB"),
        "paramK": params.param_k.unwrap_or(0),
        "multiples": params.multiples.unwrap_or(1),
        "table": params.table.as_deref().unwrap_or(""),
        "field": params.field.as_deref().unwrap_or(""),
        "remark": params.remark.as_deref().unwrap_or(""),
        "extra": extra.to_string(),
        "bitOffset": params.bit_offset.unwrap_or(0),
        "protocol": params.protocol,
    });

    // precision / effective
    if is_alt_type {
        if let Some(p) = params.precision {
            point["precision"] = json!(p);
        }
        if let Some(e) = params.effective {
            point["effective"] = json!(e);
        }
    } else {
        point["precision"] = json!(params.precision.unwrap_or(3));
    }

    if let Some(reg_num) = params.reg_num {
        point["regNum"] = json!(reg_num);
    }

    // === 7. 构造 URL ===
    let url = if is_add {
        format!("http://{}:80/plc/api/v1/group/{}/node", ip, uuid)
    } else {
        format!("http://{}:80/plc/api/v1/group/{}/node/{}", ip, uuid, id)
    };

    let forward = if is_add { json!([point]) } else { json!(point) };
    log::info!(" 处理请求修改、添加points测点 url: {}", url);
    call_device(url, forward, Some(if is_add { "POST" } else { "PUT" })).await
}

struct StringOrU16;

impl<'de> Visitor<'de> for StringOrU16 {
    type Value = Option<u16>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string, integer or null")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.is_empty() {
            Ok(None)
        } else {
            v.parse::<u16>().map(Some).map_err(E::custom)
        }
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v < 0 || v > u16::MAX as i64 {
            Err(E::custom("u16 out of range"))
        } else {
            Ok(Some(v as u16))
        }
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v > u16::MAX as u64 {
            Err(E::custom("u16 out of range"))
        } else {
            Ok(Some(v as u16))
        }
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }
}

fn string_or_u16_or_none<'de, D>(deserializer: D) -> Result<Option<u16>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(StringOrU16)
}

#[derive(Deserialize, Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePoints {
    pub name: String,     // 变量名称(必填)
    pub addr: String,     // 地址(必填)
    pub datatype: String, // 数据类型(必填)
    pub protocol: String, // 协议类型，有 s7,mc, opcua,modbus_tcp,modbus_rtu_over_tcp,modbus_rtu, dlt645_2007协议
    /* ===== 选填，全部用 Option ===== */
    pub str_num: Option<u16>, // 跳过 字符个数(选填)(s7,mc) 由于没有s7,mc协议的测试数据，先跳过，后期补充
    #[serde(deserialize_with = "string_or_u16_or_none")]
    pub reg_num: Option<u16>, // 寄存器数目(选填)(modbus) 支持 "2", 2, "", null
    pub reg_swap: Option<String>, // 寄存器交换(选填)(modbus)
    #[serde(deserialize_with = "string_or_u16_or_none")]
    pub bit_offset: Option<u16>, // 比特位置(选填)(modbus)
    pub param_k: Option<u16>, // 设备系数计算(选填)(modbus,dlt645)
    pub multiples: Option<u16>, //倍率(选填)
    pub precision: Option<u16>, //保留小数位数(选填)
    pub effective: Option<u16>, //保留有效数字位数(选填)
    pub table: Option<String>, // 数据表名称(选填)
    pub field: Option<String>, // 数据字段(选填)
    pub remark: Option<String>, // 变量描述(选填)
    pub extra: Option<String>, //自定义字段(选填)
}
// ---------- 2. Excel 生成 ----------
#[cfg(feature = "plc_gateway_xlsx")]
fn node_points_to_xlsx(nodes: Vec<NodePoints>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // 1. 创建临时路径
    let tmp_path = std::env::temp_dir().join(format!("plc_points_{}.xlsx", uuid::Uuid::new_v4()));

    // 2. 创建 Workbook（关键修复）
    let workbook = Workbook::new(tmp_path.to_str().expect("Failed to convert path to string"))?;

    // 3. 创建 sheet
    let mut sheet = workbook.add_worksheet(None)?;

    let headers = [
        "变量名称(必填)",
        "地址(必填)",
        "数据类型(必填)",
        "字符个数(选填)(s7,mc)",
        "寄存器数目(选填)(modbus)",
        "寄存器交换(选填)(modbus)",
        "比特位置(选填)(modbus)",
        "设备系数计算(选填)(modbus,dlt645)",
        "倍率(选填)",
        "保留小数位数(选填)",
        "保留有效数字位数(选填)",
        "数据表名称(选填)",
        "数据字段(选填)",
        "变量描述(选填)",
        "自定义字段(选填)",
    ];
    // 4. 表头 + 加粗格式（关键修复）
    let mut binding = Format::new();
    let bold = binding.set_bold();

    for (col, h) in headers.iter().enumerate() {
        sheet.write_string(0, col as u16, h, Some(&bold))?;
    }

    for (row_idx, node) in nodes.iter().enumerate() {
        let row = (row_idx + 1) as u32;
        sheet.write_string(row, 0, &node.name, None)?;
        sheet.write_string(row, 1, &node.addr, None)?;
        sheet.write_string(row, 2, &node.datatype, None)?;

        if node.protocol == "s7" || node.protocol == "mc" {
            write_opt_u16(&mut sheet, row, 3, node.str_num)?;
        } else {
            write_opt_u16(&mut sheet, row, 3, None)?;
        }
        write_opt_u16(&mut sheet, row, 4, node.reg_num)?;
        write_opt_str(&mut sheet, row, 5, node.reg_swap.as_deref())?;
        write_opt_u16(&mut sheet, row, 6, node.bit_offset)?;
        write_opt_u16(&mut sheet, row, 7, node.param_k)?;
        write_opt_u16(&mut sheet, row, 8, node.multiples)?;
        write_opt_u16(&mut sheet, row, 9, node.precision)?;
        write_opt_u16(&mut sheet, row, 10, node.effective)?;
        write_opt_str(&mut sheet, row, 11, node.table.as_deref())?;
        write_opt_str(&mut sheet, row, 12, node.field.as_deref())?;
        write_opt_str(&mut sheet, row, 13, node.remark.as_deref())?;
        write_opt_str(&mut sheet, row, 14, node.extra.as_deref())?;
    }

    for col in 0..15 {
        sheet.set_column(col as u16, col as u16, 20.0, None)?;
    }

    workbook.close()?;
    let bytes = fs::read(&tmp_path)?;
    let _ = fs::remove_file(&tmp_path);
    Ok(bytes)
}

#[cfg(not(feature = "plc_gateway_xlsx"))]
fn node_points_to_xlsx(_nodes: Vec<NodePoints>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Err("xlsx export disabled (enable feature: plc_gateway_xlsx)".into())
}

#[cfg(feature = "plc_gateway_xlsx")]
fn write_opt_u16(
    ws: &mut Worksheet,
    row: u32,
    col: u16,
    opt: Option<u16>,
) -> Result<(), xlsxwriter::XlsxError> {
    if let Some(v) = opt {
        ws.write_number(row, col, v as f64, None)?;
    }
    Ok(())
}

#[cfg(feature = "plc_gateway_xlsx")]
fn write_opt_str(
    ws: &mut Worksheet,
    row: u32,
    col: u16,
    opt: Option<&str>,
) -> Result<(), xlsxwriter::XlsxError> {
    if let Some(s) = opt {
        ws.write_string(row, col, s, None)?;
    }
    Ok(())
}

// ---------- 3. JSON → Vec<NodePoints> ----------
fn json_array_to_node_points(arr: &[Value]) -> Result<Vec<NodePoints>, serde_json::Error> {
    serde_json::from_value(Value::Array(arr.to_vec()))
}

// ---------- 4. 下载接口 ----------
pub async fn downlod_plc_config_group_points(
    Path((ip, uuid, name)): Path<(String, String, String)>,
) -> impl IntoResponse {
    log::info!(
        "downlod_plc_config_group_points ip: {}, uuid: {}, name: {}",
        ip,
        uuid,
        name
    );
    // 1. 取原始数据
    let response = get_plc_config_group_points(Path((ip.clone(), uuid.clone()))).await;

    // 2. 取出 nodes
    let node_arr = match response.0["data"]["nodes"].as_array() {
        Some(a) => a,
        None => return error_response("data not found".to_string(), Some(404)).into_response(),
    };
    // println!("node_arr: {:?}", node_arr);
    // 3. 转结构体
    let nodes = match json_array_to_node_points(node_arr) {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("deserialize: {}", e),
            )
                .into_response();
        }
    };

    // 4. 生成 Excel
    let xlsx_bytes = match node_points_to_xlsx(nodes) {
        Ok(b) => b,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("xlsx: {}", e)).into_response();
        }
    };

    // 5. 响应头
    let filename = format!("plc_points_{}.xlsx", name);
    let headers = [
        (
            header::CONTENT_TYPE,
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        ),
        (
            header::CONTENT_DISPOSITION,
            &format!("attachment; filename=\"{}\"", filename),
        ),
    ];

    (headers, xlsx_bytes).into_response()
}

// ---------- 4. 下载接口 ----------
pub async fn downlod_points_template() -> impl IntoResponse {
    let xlsx_bytes = match node_points_to_xlsx(vec![]) {
        Ok(b) => b,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("xlsx: {}", e)).into_response();
        }
    };

    let filename = "plc_points_template.xlsx";
    let headers = [
        (
            header::CONTENT_TYPE,
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        ),
        (
            header::CONTENT_DISPOSITION,
            &format!("attachment; filename=\"{}\"", filename),
        ),
    ];

    (headers, xlsx_bytes).into_response()
}
/// 把 Excel 里读出来的单元格转成「干净」的标题字符串
fn clean_header(s: &str) -> String {
    s.trim().to_string()
}

/// 判断实际标题是否「以期望标题开头」
/// 例如：
///   expected = "寄存器交换"
///   actual   = "寄存器交换    (选填)(modbus)"   → true
///   actual   = "    寄存器交换"               → true
///   actual   = "寄存器交换顺序"               → false
fn starts_with_expected(actual: &str, expected: &str) -> bool {
    actual.starts_with(expected)
}
// ---------- 解析 Excel ----------
#[cfg(feature = "plc_gateway_xlsx")]
fn parse_xlsx_to_node_points(
    bytes: &[u8],
    protocol: &str,
) -> Result<Vec<NodePoints>, Box<dyn std::error::Error>> {
    use calamine::{open_workbook_from_rs, Reader, Xlsx};

    let cursor = std::io::Cursor::new(bytes);
    let mut workbook: Xlsx<_> = open_workbook_from_rs(cursor)?;
    let sheet_name = workbook
        .sheet_names()
        .first()
        .ok_or("no sheet found")?
        .clone();

    let mut nodes = Vec::new();
    let range = workbook
        .worksheet_range(&sheet_name)
        .map_err(|_| "failed to read worksheet range".to_string())?;

    let mut iter = range.rows();

    // 1. 读取并校验表头
    let header_row = iter.next().ok_or("empty file")?;
    // let expected_headers = vec![
    //     "变量名称(必填)",
    //     "地址(必填)",
    //     "数据类型(必填)",
    //     "字符个数(选填)(s7,mc)",
    //     "寄存器数目(选填)(modbus)",
    //     "寄存器交换(选填)(modbus)",
    //     "比特位置(选填)(modbus)",
    //     "设备系数计算(选填)(modbus,dlt645)",
    //     "倍率(选填)",
    //     "保留小数位数(选填)",
    //     "保留有效数字位数(选填)",
    //     "数据表名称(选填)",
    //     "数据字段(选填)",
    //     "变量描述(选填)",
    //     "自定义字段(选填)",
    // ];
    let expected_headers = vec![
        "变量名称",
        "地址",
        "数据类型",
        "字符个数",
        "寄存器数目",
        "寄存器交换",
        "比特位置",
        "设备系数计算",
        "倍率",
        "保留小数位数",
        "保留有效数字位数",
        "数据表名称",
        "数据字段",
        "变量描述",
        "自定义字段",
    ];
    // 关键：先校验列数
    if header_row.len() != expected_headers.len() {
        return Err(format!(
            "表头列数不匹配: 期望 {} 列, 实际 {} 列",
            expected_headers.len(),
            header_row.len()
        )
        .into());
    }
    // 再逐个校验内容,强校验
    // for (i, (cell, expected)) in header_row.iter().zip(expected_headers.iter()).enumerate() {
    //     let cell_str = cell.to_string().trim().to_owned();
    //     if cell_str != *expected {
    //         return Err(format!(
    //             "header mismatch at column {}: expected '{}', got '{}'",
    //             i + 1,
    //             expected,
    //             cell_str
    //         )
    //         .into());
    //     }
    // }
    // ---------- 2. 再逐列做「前缀」校验 ----------
    for (i, (cell, expected)) in header_row.iter().zip(expected_headers.iter()).enumerate() {
        let cell_clean = clean_header(&cell.to_string());

        if !starts_with_expected(&cell_clean, expected) {
            return Err(format!(
                "第 {} 列表头不匹配: 期望以 '{}' 开头, 实际 '{}'",
                i + 1,
                expected,
                cell_clean
            )
            .into());
        }
    }
    // 2. 逐行解析
    for (row_idx, row) in iter.enumerate() {
        let row_num = row_idx + 2; // Excel 行号从 2 开始（第1行是表头）

        macro_rules! get_cell {
            ($idx:expr) => {{
                row.get($idx)
                    .map(|c| c.to_string().trim().to_owned())
                    .unwrap_or_default()
            }};
        }
        // println!("row: {:?}", row);

        let name = get_cell!(0);
        let addr = get_cell!(1);
        let datatype = get_cell!(2);

        if name.is_empty() || addr.is_empty() || datatype.is_empty() {
            return Err(format!("row {}: 必填字段不能为空 (name, addr, datatype)", row_num).into());
        }

        let str_num = parse_u16_opt(get_cell!(3))?;
        let reg_num = parse_u16_opt(get_cell!(4))?;
        let reg_swap = empty_to_none(&get_cell!(5));
        let bit_offset = parse_u16_opt(get_cell!(6))?;
        let param_k = parse_u16_opt(get_cell!(7))?;
        let multiples = parse_u16_opt(get_cell!(8))?;
        let precision = parse_u16_opt(get_cell!(9))?;
        let effective = parse_u16_opt(get_cell!(10))?;
        let table = empty_to_none(&get_cell!(11));
        let field = empty_to_none(&get_cell!(12));
        let remark = empty_to_none(&get_cell!(13));
        let extra = empty_to_none(&get_cell!(14));

        nodes.push(NodePoints {
            name,
            addr,
            datatype,
            protocol: protocol.to_string(),
            str_num,
            reg_num,
            reg_swap,
            bit_offset,
            param_k,
            multiples,
            precision,
            effective,
            table,
            field,
            remark,
            extra,
        });
    }

    Ok(nodes)
}

#[cfg(not(feature = "plc_gateway_xlsx"))]
fn parse_xlsx_to_node_points(
    _bytes: &[u8],
    _protocol: &str,
) -> Result<Vec<NodePoints>, Box<dyn std::error::Error>> {
    Err("xlsx import disabled (enable feature: plc_gateway_xlsx)".into())
}

// 工具函数
fn parse_u16_opt(s: String) -> Result<Option<u16>, Box<dyn std::error::Error>> {
    let s = s.trim();
    if s.is_empty() || s.eq_ignore_ascii_case("null") {
        Ok(None)
    } else {
        Ok(Some(s.parse()?))
    }
}

fn empty_to_none(s: &str) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}
// ---------- 上传接口 ----------
pub async fn upload_points_file(
    Path((ip, uuid, protocol)): Path<(String, String, String)>,
    mut multipart: Multipart,
) -> Json<Value> {
    log::info!(
        "upload_points_file ip: {}, uuid: {}, protocol: {}",
        ip,
        uuid,
        protocol
    );
    if ![
        "s7",
        "mc",
        "opcua",
        "modbus",
        "modbus_rtu_over_tcp",
        "modbus_tcp",
        "modbus_rtu",
        "dlt645_2007",
    ]
    .contains(&protocol.as_str())
    {
        return error_response(format!("invalid protocol: {}", protocol), Some(400));
    }

    let mut file_bytes = None;

    // 1. 解析 multipart：获取文件
    while let Some(field) = multipart.next_field().await.unwrap() {
        // let name = field.name().unwrap_or("").to_string();
        // println!("name: {}", name);
        let data = field.bytes().await.unwrap();
        file_bytes = Some(data.to_vec());
    }

    let file_bytes = match file_bytes {
        Some(b) => b,
        None => return error_response("missing file".to_string(), Some(400)),
    };

    // 2. 读取 Excel 并校验表头 + 解析行
    let nodes = match parse_xlsx_to_node_points(&file_bytes, &protocol) {
        Ok(n) => n,
        Err(e) => return error_response(e.to_string(), Some(400)),
    };
    // println!("nodes: {:?}", nodes);
    // 3. 构造 JSON array [{},{}]
    let forward_json = match serde_json::to_value(&nodes) {
        Ok(v) => v,
        Err(e) => return error_response(format!("json: {}", e), Some(500)),
    };
    // println!("forward_json: {}", forward_json);
    // 4. 转发请求
    let url = format!("http://{}:80/plc/api/v1/group/{}/node", ip, uuid);

    call_device(url, forward_json, Some("PUT")).await
}
