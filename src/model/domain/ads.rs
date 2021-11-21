use serde::{Deserialize, Serialize};
use rbatis::CRUDTable;
// use validator_derive::Validate;

// #[derive(CRUDTable, Validate, Serialize, Deserialize, Clone, Debug)]
#[derive(CRUDTable, Serialize, Deserialize, Default, Clone, Debug)]
pub struct Ads {
    ///编号
    pub id: Option<usize>,
    ///名称
    // #[validate(required, length(min = 2, max = 20, message = "名称必须在2-20之间"))]
    pub name: Option<String>,
    ///备注
    // #[validate(required, length(min = 0, max = 200, message = "备注长度不能超过200字符"))]
    pub remark: Option<String>,
    ///图片地址
    // #[validate(required, length(min = 0, max = 200, message = "图片地址不能超过200字符"))]
    pub image: Option<String>,
    ///页面,0:首页;1:详情页
    // #[validate(required, range(min = 0, max = 1, message = "必须在可选范围之内"))]
    pub page_id: Option<u32>,
    ///位置,0:顶部;1:中左;2:中右;3:底部
    // #[validate(required, range(min = 0, max = 3, message = "位置必须在范围之内"))]
    pub position_id: Option<u32>,
    ///链接地址
    // #[validate(required, length(min = 0, max = 200, message = "链接地址长度不能超过200"))]
    pub url: Option<String>,
    ///是否外链,0:否,1:是
    // #[validate(required, regex(path = "0|1", message = "必须输入选项是否外链"))]
    pub is_blank: Option<u32>,
    ///排序
    // #[validate(required, regex(path = "\\d+", message = "排序必须是有效的数字"))]
    pub seq: Option<isize>,
}

impl Ads {
    pub fn new() -> Self {
        return Self {
            id: None,
            name: None,
            remark: None,
            image: None,
            page_id: None,
            position_id: None,
            url: None,
            is_blank: None,
            seq: None,
        };
    }
}