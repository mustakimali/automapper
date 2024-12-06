use std::time::SystemTime;

pub fn random_string() -> String {
    format!("{:?}", SystemTime::now())
}
