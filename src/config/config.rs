use config::Config;
use dotenv::dotenv;
use serde::Deserialize;

///服务启动配置
#[derive(Debug, Deserialize)]
pub struct BootConfig {
    ///对外服务地址
    pub server_port: String,
    ///日志路径
    pub log_path: String,
    ///redis地址
    pub redis_host: String,
    ///redis端口
    pub redis_port: i32,
    ///mysql地址
    pub mysql_url: String,
    ///mysql用户名
    pub mysql_user: String,
    ///mysql密码
    pub mysql_password: String,
    ///jwt 密钥
    pub jwt_secret: String,
}

///默认配置
impl Default for BootConfig {
    fn default() -> Self {
        Self {
            server_port: "8000".to_owned(),
            log_path: "config/log4rs.yaml".to_owned(),
            redis_host: "127.0.0.1".to_owned(),
            redis_port: 6379,
            mysql_url: "mysql://localhost:3306/test".to_owned(),
            mysql_user: "root".to_owned(),
            mysql_password: "root".to_owned(),
            jwt_secret: "923123456".to_owned(),
        }
    }
}

impl BootConfig {
    pub fn init() -> Self {
        dotenv().ok();
        //创建可变的Config对象
        let mut c = Config::new();
        c.merge(config::Environment::default())
            .expect("加载配置失败!!!");
        c.try_into().expect("init env error")
    }
}