use rbatis::plugin::page::PageRequest;
use rbatis::plugin::page::Page;
// 引用 数据表相关对象
use crate::modules::equipment::domain::table::tables::EquipmentInstallation;
use crate::modules::equipment::domain::dto::equipment_installation::{EquipmentInstallationAddDTO, EquipmentInstallationUpdateDTO, EquipmentInstallationPageDTO};
use crate::modules::equipment::domain::vo::equipmentinstallationvo::EquipmentInstallationVo;
use crate::modules::system::service::CONTEXT;
//  
use crate::error::Error;
use crate::error::Result;
// pool 数据库 
use crate::pool;
use rbs::{value};
/// equipment_installation service
pub struct EquipmentInstallationService {}
impl EquipmentInstallationService {
    // 分页查询方法 传入EquipmentInstallationPageDTO,返回 Page<EquipmentInstallationVo>
    pub async fn page(&self, arg: &EquipmentInstallationPageDTO) -> Result<Page<EquipmentInstallationVo>> {
        let page_req =&PageRequest::from(arg);
        // 由于 使用的 postgresql 所以 limit 语句要从新按postgresql的语法注入参数
        let limitstr=format!(" limit {} offset {}", page_req.page_size,(page_req.page_no-1)*page_req.page_size);
        let linits=&limitstr;
        let data = EquipmentInstallation::select_page(
            pool!(),
            page_req,
            linits,
            arg
        )
            .await?;
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
        let row = EquipmentInstallation::select_by_map(pool!(), value!{
        "id":  id })
            .await?
            .into_iter()
            .next().ok_or_else(|| Error::from(format!("不存在 {:?} 不存在！", id)))?;
        let row_vo = EquipmentInstallationVo::from(row);
        return Ok(row_vo);
    }
    //  add 方法 , EquipmentInstallation::insert 
    pub async fn add(&self, arg: &EquipmentInstallation) -> Result<u64> {
        //let old = EquipmentInstallation::select_by_map(
            //pool!(),
            // value!{  
            //arg.id.as_deref().unwrap_or_default(),
        //)
            //.await?;
        let result = Ok(EquipmentInstallation::insert(pool!(), &arg).await?.rows_affected);
        return result;
    }
    //  update 方法 , EquipmentInstallation::update_by_column 
    pub async fn update(&self, data: EquipmentInstallation) -> Result<u64> {
        // 假设你的主键字段名是 id
        let result = EquipmentInstallation::update_by_column(pool!(), &data, "id").await?;
        return Ok(result.rows_affected);
    }
    //  remove 方法 , EquipmentInstallation::delete_by_column 并且调用sys_trash_service.add 保存到回收站表
    pub async fn remove(&self, id: &str) -> Result<u64> {
        let targets = EquipmentInstallation::select_by_map(pool!(), value!{"id":  id }).await?;
        let r = EquipmentInstallation::delete_by_map(pool!(),  value!{"id":  id }).await?;
        if r.rows_affected > 0 {
            //copy data to trash
            CONTEXT.sys_trash_service.add("equipment_installation", &targets).await?;
        }
        Ok(r.rows_affected)
    }
}
