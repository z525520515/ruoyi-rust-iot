
use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::dev::Server;
use actix_web_validator::{Error, JsonConfig};
use actix_web_validator::error::flatten_errors;


use crate::modules::system::controller::{img_controller, sys_auth_controller, sys_config_controller, sys_dept_controller, sys_dict_data_controller, sys_dict_type_controller, sys_menu_controller, sys_notice_controller, sys_post_controller, sys_profile_controller, sys_role_controller, sys_user_controller };
use crate::modules::system::service::CONTEXT;
use crate::modules::system::controller::sys_file_controller; //文件上传下载控制器
use crate::modules::system::controller::system_websocket;    //websocket控制器
use crate::modules::system::controller::monitor::{server_controller, sys_logininfor_controller, sys_user_online_controller};
use crate::modules::system::controller::wx::{wx_controller};
use crate::modules::system::domain::vo::RespVO;
use crate::modules::equipment::controller::{equipmentinstallation_controller};
use crate::modules::equipment_data::controller::equipmentdaum_controller;
use crate::modules::gateway::controller::gateway_controller;

// #[get("/")]
// async fn index() -> impl Responder {
//     NamedFile::open_async("../dist/index.html").await.unwrap()
// }
//
// #[get("/")]
// async fn fav() -> impl Responder {
//     NamedFile::open_async("../dist/f").await.unwrap()
// }

