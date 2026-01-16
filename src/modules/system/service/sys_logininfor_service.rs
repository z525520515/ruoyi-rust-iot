use rbatis::plugin::page::Page;
use rbatis::plugin::page::PageRequest;

use crate::error::Result;
use crate::modules::system::domain::dto::logininfor::LogininforPageDTO;
use crate::modules::system::domain::table::SysLogininfor;
use crate::modules::system::domain::vo::SysLogininforVO;
use crate::pool;

use rbs::value;
/// dictionary service
pub struct SysLogininforService {}

impl SysLogininforService {
    pub async fn page(&self, arg: &LogininforPageDTO) -> Result<Page<SysLogininforVO>> {
        let page_req = &PageRequest::from(arg); // PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10));
        let limitstr = format!(
            " limit {} offset {}",
            page_req.page_size,
            (page_req.page_no - 1) * page_req.page_size
        );
        let linits = &limitstr;
        dbg!(arg);
        let data = SysLogininfor::select_page(pool!(), page_req, linits, arg).await?;
        let page = Page::<SysLogininforVO>::from(data);
        Ok(page)
    }

    //异步加入日志
    pub async fn add_async(&self, arg: &SysLogininfor) -> Result<u64> {
        let info = arg.to_owned();
        tokio::spawn(async move {
            SysLogininfor::insert(pool!(), &info).await;
        });
        return Ok(1);
    }

    pub async fn remove(&self, info_id: &str) -> Result<u64> {
        let r = SysLogininfor::delete_by_map(pool!(), value! {"info_id": info_id}).await?;
        Ok(r.rows_affected)
    }
}
