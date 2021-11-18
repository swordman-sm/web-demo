use crate::constant::enums::VcType;
use crate::service::REDIS_SERVICE;
use crate::utils::rand_util::RandUtil;
use crate::constant::enums::Result;


pub struct VerifyCodeUtil {}

impl VerifyCodeUtil {
    pub async fn gen_code(id: &VcType, len: usize, lifetime: i32) -> Result<String> {
        //获取一个key
        let key = id.get();
        if let Some(r) = REDIS_SERVICE.get_string_value(&key).await? {
            return Ok(r);
        }
        let code = RandUtil::rand_code(len);
        REDIS_SERVICE.set_string_value(&key, &code).await?;
        //设置过期时间
        REDIS_SERVICE.expire(&key, lifetime).await?;
        Ok(code.to_owned())
    }

    pub async fn gen_code_reg(key: &str) -> Result<String> {
        VerifyCodeUtil::gen_code(&VcType::REG(key.to_owned()), 4, 3 * 60).awit
    }

    pub async fn has_code(id: &VcType) -> Result<bool> {
        let key = id.get();
        if let Some(_) = REDIS_SERVICE.get_string_value(&key).await? {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn verify(id: &VcType, code: &str) -> bool {
        let key = id.get();
        if let Ok(c) = REDIS_SERVICE.get_string_value(&key).await {
            if let Some(ref rc) = c {
                let _ = REDIS_SERVICE.del(&key).await;
                return code == rc;
            }
        }
        return false;
    }
}