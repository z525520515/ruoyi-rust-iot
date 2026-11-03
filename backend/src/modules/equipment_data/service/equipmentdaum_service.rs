use rbatis::plugin::page::Page;
use rbatis::plugin::page::PageRequest;
// 引用 数据表相关对象
use crate::modules::equipment_data::domain::dto::equipment_data::EquipmentDaumPageDTO;
use crate::modules::equipment_data::domain::table::tables::EquipmentData;
use crate::modules::equipment_data::domain::vo::equipmentdaumvo::EquipmentDaumVo;
use crate::modules::system::service::CONTEXT;
//
use crate::error::Error;
use crate::error::Result;
// pool 数据库
use crate::pool;
use rbs::value;
/// equipment_data service
pub struct EquipmentDaumService {}
impl EquipmentDaumService {
    // 分页查询方法 传入EquipmentDaumPageDTO,返回 Page<EquipmentDaumVo>
    pub async fn page(&self, arg: &EquipmentDaumPageDTO) -> Result<Page<EquipmentDaumVo>> {
        let page_req = PageRequest::from(arg);
        let mut where_sql = String::from(" where 1=1");
        let mut args: Vec<rbs::Value> = Vec::new();

        // 普通查询条件
        crate::util::data_scope::push_like(
            &mut where_sql,
            &mut args,
            "equipment_number",
            &arg.equipment_number,
        );
        crate::util::data_scope::push_like(
            &mut where_sql,
            &mut args,
            "equipment_name",
            &arg.equipment_name,
        );
        crate::util::data_scope::push_eq_str(
            &mut where_sql,
            &mut args,
            "device_category",
            &arg.device_category,
        );
        crate::util::data_scope::push_eq_i32(
            &mut where_sql,
            &mut args,
            "alarm_status",
            &arg.alarm_status,
        );

        // 部门筛选（含下级部门）
        if let Some(dept_id) = arg.dept_id {
            let ids = crate::util::data_scope::dept_and_children(dept_id as i64).await?;
            crate::util::data_scope::push_in_clause(&mut where_sql, &mut args, "dept_id", &ids);
        }

        // 数据权限
        crate::util::data_scope::apply_scope_dept_only(&mut where_sql, &mut args, "dept_id").await?;

        let count_sql = format!("select count(1) from equipment_data{}", where_sql);
        let total: u64 = pool!().query_decode(&count_sql, args.clone()).await?;

        let data_sql = format!(
            "select * from equipment_data{} order by upload_time desc limit ? offset ?",
            where_sql
        );
        let mut data_args = args;
        data_args.push(value!(page_req.page_size as i64));
        data_args.push(value!(
            ((page_req.page_no.saturating_sub(1)) * page_req.page_size) as i64
        ));
        let rows: Vec<EquipmentData> = pool!().query_decode(&data_sql, data_args).await?;

        let dept_map = crate::util::data_scope::dept_name_map().await?;
        let records: Vec<EquipmentDaumVo> = rows
            .into_iter()
            .map(|row| {
                let mut vo = EquipmentDaumVo::from(row);
                if let Some(dept_id) = vo.dept_id {
                    vo.dept_name = dept_map.get(&dept_id.to_string()).cloned();
                }
                vo
            })
            .collect();

        let mut page = Page::new_total(page_req.page_no, page_req.page_size, total);
        page.records = records;
        Ok(page)
    }
    // 调用表实体对象 EquipmentDaum 的select_all方法 ,返回 Page<EquipmentDaumVo>
    pub async fn finds_all(&self) -> Result<Vec<EquipmentDaumVo>> {
        let data = EquipmentData::select_all(pool!()).await?;
        let mut post_vos = vec![];
        for s in data {
            post_vos.push(EquipmentDaumVo::from(s));
        }
        Ok(post_vos)
    }
    // 调用表实体对象 EquipmentDaum select_by_map ,根据id返回 EquipmentDaumVo
    pub async fn detail(&self, id: &str) -> Result<EquipmentDaumVo> {
        let row = EquipmentData::select_by_map(
            pool!(),
            value! {
            "id":  id },
        )
        .await?
        .into_iter()
        .next()
        .ok_or_else(|| Error::from(format!("不存在 {:?} 不存在！", id)))?;
        let mut row_vo = EquipmentDaumVo::from(row);
        // 数据权限：校验当前用户是否可以访问该设备数据
        crate::util::data_scope::check_dept_scope(row_vo.dept_id.map(|d| d as i64)).await?;
        if let Some(dept_id) = row_vo.dept_id {
            let dept_map = crate::util::data_scope::dept_name_map().await?;
            row_vo.dept_name = dept_map.get(&dept_id.to_string()).cloned();
        }
        return Ok(row_vo);
    }
    //  add 方法 , EquipmentDaum::insert
    pub async fn add(&self, arg: &EquipmentData) -> Result<u64> {
        //let old = EquipmentDaum::select_by_map(
        //pool!(),
        // value!{
        //arg.id.as_deref().unwrap_or_default(),
        //)
        //.await?;
        let result = Ok(EquipmentData::insert(pool!(), &arg).await?.rows_affected);
        return result;
    }
    //  update 方法 , EquipmentDaum::update_by_column
    pub async fn update(&self, data: EquipmentData) -> Result<u64> {
        let id = data.id.unwrap_or_default();
        // 数据权限：校验当前用户是否可以修改该设备数据
        let old = EquipmentData::select_by_map(pool!(), value! {"id": id})
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from(format!("不存在 {:?} ！", id)))?;
        crate::util::data_scope::check_dept_scope(old.dept_id.map(|d| d as i64)).await?;
        let result = EquipmentData::update_by_map(
            pool!(),
            &data,
            value! {"id": id},
        )
        .await;
        return Ok(result?.rows_affected);
    }
    //  remove 方法 , EquipmentDaum::delete_by_column 并且调用sys_trash_service.add 保存到回收站表
    pub async fn remove(&self, id: &str) -> Result<u64> {
        let targets: Vec<EquipmentData> = EquipmentData::select_by_map(pool!(), value! {"id": id}).await?;
        // 数据权限：校验当前用户是否可以删除该设备数据
        for row in &targets {
            crate::util::data_scope::check_dept_scope(row.dept_id.map(|d| d as i64)).await?;
        }
        let r = EquipmentData::delete_by_map(pool!(), value! {"id":  id }).await?;
        if r.rows_affected > 0 {
            //copy data to trash
            CONTEXT
                .sys_trash_service
                .add("equipment_data", &targets)
                .await?;
        }
        Ok(r.rows_affected)
    }
    pub async fn get_status_summary(&self) -> Result<serde_json::Value> {
        let mut where_sql = String::from(" where 1=1");
        let mut args: Vec<rbs::Value> = Vec::new();
        crate::util::data_scope::apply_scope_dept_only(&mut where_sql, &mut args, "dept_id").await?;
        let total: i64 = pool!()
            .query_decode(
                &format!("select count(*) from view_device_status{}", where_sql),
                args.clone(),
            )
            .await?;
        let online_where = format!("{} and online_state_calc = '在线'", where_sql);
        let online: i64 = pool!()
            .query_decode(
                &format!("select count(*) from view_device_status{}", online_where),
                args,
            )
            .await?;
        Ok(serde_json::json!({
            "total": total,
            "online": online,
            "offline": total - online
        }))
    }
}
