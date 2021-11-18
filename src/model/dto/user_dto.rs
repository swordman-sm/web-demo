use validator::Validate;
use validator_derive::Validate;

///登陆request数据对象
#[derive(Serialize, Deserialize, Clone, Debug, Validate)]
pub struct LoginReqDto {
    // 类似于@NotNull 等校验注解
    #[validate(required, length(min = 6, max = 20, message = "参数错误"))]
    pub username: Option<String>,
    #[validate(required, length(min = 8, max = 20, message = "参数错误"))]
    pub password: Option<String>,
    #[validate(required, length(min = 4, max = 4, message = "参数错误"))]
    pub verify_code: Option<String>,
}

///用户添加request数据对象
#[derive(Serialize, Deserialize, Clone, Debug, Validate)]
pub struct UserAddDto {
    #[validate(required, length(min = 6, max = 20, message = "参数错误"))]
    pub username: Option<String>,
    #[validate(required, length(min = 8, max = 20, message = "参数错误"))]
    pub password: Option<String>,
    #[validate(required, length(min = 4, max = 4, message = "参数错误"))]
    pub verify_code: Option<String>,
}

/// 用户分页
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserPageDTO {
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub username: Option<String>,
    pub name: Option<String>,
}

///验证码校验request数据对象
#[derive(Serialize, Deserialize, Clone, Debug, Validate)]
pub struct VerifyCodeDTO {
    #[validate(required)]
    pub vc_type: Option<VcType>,
    #[serde(default = "verify_code_default_len")]
    pub len: usize,
}

fn verify_code_default_len() -> usize { 4 }

///修改密码request数据对象
#[derive(Serialize, Deserialize, Clone, Debug, Validate)]
pub struct ChangePasswordDto {
    #[validate(required, length(min = 6, max = 20, message = "参数错误"))]
    pub username: Option<String>,
    #[validate(required, length(min = 8, max = 20, message = "参数错误"))]
    pub old_password: Option<String>,
    #[validate(required, length(min = 8, max = 20, message = "参数错误"))]
    pub new_password: Option<String>,
    #[validate(required, length(min = 4, max = 4, message = "参数错误"))]
    pub verify_code: Option<String>,
}

