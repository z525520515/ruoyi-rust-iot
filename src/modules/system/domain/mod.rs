use crate::modules::system::service::ApplicationConfig;
pub mod dto;
pub mod mapper;
pub mod table;
pub mod vo;

pub fn init_rbatis(config: &ApplicationConfig) -> rbatis::RBatis {
    let rbatis = rbatis::RBatis::new();
    if rbatis.is_debug_mode() == false  {
        dbg!("rbatis 已使用release模式运行"
        );
    }
    if config.debug.eq(&true) {
        dbg!("根据 application.yml 已使用debug模式运行");
        
    }else{
        dbg!("根据 application.yml 已使用release模式运行");
    }
    return rbatis;
}
