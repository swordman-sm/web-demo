use actix_web::{Responder, web};
use crate::model::dto::user_dto::{ChangePasswordDto, LoginReqDto, UserAddDto, UserPageDTO};
use crate::model::vo::resp_vo::RespVo;
use crate::service::USER_SERVICE;

/// 用户登陆 request mapping(返回对象为实现Responder标准的对象)
pub async fn user_login(arg: web::Json<LoginReqDto>) -> impl Responder {
    let vo = USER_SERVICE.login(&arg.0).await;
    return RespVo::from_result(&vo).to_json_resp();
}

/// 用户新增 request mapping(返回对象为实现Responder标准的对象)
pub async fn user_register(arg: web::Json<UserPageDTO>) -> impl Responder {
    let vo = USER_SERVICE.add(&arg.0).await;
    return RespVo::from_result(&vo).to_json_resp();
}

/// 修改密码 request mapping(返回对象为实现Responder标准的对象)
pub async fn change_password(arg: web::Json<ChangePasswordDto>) -> impl Responder {
    let vo = USER_SERVICE.change_password(&arg.0).await;
    return RespVo::from_result(&vo).to_json_resp();
}

/// 用户分页 request mapping(返回对象为实现Responder标准的对象)
pub async fn user_page(arg: web::Json<UserPageDTO>) -> impl Responder {
    let vo = USER_SERVICE.page(&arg.0).await;
    return RespVo::from_result(&vo).to_json_resp();
}

