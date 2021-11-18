use rbatis::crud::CRUD;
use rbatis::wrapper::Wrapper;
use validator::Validate;
use crate::model::dto::user_dto::LoginReqDto;
use crate::constant::enums::{Error, Result, VcType}
use crate::dao::RB;
use crate::utils::password_util::PasswordUtil;
use crate::utils::verify_code_util::VerifyCodeUtil;
use crate::model::domain::

pub struct UserService {}

impl UserService {
    pub async fn login(&self, arg: &LoginReqDto) -> Result<LoginReqDto> {
        arg.validate();
        //校验验证码
        if VerifyCodeUtil::verify(&VcType::SignIn(arg.username.as_ref().unwrap().clone()),
                                  arg.verify_code.as_ref().unwrap()).await {
            return Err(Error::from("验证码错误"));
        }
        let w = Wrapper::new(&RB.driver_type()?)
            .eq("username", &arg.username).check()?;

        let user: Option<TmUser> = RB.fetch_by_wrapper("", &w).await?;
        if user.is_none() {
            return Err(Error::from(format!("用户{}不存在!", arg.username.as_ref().unwrap())));
        }
        let user = user.unwrap();

        if !PasswordUtil::verify(user.) { }

        return Err();
    }
}