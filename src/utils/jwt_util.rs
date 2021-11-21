use crate::constant::enums::Error;
use crate::constant::enums::Result;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTToken {
    id: String,
    username: String,
    aud: String,
    // (audience)：受众
    exp: usize,
    iat: usize,
    // (Issued At)：签发时间
    iss: String,
    // (issuer)：签发人
    nbf: usize,
    // (Not Before)：生效时间
    sub: String,
    // (subject)：主题
    jti: String, // (JWT ID)：编号
}

impl JWTToken {
    pub fn new(username: &str) -> JWTToken {
        let now = SystemTime::now();
        // 30 分钟过期时间
        let m30 = Duration::from_secs(30 * 60);
        let now = now.duration_since(UNIX_EPOCH).expect("获取系统时间失败");
        JWTToken {
            id: String::from("abc"),
            username: String::from(username),
            aud: String::from("s: &str"), // (audience)：受众
            exp: (now + m30).as_secs() as usize,
            iat: now.as_secs() as usize,  // (Issued At)：签发时间
            iss: String::from("luokai"),     // (issuer)：签发人
            nbf: now.as_secs() as usize,  // (Not Before)：生效时间
            sub: String::from("luokai.ltd"), // (subject)：主题
            jti: String::from("ignore"),  // (JWT ID)：编号
        }
    }
    ///创建token
    pub fn create_token(&self, secret: &str) -> Result<String> {
        return match encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        ) {
            Ok(t) => Ok(t),
            Err(_) => Err(Error::from("JWTToken encode fail!")),
        };
    }
    ///校验token
    pub fn verify(secret: &str, token: &str) -> Result<JWTToken> {
        let mut validation = Validation::default();
        validation.validate_nbf = true;
        //使用secret解码token
        return match decode::<JWTToken>(&token, &DecodingKey::from_secret(secret.as_ref()), &validation) {
            Ok(c) => Ok(c.claims),
            Err(err) => return match *err.kind() {
                ErrorKind::InvalidToken => Err(Error::from("InvalidToken")),
                ErrorKind::InvalidIssuer => Err(Error::from("InvalidIssuer")),
                ErrorKind::ExpiredSignature => Err(Error::from("ExpiredSignature")),
                _ => Err(Error::from("InvalidToken other errors")),
            },
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::jwt_util::*;

    #[test]
    fn test_jwt() {
        let jwt = JWTToken::new("username: &str");
        let res = jwt.create_token("test");
        let res = res.unwrap();
        let token = JWTToken::verify("test", &res);
        assert!(token.is_ok());
    }
}
