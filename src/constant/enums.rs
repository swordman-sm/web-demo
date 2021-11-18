use redis::RedisError;
use validator::ValidationErrors;

/// 验证码类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CacheKey {
    // Token
    Token(String),
    // JWT
    JwtSecret(String),
    // 验证码
    VerifyCode(String),
}

impl CacheKey {
    pub fn get_key(&self) -> String {
        match self {
            CacheKey::Token(s) => format!("token:{}", s),
            CacheKey::JwtSecret(s) => format!("jwt:{}", s),
            CacheKey::VerifyCode(s) => format!("vc:{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VcType {
    ///注册
    REG(String),
    SignIn(String),
    ChangePassword(String),
}

impl VcType {
    pub fn get(&self) -> CacheKey {
        CacheKey::VerifyCode(match self {
            Self::REG(id) => format!("reg:{}", id),
            Self::SignIn(id) => format!("sign_in:{}", id),
            Self::ChangePassword(id) => format!("cpwd:{}", id)
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Error {
    E(String),
    JsonError(String),
    RedisError(String),
    RbatisError(String),
    ValidationError(String),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::E(msg) => msg.clone(),
            Error::JsonError(msg) => msg.clone(),
            Error::RedisError(msg) => msg.clone(),
            Error::RbatisError(msg) => msg.clone(),
            Error::ValidationError(msg) => msg.clone()
        }
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error::E(msg.to_owned())
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Error::E(msg)
    }
}

impl From<RedisError> for Error {
    fn from(e: RedisError) -> Self {
        Error::RedisError(e.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::JsonError(e.to_string())
    }
}

impl From<rbatis_core::Error> for Error {
    fn from(e: rbatis_core::Error) -> Self {
        Error::RbatisError(e.to_string())
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(e: ValidationErrors) -> Self {
        let emap = e.errors();
        let msg = emap.iter().map(|(k, v)| match v {
            validator::ValidationErrorsKind::Field(f) => {
                return format!("{}:{}", k, f.iter()
                    .filter(|e| e.message.is_some())
                    .map(|e| e.message.as_ref().unwrap().to_string())
                    .join(",")
                );
            }
            _ => "参数错误".to_owned(),
        }).collect::<Vec<String>>().join(", ");
        Error::ValidationError(msg)
    }
}

pub type Result<T> = std::result::Result<T,Error>;