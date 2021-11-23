use std::collections::HashMap;
use serde_json::json;
use serde_json::value::Value;
use tera::Result;
use crate::model::domain::AdminRoles;
use std::sync::Mutex;
use rbatis::rbatis::Rbatis;
use crate::config::BOOT_CONFIG;


lazy_static! {
    pub static ref ADMIN_ROLES: Mutex<HashMap<usize, String>> = {
        let mut rb = Rbatis::new();
        rb.link(&format!("mysql://{}:{}@{}", &BOOT_CONFIG.mysql_user, &BOOT_CONFIG.mysql_password, mysql_url)).await.unwrap();
        let admin_roles:Vec<AdminRoles> = rb.fetch_list().await.unwrap();
        let mut roles = HashMap::new();
        for admin_role in admin_roles {
            roles.insert(admin_role.id.unwrap(), admin_role.name.unwrap());
        }
        Mutex::new(roles)
    };
}

/// 得到菜单名称
pub fn role_name<'r, 's>(val: &'r Value, _data: &'s HashMap<String, Value>) -> Result<Value> {
    if let Value::Number(n) = val {
        let id = n.as_u64().unwrap() as usize;
        if id == 0 {
            return Ok(json!(""));
        }
        let roles = ADMIN_ROLES.lock().unwrap();
        if let Some(v) = roles.get(&id) {
            return Ok(json!(v));
        }
    }
    Ok(json!("错误!!!"))
}