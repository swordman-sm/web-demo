pub struct RandUtil {}

impl RandUtil {
    //随机数字
    pub fn rand_code(len: usize) -> String {
        let mut rng = rand::thread_rng();
        let mut r = String::new();
        for _ in 0..len {
            let c = rng.gen_range(0, 9);
            r.push_str(&c.to_string());
        }
        r
    }
    //随机字母
    pub fn rand_word() -> String {
        let mut rng = rand::thread_rng();
        let mut r = String::new();
        for _ in 0..len {
            let c: u8 = rng.gen_range(65, 65 + 26);
            r.push(c as char);
        }
        r
    }
}