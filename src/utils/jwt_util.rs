use std::time::{Duration, SystemTime, UNIX_EPOCH};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use jsonwebtoken::errors::ErrorKind;
use crate::constant::enums::{Error, Result};

pub struct JWTToken {
    id: String,
    username: String,
    //(audience)受众
    aud: String,
    exp: usize,
    //(Issued At)签发时间
    iat: usize,
    //(issuer)签发人
    iss: String,
    //(Not Before)生效时间
    nbf: usize,
    //(Subject)主题
    sub: String,
    //(JWT ID)编号
    jti: String,
}

impl JWTToken {
    pub fn new(username: &str) -> Self {
        let now = SystemTime::now();
        //30分钟过期
        let m30 = Duration::from_secs(30 * 60);
        let d_now = now.duration_since(UNIX_EPOCH).expect("获取系统时间失败");
        Self {
            id: String::from("abc"),
            username: String::from(username),
            aud: String::from("s: &str"), // (audience)：受众
            exp: (d_now + m30).as_secs() as usize,
            iat: d_now.as_secs() as usize,  // (Issued At)：签发时间
            iss: String::from("luokai"),     // (issuer)：签发人
            nbf: d_now.as_secs() as usize,  // (Not Before)：生效时间
            sub: String::from("luokai.ltd"), // (subject)：主题
            jti: String::from("ignore"),  // (JWT ID)：编号
        }
    }

    pub fn create_token(&self, secret: &str) -> Result<String> {
        return match encode(&Header::default(), self,
                            &EncodingKey::from_secret(secret.as_ref())) {
            Ok(t) => Ok(t),
            Err(_) => Err(Error::from("JWTToken encode fail!"))
        };
    }

    pub fn verify(secret: &str, token: &str) -> Result<JWTToken> {
        let mut validation = Validation::default();
        validation.validate_nbf = true;
        return match decode(&token, &DecodingKey::from_secret(secret.as_ref()),
                            &validation) {
            Ok(c) => Ok(c.claims),
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => Err(Error::from("InvalidToken")),
                ErrorKind::InvalidIssuer => return Err(Error::from("InvalidIssuer")), // Example on how to handle a specific error
                ErrorKind::ExpiredSignature => return Err(Error::from("ExpiredSignature")),
                _ => return Err(Error::from("InvalidToken other errors")),
            }
        };
    }
}