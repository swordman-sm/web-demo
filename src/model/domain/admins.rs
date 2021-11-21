use chrono::Utc;
use serde::{Deserialize, Serialize};
use rbatis::CRUDTable;
use validator_derive::Validate;

#[derive(CRUDTable, Validate, Serialize, Deserialize, Default, Clone, Debug)]
pub struct Admins {
    ///编号
    pub id: Option<usize>,
    ///用户名称
    pub name: Option<String>,
    ///用户密码
    #[validate(required, length(min = 6, max = 16, message = "密码必须在6-16之间"))]
    pub password: Option<String>,
    ///最后登录ip
    pub last_ip: Option<String>,
    ///状态, 是否可用, 0: 不可用, 1:可用
    pub state: Option<u32>,
    ///登录次数
    pub login_count: Option<u32>,
    ///最后登录时间
    pub last_login: Option<u32>,
    ///角色编号
    pub role_id: Option<usize>,
    ///添加时间
    pub created: u32,
    ///更新时间
    pub updated: Option<u32>,
    ///排序
    #[validate(required, regex(path = "\\d+", message = "排序必须是有效的数字"))]
    pub seq: Option<isize>,
}

impl Admins {
    pub fn new() -> Self {
        let utc = Utc::now();
        let now = utc.naive_local().timestamp_subsec_nanos();

        return Self {
            id: None,
            name: None,
            password: None,
            last_ip: None,
            state: None,
            login_count: None,
            last_login: None,
            role_id: None,
            created: now,
            updated: None,
            seq: None,
        };
    }
}