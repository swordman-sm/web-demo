#[macro_use]
extern crate lazy_static;
extern crate rbatis;
extern crate tera;

pub mod config;
pub mod controller;
pub mod dao;
pub mod mid;
pub mod model;
pub mod service;
pub mod utils;
pub mod constant;
pub mod filters;

use crate::config::BOOT_CONFIG;
use crate::controller::{user_controller, verify_code_controller};
use actix_web::middleware;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use log4rs;
use tera::Tera;
// use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
// use rbatis::rbatis::Rbatis;
// use serde_json::json;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //初始化日志
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    //初始化rbatis
    let mysql_url = &BOOT_CONFIG.mysql_url.replace("mysql://", "");
    dao::RB.link(&format!("mysql://{}:{}@{}", &BOOT_CONFIG.mysql_user, &BOOT_CONFIG.mysql_password, mysql_url)).await.unwrap();
    //初始化路由，启动http服务
    HttpServer::new(|| {
        let auth = HttpAuthentication::bearer(mid::validator);

        //模板引擎
        let mut tera = match Tera::new("/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        tera.register_filter("state_name", filters::common_filter::state_name);
        // tera.register_filter("menu_name", filters::menus::menu_name);
        tera.register_filter("yes_no", filters::common_filter::yes_or_no);
        // tera.register_filter("admin_role", filters::admin_roles::role_name);
        // tera.register_filter("position_name", filters::ads::position_name);

        App::new()
            .wrap(middleware::Logger::default())
            .data(tera)
            .service(
                web::scope("/v1")
                    .wrap(auth)
                    .configure(controller::configure),
            )
            .route("/login", web::post().to(user_controller::user_login))
            .route("/register", web::post().to(user_controller::user_reg))
            .route(
                "/verify_code",
                web::post().to(verify_code_controller::verify_code),
            )
            .route("/", web::get().to(index))
            .route("/index", web::get().to(index))
    })
        .bind(format!("127.0.0.1:{}", &BOOT_CONFIG.server_port))?
        .run()
        .await
}
