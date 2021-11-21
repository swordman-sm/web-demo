use serde::{Serialize, Deserialize};

/// Redis Key
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CacheKey {
    /// Token
    Token(String),
    /// JWT
    JwtSecret(String),
    /// 验证码
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

/// 验证码类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VcType {
    /// 注册
    Register(String),
    /// 登陆
    Login(String),
    /// 修改密码
    ChangePassword(String),
}

impl VcType {
    pub fn get(&self) -> CacheKey {
        CacheKey::VerifyCode(match self {
            Self::Register(id) => format!("reg:{}", id),
            Self::Login(id) => format!("sign_in:{}", id),
            Self::ChangePassword(id) => format!("cpwd:{}", id),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum Error {
    ///通用错误
    CommonError(String),
    ///Json相关错误
    JsonError(String),
    ///Redis相关错误
    RedisError(String),
    ///Rbatis相关错误
    RbatisError(String),
    ///校验匹配等相关错误
    ValidationError(String),
}

impl ToString for Error {
    fn to_string(&self) -> std::string::String {
        match self {
            Error::CommonError(msg) => msg.clone(),
            Error::JsonError(msg) => msg.clone(),
            Error::RedisError(msg) => msg.clone(),
            Error::RbatisError(msg) => msg.clone(),
            Error::ValidationError(msg) => msg.clone(),
        }
        // serde_json::to_string(self).unwrap()
    }
}

///字面量类型错误
impl From<&str> for Error {
    fn from(msg: &str) -> Error {
        Error::CommonError(msg.to_owned())
    }
}

///String类型错误
impl From<String> for Error {
    fn from(msg: String) -> Error {
        Error::CommonError(msg)
    }
}

///Redis相关错误
impl From<redis::RedisError> for Error {
    fn from(e: redis::RedisError) -> Error {
        Error::RedisError(e.to_string())
    }
}

///Json相关错误
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::JsonError(e.to_string())
    }
}

///Rbatis相关错误
impl From<rbatis::Error> for Error {
    fn from(e: rbatis::Error) -> Error {
        Error::RbatisError(e.to_string())
    }
}

///校验类型相关错误
impl From<validator::ValidationErrors> for Error {
    fn from(e: validator::ValidationErrors) -> Error {
        let emap = e.errors();
        let msg = emap.iter().map(|(k, v)| match v {
            validator::ValidationErrorsKind::Field(f) => {
                return format!("{}:{}", k, f.iter()
                    .filter(|e| e.message.is_some())
                    .map(|e| e.message.as_ref().unwrap().to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
                );
            }
            _ => return "参数错误".to_owned(),
        }).collect::<Vec<String>>().join(", ");
        Error::ValidationError(msg)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use crate::constant::enums::Error;

    #[test]
    fn test_error() {
        println!("{:?}", Error::from("错误的消息"));
    }
}