use std::collections::HashMap;
use serde_json::{json, Value};
use tera::Result;

/// 状态常量
pub const STATES: [&'static str; 2] = ["禁用", "正常"];

///状态名称
pub fn state_name<'r, 's>(val: &'r Value, _data: &'s HashMap<String, Value>) -> Result<Value> {
    if let Value::Number(v) = val {
        let n = v.as_u64().unwrap();
        if n != 0 && n != 1 {
            return Ok(json!("未知等级"));
        }
        return Ok(json!(STATES[n as usize]));
    }
    Ok(json!("错误!!!"))
}

///判断是否
pub fn yes_or_no<'r, 's>(val: &'r Value, _data: &'s HashMap<String, Value>) -> Result<Value> {
    if let Value::Number(v) = val {
        let n = v.as_u64().unwrap();
        if n == 1 {
            return Ok(json!("是"));
        }
        if n == 0 {
            return Ok(json!("否"));
        }
    }
    Ok(json!("错误!!!"))
}