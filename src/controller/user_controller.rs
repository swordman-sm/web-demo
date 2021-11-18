use actix_web::{Responder, web};
use crate::model::dto::req_page_dto::ReqPageDTO;
use crate::model::dto::user_dto::{ChangePasswordDto, LoginReqDto, UserAddDto, UserPageDTO};

/// 用户登陆 request mapping(返回对象为实现Responder标准的对象)
pub async fn user_login(arg: web::Json<LoginReqDto>) -> impl Responder {}

/// 用户新增 request mapping(返回对象为实现Responder标准的对象)
pub async fn user_register(arg: web::Json<UserPageDTO>) -> impl Responder {}

/// 修改密码 request mapping(返回对象为实现Responder标准的对象)
pub async fn change_password(arg: web::Json<ChangePasswordDto>) -> impl Responder {}

/// 用户分页 request mapping(返回对象为实现Responder标准的对象)
pub async fn user_page(arg: web::Json<ReqPageDTO>) -> impl Responder {}

