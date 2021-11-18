pub mod config;

use lazy_static::lazy_static;
use config::BootConfig;

//当前服务配置
lazy_static! {
    pub static ref BOOT_CONFIG: BootConfig = BootConfig::init();
}