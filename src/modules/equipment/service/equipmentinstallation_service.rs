use rbatis::plugin::page::Page;
use rbatis::plugin::page::PageRequest;
// 引用 数据表相关对象
use crate::modules::equipment::domain::dto::equipment_installation::{
    EquipmentInstallationAddDTO, EquipmentInstallationPageDTO, EquipmentInstallationUpdateDTO, EquipmentInstallationImportDTO,
};
use crate::modules::equipment::domain::table::tables::EquipmentInstallation;
use crate::modules::equipment::domain::vo::equipmentinstallationvo::EquipmentInstallationVo;
use crate::modules::system::service::CONTEXT;
//
use crate::error::Error;
use crate::error::Result;
// pool 数据库
use crate::pool;
use rbs::value;
use umya_spreadsheet::Spreadsheet;
use std::io::Write;
/// equipment_installation service
pub struct EquipmentInstallationService {}
impl EquipmentInstallationService {
    // 分页查询方法 传入EquipmentInstallationPageDTO,返回 Page<EquipmentInstallationVo>
    pub async fn page(
        &self,
        arg: &EquipmentInstallationPageDTO,
    ) -> Result<Page<EquipmentInstallationVo>> {
        let page_req = &PageRequest::from(arg);
        // 由于 使用的 postgresql 所以 limit 语句要从新按postgresql的语法注入参数
        let limitstr = format!(
            " limit {} offset {}",
            page_req.page_size,
            (page_req.page_no - 1) * page_req.page_size
        );
        let linits = &limitstr;
        let data = EquipmentInstallation::select_page(pool!(), page_req, linits, arg).await?;
        let page = Page::<EquipmentInstallationVo>::from(data);
        Ok(page)
    }
    // 调用表实体对象 EquipmentInstallation 的select_all方法 ,返回 Page<EquipmentInstallationVo>
    pub async fn finds_all(&self) -> Result<Vec<EquipmentInstallationVo>> {
        let data = EquipmentInstallation::select_all(pool!()).await?;
        let mut post_vos = vec![];
        for s in data {
            post_vos.push(EquipmentInstallationVo::from(s));
        }
        Ok(post_vos)
    }
    // 调用表实体对象 EquipmentInstallation select_by_map ,根据id返回 EquipmentInstallationVo
    pub async fn detail(&self, id: &str) -> Result<EquipmentInstallationVo> {
        // 1. 把 &str 解析成 i64，失败就返回一个业务错误
        let id_num: i64 = id.parse()
            .map_err(|_| Error::from(format!("id 必须是纯数字: {}", id)))?;

        // 2. 构造查询参数，此时 Value 是 Number，不再是 String
        let row = EquipmentInstallation::select_by_map(
            pool!(),
            value! { "id": id_num },   // 关键：这里传数字
        )
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from(format!("不存在 {:?} ！", id)))?;

