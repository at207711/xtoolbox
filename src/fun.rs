use crypto::digest::Digest;
use crypto::md5::Md5;
use uuid::Uuid;

pub fn md5<S: Into<String>>(input: S) -> String {
    let mut md5 = Md5::new();
    md5.input_str(&input.into());
    md5.result_str()
}

pub fn get_guid() -> String {
    Uuid::new_v4().to_string()
}
