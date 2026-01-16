use actix_web::{get, HttpRequest, post, put, Responder, web};


use crate::config::global_variables::{ADMIN_NAME};
use crate::modules::system::domain::dto::user::{ UserAddDTO, UserPageDTO, UserRoleAuthQueryDTO, UserUpdateDTO};
use crate::modules::system::domain::table::SysUser;
use crate::modules::system::domain::vo::{ PageVO, RespJson, RespVO};
use crate::modules::system::service::CONTEXT;
use crate::web_data::get_user_name;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct CallbackInfo {
    code: Option<String>,
	state: Option<String>,
}

#[get("/wecallback")]
pub async fn wecallback(info: web::Query<CallbackInfo>) -> impl Responder {
    let arg: &CallbackInfo = &info.0;
	let code = arg.code.clone().unwrap_or("".to_string()) ;
    let state = arg.state.clone().unwrap_or("".to_string()) ;
    let res:CallbackInfo=CallbackInfo{
        code: Some(code ),
        state: Some(state ),
    };

    return RespVO::from_result(&Ok( res)).resp_json();
}