use std::collections::HashMap;
use rbatis::plugin::page::PageRequest;
use rbatis::plugin::page::Page;

use crate::modules::system::domain::dto::dict_data::{ DictDataPageDTO};
use crate::modules::system::domain::table::SysDictData;
use crate::modules::system::domain::vo::{SysDictDataSimpleVO, SysDictDataVO};
use crate::error::Error;
use crate::error::Result;
use crate::pool;
// use rbdc_pg::driver::PgDriver;
use crate::modules::system::service::CONTEXT;
use rbs::{value};
//const DICT_KEY: &'static str = "sys_dict_data:all";

/// dictionary service
pub struct SysDictDataService {}

impl SysDictDataService {
    pub async fn page(&self, arg: &DictDataPageDTO) -> Result<Page<SysDictDataVO>> {
        let page_req =&PageRequest::from(arg);// PageRequest::new(arg.page_no.unwrap_or(1), arg.page_size.unwrap_or(10));
        let limitstr=format!(" limit {} offset {}", page_req.page_size,(page_req.page_no-1)*page_req.page_size);
        let linits=&limitstr;
        dbg!(arg);
        let data = SysDictData::select_page(pool!(), page_req, linits,arg).await?;
        let page = Page::<SysDictDataVO>::from(data);
        Ok(page)
    }

    pub async fn get_by_dict_type(&self, dict_type: &String) -> Result<Vec<SysDictDataSimpleVO>> {
        let data = SysDictData::select_by_dict_type(pool!(), &dict_type).await?;
        let mut res = vec![];
        for d in data {
            res.push(SysDictDataSimpleVO::from(d))
        }
        Ok(res)
    }

    pub async fn detail(&self, dict_code: &str) -> Result<SysDictDataVO> {
        let dict_data = SysDictData::select_by_map(pool!(), value! {"dict_code": dict_code})
            .await?
            .into_iter()
            .next().ok_or_else(|| Error::from(format!("不存在:{:?} 不存在！", dict_code)))?;
        let dict_data_vo = SysDictDataVO::from(dict_data);
        return Ok(dict_data_vo);
    }

    pub async fn add(&self, arg: &SysDictData) -> Result<u64> {
        let old = SysDictData::select_by_map(
            pool!(),
            value! {"dict_code":
            arg.dict_code.as_deref().unwrap_or_default()}
        )
            .await?;
        // if old.len() > 0 {
        //     return Err(Error::from(format!(
        //         "字典已存在! code={}",
        //         arg.code.as_deref().unwrap_or_default()
        //     )));
        // }
        let result = Ok(SysDictData::insert(pool!(), &arg).await?.rows_affected);
        self.update_cache().await?;
        return result;
    }

    pub async fn update(&self, data: SysDictData) -> Result<u64> {

        let result = SysDictData::update_by_map(pool!(), &data, value! {"dict_code":
            data.dict_code.as_deref().unwrap_or_default()}).await;

        // let result = SysDictData::update_by_map(pool!(), &data, "dict_code").await;
        if result.is_ok() {
            self.update_cache().await?;
        }
        return Ok(result?.rows_affected);
    }

    pub async fn remove(&self, dict_code: &str) -> Result<u64> {
        let targets = SysDictData::select_by_map(pool!(), value! {"dict_code", dict_code}).await?;

        let r = SysDictData::delete_by_map(pool!(), value! {"dict_code", dict_code}).await?;
        if r.rows_affected > 0 {
            self.update_cache().await?;
            //copy data to trash
            CONTEXT.sys_trash_service.add("sys_dict_data", &targets).await?;
        }
        Ok(r.rows_affected)
    }

    /// update for all cache
    pub async fn update_cache(&self) -> Result<()> {

        // let rbatis = rbatis::RBatis::new();
        // let connect_str = &CONTEXT.config.database_url;

        // rbatis.init(PgDriver {}, connect_str)
        //     .expect("[ruoyi_rust] rbatis pool init fail!");

        //     rbatis.get_pool().unwrap().set_max_open_conns(10).await;

        // let table: std::result::Result<SysDictData, rbs::Error> = rbatis
        // .query_decode("select * from sys_dict_data limit ?", vec![rbs::to_value!(1)])
        // .await
        // ;

        // println!(">>>>> table={:?}", table);

        let mut all: Vec<SysDictData> = SysDictData::select_all(pool!()).await?;
        all.sort_by(|a, b| a.dict_sort.cmp(&b.dict_sort));
        let mut dict_data_map: HashMap<&str, Vec<SysDictDataSimpleVO>> = HashMap::new();
        for dict_data in all {
            let key = dict_data.dict_type.clone().as_deref().unwrap();
            let key = "1233";//fixme
            let data_sim = SysDictDataSimpleVO::from(dict_data);
            if dict_data_map.contains_key(key) {
                dict_data_map.get_mut(key).unwrap().push(data_sim);
            } else {
                dict_data_map.insert(key, vec![data_sim]);
            }
        }
        //  CONTEXT.cache_service.set_json(DICT_KEY, &dict_data_map).await?;
        return Ok(());
    }
}
