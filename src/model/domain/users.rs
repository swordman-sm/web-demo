use chrono::Utc;
use serde::{Deserialize, Serialize};
use rbatis::CRUDTable;
// use validator::{ValidationError};
// use validator_derive::Validate;
// use crate::model::domain::RE_USERNAME;

// #[derive(CRUDTable, Validate, Serialize, Deserialize, Default, Clone, Debug)]
#[derive(CRUDTable, Serialize, Deserialize, Default, Clone, Debug)]
pub struct Users {
    ///编号
    pub id: Option<usize>,
    ///用户名称
    // #[validate(required, custom(function = "is_username", message = "必须是用户名称"))]
    pub name: Option<String>,
    ///登陆密码
    pub password: Option<String>,
    ///用户密钥
    pub secret: Option<String>,
    ///电子邮件
    pub mail: Option<String>,
    ///最后登录ip
    pub last_ip: Option<String>,
    ///状态, 是否可用, 0: 不可用, 1:可用
    pub state: Option<u32>,
    ///登陆次数
    pub login_count: Option<u32>,
    ///等级编号
    pub level_id: Option<u32>,
    ///积分
    pub score: Option<u32>,
    ///上次登录时间
    pub last_login: Option<u32>,
    ///创建时间
    pub created: u32,
    ///更新时间
    pub updated: Option<u32>,
    ///最后登录时间
    pub remark: Option<String>,
}

// /// 是否是用户名称, 6-20位, 英文开头, 数字、下划线、英文
// pub fn is_username(v: &str) -> Result<(), ValidationError> {
//     let count = v.chars().count();
//     if count >= 5 && count < 20 && RE_USERNAME.is_match(v) {
//         return Ok(());
//     }
//
//     return Err(ValidationError::new("invalid identity"));
// }

impl Users {
    pub fn new() -> Self {
        let utc = Utc::now();
        let now = utc.naive_local().timestamp_subsec_nanos();
        return Self {
            id: None,
            name: None,
            password: None,
            secret: None,
            mail: None,
            last_ip: None,
            state: None,
            login_count: None,
            level_id: None,
            score: None,
            last_login: None,
            created: now,
            updated: None,
            remark: None,
        };
    }
}