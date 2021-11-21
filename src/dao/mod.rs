use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
use rbatis::rbatis::Rbatis;

// 示例-Rbatis示例初始化(必须)
lazy_static! {
    pub static ref RB: Rbatis = {
        let mut rb = Rbatis::new();
        let del = RbatisLogicDeletePlugin::new("del_state");
        rb.logic_plugin = Some(Box::new(del));
        rb
    };
}

#[tokio::main]
#[test]
async fn test_rbatis() {
    use crate::config::BOOT_CONFIG;
    use bson2::bson;
    use rbatis::executor::Executor;

    println!("{}", &BOOT_CONFIG.mysql_url);
    let mysql_url = &BOOT_CONFIG.mysql_url.replace("mysql://", "");
    RB.link(&format!("mysql://{}:{}@{}", &BOOT_CONFIG.mysql_user, &BOOT_CONFIG.mysql_password, mysql_url)).await.unwrap();
    let arg = vec![bson!(1)];
    println!("{}", arg[0]);
    // let w = RB.new_wrapper().eq("DEL_STATE", arg);
    let v: i64 = RB.fetch("SELECT count(1) FROM TM_USER where DEL_STATE = ?;", arg).await.unwrap();

    // let v: serde_json::Value = RB
    //     .("", "SELECT count(1) FROM TM_USER where DEL_STATE = ?;", arg)
    //     .await
    //     .unwrap();
    println!("{}", v.to_string());
}
