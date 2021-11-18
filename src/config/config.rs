use config::Config;
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BootConfig {
    // 服务对外地址
    pub server_endpoint: String,
    // 日志路径
    pub log_path: String,
    // redis地址
    pub redis_url: String,
    // mysql地址
    pub mysql_url: String,
    // jwt 密钥
    pub jwt_secret: String,
}

impl Default for BootConfig {
    fn default() -> Self {
        Self {
            server_endpoint: "127.0.0.1:8000".to_owned(),
            log_path: "config/log4rs.yaml".to_owned(),
            redis_url: "redis://127.0.0.1:6379".to_owned(),
            mysql_url: "mysql://root:root@localhost:3306/test".to_owned(),
            jwt_secret: "923123456".to_owned(),
        }
    }
}

impl BootConfig {
    pub fn init() -> Self {
        dotenv().ok();
        let mut c = Config::new();
        c.merge(config::Environment::default())
            .expect("加载配置失败!!!");
        ///尝试将配置对象解析成需要的类型
        c.try_into().expect("init env error")
    }
}