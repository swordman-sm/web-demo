use chrono::Utc;
use serde::{Deserialize, Serialize};
use rbatis::CRUDTable;

#[derive(CRUDTable, Serialize, Deserialize, Default, Clone, Debug)]
pub struct WatchRecords {
    ///编号
    pub id: Option<usize>,
    ///用户编号
    pub user_id: Option<usize>,
    ///用户名称
    pub user_name: Option<String>,
    ///用户编号
    pub video_id: Option<usize>,
    ///创建时间
    pub created: u32,
}

impl WatchRecords {
    pub fn new() -> Self {
        let utc = Utc::now();
        let now = utc.naive_local().timestamp_subsec_nanos();
        return Self {
            id: None,
            user_id: None,
            user_name: None,
            video_id: None,
            created: now,
        };
    }
}