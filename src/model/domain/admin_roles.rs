use serde::{Deserialize, Serialize};
use rbatis::CRUDTable;
use validator::Validate;
use validator_derive::Validate;

//
#[derive(CRUDTable, Validate, Serialize, Deserialize, Clone, Debug)]
pub struct AdminRoles {
    ///编号
    pub id: Option<usize>,
    ///名称
    // #[validate(required, length(min = 2, max = 20, message = "分类名称必须在2-20之间"))]
    pub name: Option<String>,
    ///备注
    // #[validate(required, length(min = 0, max = 50, message = "备注长度必须在0-50之间"))]
    pub remark: Option<String>,
    ///菜单编号
    pub menu_ids: Option<String>,
    ///排序
    #[validate(required, regex(path = "\\d+", message = "排序必须是有效的数字"))]
    pub seq: Option<isize>,
}

impl AdminRoles {
    pub fn new() -> Self {
        return Self {
            id: None,
            name: None,
            remark: None,
            menu_ids: None,
            seq: None,
        };
    }
}