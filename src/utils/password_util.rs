pub struct PasswordUtil {}

impl PasswordUtil {
    pub fn encode(raw_password: &str, salt: &str) -> String {
        let digest = md5::compute(format!("{}-{}", raw_password, salt));
        format!("{:x}", digest)
    }
    pub fn verify(password: &str, raw_password: &str, salt: &str) -> bool {
        let hashed = Self::encode(raw_password, salt);
        password.eq(&hashed)
    }
}

#[test]
fn test_encode() {
    let s = PasswordUtil::encode("123456", "123");
    println!("{}", s);
    assert_eq!(
        PasswordUtil::encode("123456", "123"),
        PasswordUtil::encode("123456", "123")
    )
}