pub fn build_server(base_api: &'static str) -> Server {
    HttpServer::new(|| {
        //定义validate 错误的返回json
        //目前只支持Json，form query path需要再写，目前用不上
        let json_config = JsonConfig::default().error_handler(|err, _req| {
            match err {
                Error::Validate(e) => {
                    let err_str = flatten_errors(&e).iter()
                        .map(|(_, field, err)| { format!("{}", err) })
                        .collect::<Vec<_>>()
                        .join("\n");
                    let resp: RespVO<String> = RespVO {
                        code: 500,
                        msg: Some(err_str),
                        data: None,
                    };
                    actix_web::error::InternalError::from_response(e, resp.resp_json()).into()
                }
                _ => {
                    actix_web::error::InternalError::from_response(err, HttpResponse::BadRequest().finish()).into()
                }
            }
        });
        let base_api = base_api.to_string();
        App::new()
            //  .wrap(Auth {})
            .app_data(json_config)
            //.app_data(web::Data::new(ConnectionPool::new(100)))
            .app_data(web::PayloadConfig::new(50 * 1024 * 1024))

            // .service(index)
            //.service(fs::Files::new("/assets", "../dist/assets").show_files_listing())
            .service(web::scope(&base_api).service(
                web::scope("/system") //系统应用
                    .service(sys_menu_controller::role_menu_treeselect)
                    .service(sys_menu_controller::treeselect)
                    .service(sys_menu_controller::query_menu)
                    .service(sys_menu_controller::detail)
                    .service(sys_menu_controller::add)
                    .service(sys_menu_controller::update)
                    .service(sys_menu_controller::remove)

                    .service(sys_file_controller::upload_document)
                    .service(sys_file_controller::upload_form)

                    .service(sys_profile_controller::profile)
                    .service(sys_profile_controller::update_profile)
                    .service(sys_profile_controller::update_pwd)

                    .service(sys_user_controller::page)
                    .service(sys_user_controller::get_dept_tree)
                    .service(sys_user_controller::detail)
                    .service(sys_user_controller::add)
                    .service(sys_user_controller::update)
                    .service(sys_user_controller::remove)
                    .service(sys_user_controller::set_auth_roles)
                    .service(sys_user_controller::before_add)
                    .service(sys_user_controller::get_auth_roles)
                    .service(sys_user_controller::change_status)


                    .service(sys_role_controller::page)
                    .service(sys_role_controller::detail)
                    .service(sys_role_controller::add)
                    .service(sys_role_controller::update)
                    .service(sys_role_controller::remove)
                    .service(sys_role_controller::auth_user_list)
                    .service(sys_role_controller::unallocated_user_list)
                    .service(sys_role_controller::change_status)
                    .service(sys_role_controller::cancel_user)
                    .service(sys_role_controller::auth_user_all)
                    .service(sys_role_controller::cancel_user_all)


                    .service(sys_dict_type_controller::optionselect)
                    .service(sys_dict_type_controller::page)
                    .service(sys_dict_type_controller::detail)
                    .service(sys_dict_type_controller::add)
                    .service(sys_dict_type_controller::update)
                    .service(sys_dict_type_controller::remove)

                    .service(sys_dict_data_controller::page)
                    .service(sys_dict_data_controller::detail)
                    .service(sys_dict_data_controller::add)
                    .service(sys_dict_data_controller::update)
                    .service(sys_dict_data_controller::remove)
                    .service(sys_dict_data_controller::get_by_dict_type)

                    .service(sys_config_controller::page)
                    .service(sys_config_controller::detail)
                    .service(sys_config_controller::add)
                    .service(sys_config_controller::update)
                    .service(sys_config_controller::remove)

                    .service(sys_dept_controller::page)
                    .service(sys_dept_controller::detail)
                    .service(sys_dept_controller::add)
                    .service(sys_dept_controller::update)
                    .service(sys_dept_controller::remove)

                    .service(sys_post_controller::page)
                    .service(sys_post_controller::detail)
                    .service(sys_post_controller::add)
                    .service(sys_post_controller::update)
                    .service(sys_post_controller::remove)

                    .service(sys_notice_controller::page)
                    .service(sys_notice_controller::detail)
                    .service(sys_notice_controller::add)
                    .service(sys_notice_controller::update)
                    .service(sys_notice_controller::remove)
            ).service(
                web::scope("/monitor") //系统应
                    .service(sys_logininfor_controller::page)
                    .service(sys_logininfor_controller::remove)


                    .service(sys_user_online_controller::page)
                    .service(sys_user_online_controller::force_logout)

                    .service(server_controller::server_info)
            ).service(
                web::scope("/equipment")
                    .service(equipmentinstallation_controller::page)
                    .service(equipmentinstallation_controller::detail)
                    .service(equipmentinstallation_controller::add)
                    .service(equipmentinstallation_controller::update)
                    .service(equipmentinstallation_controller::remove)
                    .service(equipmentinstallation_controller::template_export)
                    .service(equipmentinstallation_controller::import_excel)
                    .service(equipmentinstallation_controller::status_list)
            ).service(
                web::scope("/equipment_data")
                    .service(equipmentdaum_controller::page)
                    .service(equipmentdaum_controller::detail)
                    .service(equipmentdaum_controller::add)
                    .service(equipmentdaum_controller::update)
                    .service(equipmentdaum_controller::remove)
                    .service(equipmentdaum_controller::stats)
            ).service(
                web::scope("/gateway")
                    // 通用网关调用接口
                    .service(gateway_controller::gateway_invoke)
                    // System 模块接口
                    .service(gateway_controller::system_device_unique_id)
                    .service(gateway_controller::system_device_unique_id_hash)
                    .service(gateway_controller::system_info)
                    .service(gateway_controller::system_change_password)
                    .service(gateway_controller::system_application_info)
                    .service(gateway_controller::system_iccid)
                    .service(gateway_controller::system_set_time)
                    .service(gateway_controller::system_set_timezone)
                    .service(gateway_controller::system_available_timezones)
                    .service(gateway_controller::system_current_timezone)
                    .service(gateway_controller::system_reboot)
                    .service(gateway_controller::system_device_unique_id_hash_by_sn)
                    // Serial Port Manager 模块接口
                    .service(gateway_controller::serial_available_ports)
                    .service(gateway_controller::serial_open_port)
                    .service(gateway_controller::serial_close_port)
                    .service(gateway_controller::serial_set_baudrate)
                    .service(gateway_controller::serial_available_databits)
                    .service(gateway_controller::serial_available_parities)
                    .service(gateway_controller::serial_available_stopbits)
                    .service(gateway_controller::serial_available_flowcontrols)
                    .service(gateway_controller::serial_set_databits)
                    .service(gateway_controller::serial_set_parity)
                    .service(gateway_controller::serial_set_stopbits)
                    .service(gateway_controller::serial_set_flowcontrol)
                    // Logger
                    .service(gateway_controller::logger_available_levels)
                    .service(gateway_controller::logger_open_channel)
                    .service(gateway_controller::logger_close_channel)
                    .service(gateway_controller::logger_global_level)
                    .service(gateway_controller::logger_set_global_level)
                    .service(gateway_controller::logger_max_file_size)
                    .service(gateway_controller::logger_set_max_file_size)
                    // Maintenance Tool
                    .service(gateway_controller::maintenance_add_repository)
                    .service(gateway_controller::maintenance_add_temp_repository)
                    .service(gateway_controller::maintenance_check_updates)
                    .service(gateway_controller::maintenance_update)
                    // Network Interface Manager
                    .service(gateway_controller::network_all_interfaces)
                    .service(gateway_controller::network_set_interface_up)
                    .service(gateway_controller::network_set_interface_down)
                    .service(gateway_controller::network_enable_dhcp)
                    .service(gateway_controller::network_disable_dhcp)
                    .service(gateway_controller::network_add_address)
                    .service(gateway_controller::network_remove_address)
                    .service(gateway_controller::network_set_address)
                    // TCP Client Manager
                    .service(gateway_controller::tcp_available_clients)
                    .service(gateway_controller::tcp_register_client)
                    .service(gateway_controller::tcp_unregister_client)
                    .service(gateway_controller::tcp_set_client_reconnect)
                    // MQTT Client Manager
                    .service(gateway_controller::mqtt_available_clients)
                    .service(gateway_controller::mqtt_register_client)
                    .service(gateway_controller::mqtt_unregister_client)
                    .service(gateway_controller::mqtt_set_ssl_reconnect)
                    .service(gateway_controller::mqtt_set_client_reconnect)
                    // NTP Client Manager
                    .service(gateway_controller::ntp_available_clients)
                    .service(gateway_controller::ntp_send_requests)
                    .service(gateway_controller::ntp_register_client)
                    .service(gateway_controller::ntp_unregister_client)
                    .service(gateway_controller::ntp_set_client)
                    // Protocol Stack Manager
                    .service(gateway_controller::protocol_available_stacks)
                    .service(gateway_controller::protocol_available_types)
                    .service(gateway_controller::protocol_available_formats)
                    .service(gateway_controller::protocol_register_stack)
                    .service(gateway_controller::protocol_unregister_stack)
                    .service(gateway_controller::protocol_set_stack)
            )
                .service(
                    web::scope("/wx")
                        .service(wx_controller::wecallback)
                    // ).service(
                    //     web::scope("/ws")
                    //     .service(system_websocket::websocket_handler2)
                )
                .route("/captchaImage", web::get().to(img_controller::captcha))
                .route(
                    "/login",
                    web::post().to(sys_auth_controller::login),
                )
                .route(
                    "/getInfo",
                    web::get().to(sys_auth_controller::info),
                )
                .route(
                    "/getRouters",
                    web::get().to(sys_menu_controller::routers),
                )
                .route(
                    "/logout",
                    web::post().to(sys_auth_controller::logout),
                )
                .route(
                    "/ws/{user_id}",
                    web::get().to(system_websocket::websocket_handler),
                )
            )
    })
        .bind(&CONTEXT.config.server_url).unwrap()
        .run()
}
