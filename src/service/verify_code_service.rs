use log::debug;
use validator::Validate;
use crate::model::dto::user_dto::VerifyCodeDTO;
use crate::constant::enums::{Error, Result};
use crate::constant::enums::CacheKey::VerifyCode;
use crate::utils::verify_code_util::VerifyCodeUtil;


pub struct VerifyCodeService {}

impl VerifyCodeService {
    pub async fn send_register_code(&self, vc: &VerifyCodeDTO) -> Result<bool> {
        //参数验证
        vc.validate()?;
        if VerifyCodeUtil::has_code(vc.vc_type.as_ref().unwrap()).await? {
            return Err(Error::from("重复发送验证码!"));
        }
        match VerifyCodeUtil::gen_code(vc.vc_type.as_ref().unwrap(), vc.len, 3 * 60).await? {
            Ok(code) => {
                debug!("发送验证码, {:?}, code: {}",&vc.vc_type,code);
                Ok(true)
            }
            Err(e) => Err(e)
        }
    }
}