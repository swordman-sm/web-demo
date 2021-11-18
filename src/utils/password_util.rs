pub struct PasswordUtil {}

impl PasswordUtil {
    pub fn encode(raw_password: &str, salt: &str) -> String {
        let digest = md5::compute(format!("{}-{}", raw_password, salt));
        format!("{:x}", digest)
    }

    pub fn verify(password: &str, raw_password: &str, salt: &str) -> bool {
        let hashed = PasswordUtil::encode(raw_password, salt);
        password.eq(&hashed)
    }
}