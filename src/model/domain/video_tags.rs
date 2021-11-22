use serde::{Deserialize, Serialize};
use rbatis::CRUDTable;
use validator::Validate;
use validator_derive::Validate;

#[derive(CRUDTable, Validate, Serialize, Deserialize, Default, Clone, Debug)]
pub struct VideoTags {
    ///编号
    pub id: Option<usize>,
    ///名称
    #[validate(required, length(min = 2, max = 20, message = "名称必须在2-20之间"))]
    pub name: Option<String>,
    ///备注
    #[validate(required, length(min = 0, max = 50, message = "备注长度必须在0-50之间"))]
    pub remark: Option<String>,
    ///排序
    #[validate(required, range(min = - 2147483648, max = 2147483647, message = "排序必须是有效的数字"))]
    pub seq: Option<isize>,
}

impl VideoTags {
    pub fn new() -> Self {
        return Self {
            id: None,
            name: None,
            remark: None,
            seq: None,
        };
    }
}