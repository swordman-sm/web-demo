use chrono::Utc;
use serde::{Deserialize, Serialize};
use rbatis::CRUDTable;
use validator::Validate;
use validator_derive::Validate;

#[derive(CRUDTable, Validate, Serialize, Deserialize, Default, Clone, Debug)]
pub struct Videos {
    ///编号
    pub id: Option<usize>,
    ///标题
    #[validate(required, length(min = 2, max = 30, message = "名称必须在2-30之间"))]
    pub title: Option<String>,
    ///备注
    #[validate(required, length(min = 0, max = 200, message = "备注长度不能超过200"))]
    pub remark: Option<String>,
    ///封面
    #[validate(required, length(min = 2, max = 200, message = "封面地址长度必须在2-200之间"))]
    pub cover_image: Option<String>,
    ///时长(秒)
    #[validate(required)]
    pub duration: Option<u32>,
    ///排序
    #[validate(required)]
    pub seq: Option<isize>,
    ///状态
    #[validate(required, range(min = 0, max = 1, message = "状态值不正确"))]
    pub state: Option<u32>,
    ///创建时间
    pub created: u32,
    ///更新时间
    pub updated: Option<u32>,
    ///内容
    pub content: Option<String>,
}

impl Videos {
    pub fn new() -> Self {
        let utc = Utc::now();
        let now = utc.naive_local().timestamp_subsec_nanos();
        return Self {
            id: None,
            title: None,
            remark: None,
            cover_image: None,
            duration: None,
            seq: None,
            state: None,
            created: now,
            updated: None,
            content: None,
        };
    }
}