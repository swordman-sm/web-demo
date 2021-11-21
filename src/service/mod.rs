mod redis_service;
mod user_service;
mod verify_code_service;

use crate::constant::enums::CacheKey;
use redis_service::RedisService;
use user_service::UserService;
use verify_code_service::VerifyCodeService;

use crate::config::BOOT_CONFIG;

lazy_static! {
   ///chache
   pub static ref REDIS_SERVICE: RedisService = RedisService::new(format!("redis://{}:{}",&BOOT_CONFIG.redis_host,&BOOT_CONFIG.redis_port).as_str());

   pub static ref USER_SERVICE: UserService = UserService{};
   pub static ref VERIFY_CODE_SERVICE: VerifyCodeService = VerifyCodeService{};
}
