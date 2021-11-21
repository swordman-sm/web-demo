use serde::{Deserialize, Serialize};
use rbatis::CRUDTable;
use validator::Validate;
use validator_derive::Validate;

#[derive(CRUDTable, Validate, Serialize, Deserialize, Default, Clone, Debug)]
pub struct UserLevels {
    ///编号
    pub id: Option<usize>,
    ///等级名称
    #[validate(required, length(min = 2, max = 20, message = "名称必须在2-20之间"))]
    pub name: Option<String>,
    ///备注
    pub remark: Option<String>,
    ///每天观看数
    pub watch_per_day: Option<usize>,
    ///最低积分
    pub score_min: Option<u32>,
    ///最高积分
    pub score_max: Option<u32>,
    ///排序
    pub seq: Option<isize>,
}

impl UserLevels {
    pub fn new() -> Self {
        return Self {
            id: None,
            name: None,
            remark: None,
            watch_per_day: None,
            score_min: None,
            score_max: None,
            seq: None,
        };
    }
}