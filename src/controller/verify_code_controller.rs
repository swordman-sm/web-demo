use actix_web::{Responder, web};
use crate::model::dto::user_dto::VerifyCodeDTO;
use crate::service::VERIFY_CODE_SERVICE;
use crate::model::vo::resp_vo::RespVo;

//获取验证码
pub async fn verify_code(arg: web::Json<VerifyCodeDTO>) -> impl Responder {
    let vo = VERIFY_CODE_SERVICE.send_register_code(&arg.0).await;
    return RespVo::from_result(&vo).to_json_resp();
}