use lazy_static::lazy_static;

mod user_service;
mod verify_code_service;
mod redis_service;

use redis_service::RedisService;
use verify_code_service::VerifyCodeService;
use user_service::UserService;
use crate::config::BOOT_CONFIG;


lazy_static! {
    pub static ref REDIS_SERVICE: RedisService = RedisService::new(&BOOT_CONFIG.redis_url);
    pub static ref VERIFY_CODE_SERVICE : VerifyCodeService = VerifyCodeService{};
    pub static ref USER_SERVICE:UserService = UserService{};
}