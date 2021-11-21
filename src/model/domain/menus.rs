use serde::{Deserialize, Serialize};
use rbatis::CRUDTable;
use validator_derive::Validate;

#[derive(CRUDTable, Validate, Serialize, Deserialize, Default, Clone, Debug)]
pub struct Menus {
    ///编号
    pub id: Option<usize>,
    ///上级编号
    #[validate(required, regex(path = "\\d+", message = "上级菜单编号必须是有效的数字"))]
    pub parent_id: Option<usize>,
    ///菜单名称
    #[validate(required, length(min = 2, max = 20, message = "分类名称必须在2-20之间"))]
    pub name: Option<String>,
    ///菜单级别 级别ID,1:主菜单;2:子菜单
    pub level_id: Option<usize>,
    ///状态,0:隐藏;1:显示
    #[validate(required, regex(path = "0|1", message = "状态值不正确"))]
    pub state: Option<u32>,
    ///链接地址
    #[validate(required, length(min = 0, max = 200, message = "链接地址长度不能超过200"))]
    pub url: Option<String>,
    ///是否新窗口 是否外链,0:否,1:是
    #[validate(required, regex(path = "0|1", message = "是否外链值不正确"))]
    pub is_blank: Option<u32>,
    ///是否显式,0:否,1:是
    pub is_show: Option<u32>,
    ///排序
    #[validate(required, regex(path = "\\d+", message = "排序必须是有效的数字"))]
    pub seq: Option<isize>,
}

impl Menus {
    pub fn new() -> Self {
        return Self {
            id: None,
            parent_id: None,
            name: None,
            level_id: None,
            state: None,
            url: None,
            is_blank: None,
            is_show: None,
            seq: None,
        };
    }
}