        let row_vo = EquipmentInstallationVo::from(row);
        Ok(row_vo)
    }
    //  add 方法 , EquipmentInstallation::insert
    pub async fn add(&self, arg: &EquipmentInstallation) -> Result<u64> {
        //let old = EquipmentInstallation::select_by_map(
        //pool!(),
        // value!{
        //arg.id.as_deref().unwrap_or_default(),
        //)
        //.await?;
        let result = Ok(EquipmentInstallation::insert(pool!(), &arg)
            .await?
            .rows_affected);
        return result;
    }
    //  update 方法 , EquipmentInstallation::

    pub async fn update(&self, data: EquipmentInstallation) -> Result<u64> {
        // 假设你的主键字段名是 id
        // let result = EquipmentInstallation::update_by_column(pool!(), &data, "id").await?;
        let result = EquipmentInstallation::update_by_map(
            pool!(),
            &data,
            value! {"id": data.id.unwrap_or_default()},
        )
        .await?;
        return Ok(result.rows_affected);
    }
    //  remove 方法 , EquipmentInstallation::delete_by_column 并且调用sys_trash_service.add 保存到回收站表
    pub async fn remove(&self, id: &str) -> Result<u64> {
        let id_num: i64 = id.parse()
            .map_err(|_| Error::from(format!("id 必须是纯数字: {}", id)))?;
        let targets = EquipmentInstallation::select_by_map(pool!(), value! {"id":  id_num }).await?;
        let r = EquipmentInstallation::delete_by_map(pool!(), value! {"id":  id_num }).await?;
        if r.rows_affected > 0 {
            //copy data to trash
            CONTEXT
                .sys_trash_service
                .add("equipment_installation", &targets)
                .await?;
        }
        Ok(r.rows_affected)
    }

    pub async fn export_template(&self) -> Result<Vec<u8>> {
        let mut book = umya_spreadsheet::new_file();

        let sheet = book.get_sheet_mut(&0).unwrap();
        sheet.set_name("设备安装");

        let headers = vec![
            ("A1", "设备编号"),
            ("B1", "设备名称"),
            ("C1", "设备类型"),
            ("D1", "设备型号标签"),
            ("E1", "设备状态"),
            ("F1", "通信方式"),
            ("G1", "地址"),
            ("H1", "位置ID"),
            ("I1", "下级设备"),
            ("J1", "项目编号"),
        ];

        for (coord, header) in headers {
            sheet.get_cell_mut(coord).set_value(header);
        }

        let temp_path = std::env::temp_dir().join(format!("template_{}.xlsx", uuid::Uuid::new_v4().to_string()));
        umya_spreadsheet::writer::xlsx::write(&book, &temp_path).map_err(|e| Error::from(format!("导出Excel失败: {}", e)))?;
        
        let output = std::fs::read(&temp_path).map_err(|e| Error::from(format!("读取临时文件失败: {}", e)))?;
        std::fs::remove_file(&temp_path).ok();
        
        Ok(output)
    }

    pub async fn import_excel(&self, bytes: &[u8]) -> Result<u64> {
        let temp_path: String = std::env::temp_dir().join(format!("import_{}.xlsx", uuid::Uuid::new_v4().to_string())).to_string_lossy().into_owned();
        let mut file = std::fs::File::create(&temp_path).map_err(|e| Error::from(format!("创建临时文件失败: {}", e)))?;
        file.write_all(bytes).map_err(|e| Error::from(format!("写入临时文件失败: {}", e)))?;
        file.sync_all().map_err(|e| Error::from(format!("同步文件失败: {}", e)))?;

        let book = umya_spreadsheet::reader::xlsx::read(&temp_path)
            .map_err(|e| Error::from(format!("读取Excel失败: {}", e)))?;

        std::fs::remove_file(&temp_path).ok();

        let sheet = book.get_sheet(&0)
            .ok_or_else(|| Error::from("工作表不存在"))?;

        let mut imported_count = 0u64;

        let mut row = 2u32;
        loop {
            let mut has_data = false;
            let mut cells: Vec<String> = Vec::with_capacity(10);
            
            let col_names = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
            for (i, col_name) in col_names.iter().enumerate() {
                let coord = format!("{}{}", col_name, row);
                let val = sheet.get_value(coord);
                if !val.is_empty() {
                    has_data = true;
                }
                cells.push(val);
            }

            if !has_data {
                break;
            }

            let import_dto = EquipmentInstallationImportDTO {
                equipment_number: if cells[0].is_empty() { None } else { Some(cells[0].clone()) },
                equipment_name: if cells[1].is_empty() { None } else { Some(cells[1].clone()) },
                equipment_type: if cells[2].is_empty() { None } else { Some(cells[2].clone()) },
                equipment_model_label: if cells[3].is_empty() { None } else { Some(cells[3].clone()) },
                equipment_status: if cells[4].is_empty() { None } else { Some(cells[4].clone()) },
                communication_means: if cells[5].is_empty() { None } else { Some(cells[5].clone()) },
                addr: if cells[6].is_empty() { None } else { Some(cells[6].clone()) },
                location_id: if cells[7].is_empty() { None } else { Some(cells[7].clone()) },
                lower_equipment: if cells[8].is_empty() { None } else { Some(cells[8].clone()) },
                project_num: if cells[9].is_empty() { None } else { Some(cells[9].clone()) },
            };

            let data = EquipmentInstallation::from(import_dto);
            EquipmentInstallation::insert(pool!(), &data).await
                .map_err(|e| Error::from(format!("插入数据失败: {}", e)))?;

            imported_count += 1;
            row += 1;
        }

        Ok(imported_count)
    }
}
