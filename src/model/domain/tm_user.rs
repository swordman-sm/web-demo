use chrono::{NaiveDateTime, Utc};
use rbatis::crud::CRUDEnable;
use serde::de::DeserializeOwned;
use serde::Serialize;

///用户对象结构体
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TmUser {
    pub user_no: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub salt: Option<String>,
    pub head_icon: Option<String>,
    pub del_state: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub last_modifyed_time: Option<NaiveDateTime>,
}

///给用户增加方法
impl TmUser {
    pub fn new() -> Self {
        let utc = Utc::now();
        TmUser {
            user_no: None,
            username: None,
            password: None,
            salt: None,
            head_icon: None,
            del_state: Some(1),
            create_time: Some(utc.naive_local()),
            last_modifyed_time: Some(utc.naive_local()),
        }
    }
}

impl CRUDEnable for TmUser {
    type IdType = String;
}