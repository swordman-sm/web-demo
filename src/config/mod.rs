pub mod config;

use self::config::BootConfig;

//当前服务配置
//给静态变量延迟赋值的宏
lazy_static! {
    pub static ref BOOT_CONFIG: BootConfig = BootConfig::init();
}