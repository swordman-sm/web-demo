use chrono::Utc;
use serde::{Deserialize, Serialize};
use rbatis::CRUDTable;

// #[derive(CRUDTable, Serialize, Deserialize, Default, Clone, Debug)]
#[derive(CRUDTable, Serialize, Deserialize, Default, Clone, Debug)]
pub struct VideoReplies {
    ///编号
    pub id: Option<usize>,
    ///视频编号
    pub video_id: Option<usize>,
    ///评论编号
    pub reply_id: Option<usize>,
    ///用户编号
    pub user_id: Option<usize>,
    ///用户名称
    pub user_name: Option<String>,
    ///内容
    pub content: Option<String>,
    ///排序
    pub seq: Option<isize>,
    ///状态
    pub state: Option<u32>,
    ///创建时间
    pub created: u32,
}

impl VideoReplies {
    pub fn new() -> Self {
        let utc = Utc::now();
        let now = utc.naive_local().timestamp_subsec_nanos();
        return Self {
            id: None,
            video_id: None,
            reply_id: None,
            user_id: None,
            user_name: None,
            content: None,
            seq: None,
            state: None,
            created: now,
        };
    }
}