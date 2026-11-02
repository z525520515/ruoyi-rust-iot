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
        let page_req = &PageRequest::from(arg);
        let limitstr = format!(
            " limit {} offset {}",
            page_req.page_size,
            (page_req.page_no - 1) * page_req.page_size
        );
        let linits = &limitstr;
        let data = EquipmentData::select_page(pool!(), page_req, linits, arg).await?;
        let page = Page::<EquipmentDaumVo>::from(data);
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
        let row_vo = EquipmentDaumVo::from(row);
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
        let result = EquipmentData::update_by_map(
            pool!(),
            &data,
            value! {"id": data.id.unwrap_or_default()},
        )
        .await;
        return Ok(result?.rows_affected);
    }
    //  remove 方法 , EquipmentDaum::delete_by_column 并且调用sys_trash_service.add 保存到回收站表
    pub async fn remove(&self, id: &str) -> Result<u64> {
        let targets: Vec<EquipmentData> = EquipmentData::select_by_map(pool!(), value! {"id": id}).await?;
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
        let total: i64 = pool!()
            .query_decode("select count(*) from view_device_status", vec![])
            .await?;
        let online: i64 = pool!()
            .query_decode(
                "select count(*) from view_device_status where online_state_calc = '在线'",
                vec![],
            )
            .await?;
        Ok(serde_json::json!({
            "total": total,
            "online": online,
            "offline": total - online
        }))
    }
}
