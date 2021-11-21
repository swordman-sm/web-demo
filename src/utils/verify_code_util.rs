use crate::constant::enums::Result;
use crate::constant::enums::VcType;
use crate::service::REDIS_SERVICE;
use crate::utils::rand_util::RandUtil;

/// 验证码
pub struct VerifyCode {}

impl VerifyCode {
    pub async fn gen_code_reg(key: &str) -> Result<String> {
        VerifyCode::gen_code(&VcType::Register(key.to_owned()), 4, 3 * 60).await
    }

    pub async fn has_code(id: &VcType) -> Result<bool> {
        let key = id.get();
        if let Some(_) = REDIS_SERVICE.get_string_value(&key).await? {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn gen_code(id: &VcType, len: usize, second: i32) -> Result<String> {
        let key = id.get();
        if let Some(c) = REDIS_SERVICE.get_string_value(&key).await? {
            return Ok(c);
        }
        let code = RandUtil::rand_code(len);
        // 保存验证码
        REDIS_SERVICE.add_string_value(&key, &code).await?;
        // 60 秒过期
        REDIS_SERVICE.expire(&key, second).await?;
        Ok(code.to_owned())
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

#[cfg(test)]
mod tests {
    use crate::constant::VcType;
    use crate::utils::verify_code_util::VerifyCode;

    #[tokio::main]
    #[test]
    async fn test_vc() {
        let code = VerifyCode::gen_code_reg("zhang").await.unwrap();
        println!("验证码:{}", code);
        assert_eq!(
            false,
            VerifyCode::verify(&VcType::Login(String::from("zhang")), &code).await
        );
        assert_eq!(
            true,
            VerifyCode::verify(&VcType::Register(String::from("zhang")), &code).await
        );
        assert_eq!(
            false,
            VerifyCode::verify(&VcType::Register(String::from("zhang")), &code).await
        );
    }
}
