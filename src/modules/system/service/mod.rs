mod cache_service;
mod mem_service;
mod redis_service;

mod sys_config_service;
mod sys_dept_service;
mod sys_dict_data_service;
mod sys_dict_type_service;
mod sys_logininfor_service;
mod sys_menu_service;
mod sys_notice_service;
mod sys_post_service;
mod sys_role_menu_service;
mod sys_role_service;
mod sys_sms_service;
mod sys_trash_service;
mod sys_user_role_service;
mod sys_user_service;

pub use crate::config::config::ApplicationConfig;
pub use cache_service::*;
pub use mem_service::*;
use once_cell::sync::Lazy;
use rbdc_pg::driver::PgDriver;
// use rbatis::rbatis::Rbatis;
// use rbdc_mysql::driver::MysqlDriver;
use super::super::equipment::service::equipmentinstallation_service::EquipmentInstallationService;
pub use redis_service::*;
pub use sys_config_service::*;
pub use sys_dept_service::*;
pub use sys_dict_data_service::*;
pub use sys_dict_type_service::*;
pub use sys_logininfor_service::*;
pub use sys_menu_service::*;
pub use sys_notice_service::*;
pub use sys_post_service::*;
pub use sys_role_menu_service::*;
pub use sys_role_service::*;
pub use sys_sms_service::*;
pub use sys_trash_service::*;
pub use sys_user_role_service::*;
pub use sys_user_service::*;

use chrono::{Datelike, Local};
/// CONTEXT is all of the service struct
pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());

#[macro_export]
macro_rules! pool {
    () => {
        &mut crate::modules::system::service::CONTEXT.rb.clone()
    };
}

#[macro_export]
macro_rules! get_config_value {
    ($arg:expr) => {
        $crate::modules::system::service::CONTEXT
            .sys_config_service
            .select_config_by_key($arg)
            .await
            .unwrap_or_default()
    };
}

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rb: rbatis::RBatis,
    pub cache_service: CacheService,
    pub sys_menu_service: SysMenuService,
    pub sys_user_service: SysUserService,
    pub sys_role_service: SysRoleService,
    pub sys_role_menu_service: SysRoleMenuService,
    pub sys_user_role_service: SysUserRoleService,
    pub sys_dict_type_service: SysDictTypeService,
    pub sys_dict_data_service: SysDictDataService,
    pub sys_config_service: SysConfigService,
    pub sys_dept_service: SysDeptService,
    pub sys_trash_service: SysTrashService,
    pub sys_logininfor_service: SysLogininforService,
    pub sys_post_service: SysPostService,
    pub sys_notice_service: SysNoticeService,
    pub char_array: [char; 52],
    pub equipment_installation_service: EquipmentInstallationService,
}

impl ServiceContext {
    /// init database pool
    pub async fn init_pool(&self) {
        //连接数据库
        println!(
            "[ruoyi_rust] rbatis pool init ({})...",
            self.config.database_url
        );
        self.rb
            .init(PgDriver {}, &self.config.database_url)
            .expect("[ruoyi_rust] rbatis pool init fail!");

        self.rb.get_pool().unwrap().set_max_open_conns(10).await;
        log::info!(
            "[ruoyi_rust] rbatis pool init success! pool state = {:?}",
            self.rb.get_pool().expect("pool not init!").state().await
        );
        //self.rb.get_pool().unwrap().;
        println!("Local:  http://{}", self.config.server_url);
        //println!("Local:  http://{}",self.config.server_url.replace("0.0.0.0", "127.0.0.1"));
    }

    // 获取业务流水号
    pub async fn get_table_maxid(idfield: &str, table_name: &str) -> i64 {
        let local = Local::now();
        let year = local.year();
        let month = local.month();
        let day = local.day();
        let prefix = format!("{:04}{:02}{:02}", year, month, day);

        let query_perms = format!(
            "select max({}) from {} where CAST({} AS TEXT)  like '{}%'",
            idfield, table_name, idfield, prefix
        );
        let max_id: i64 = pool!()
            .query_decode(&query_perms, vec![])
            .await
            .unwrap_or(0);

        let mut newid: i64 = 0;
        if max_id <= 0 {
            let new_str = format!("{:04}{:02}{:02}{:05}", year, month, day, 1);
            let s = new_str.to_string();
            //字符串转32位有符号整型
            newid = s.parse::<i64>().unwrap();
        } else {
            newid = max_id + 1;
        }
        newid.into()
    }
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        let str = "abcdefghkmnopqrstuwxyz23456789ABCDEFGHKMNOPQRSTUWXYZ";
        let char_array1: Vec<char> = str.chars().collect();
        let char_array2: [char; 52] = char_array1
            .into_iter()
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();

        ServiceContext {
            rb: crate::modules::system::domain::init_rbatis(&config),
            cache_service: CacheService::new(&config).unwrap(),
            config,
            sys_menu_service: SysMenuService {},
            sys_user_service: SysUserService {},
            sys_role_service: SysRoleService {},
            sys_role_menu_service: SysRoleMenuService {},
            sys_user_role_service: SysUserRoleService {},
            sys_dict_type_service: SysDictTypeService {},
            sys_dict_data_service: SysDictDataService {},
            sys_config_service: SysConfigService {},
            sys_dept_service: SysDeptService {},
            sys_trash_service: SysTrashService {},
            sys_logininfor_service: SysLogininforService {},
            sys_post_service: SysPostService {},
            sys_notice_service: SysNoticeService {},
            char_array: char_array2,
            equipment_installation_service: EquipmentInstallationService {},
        }
    }
}
