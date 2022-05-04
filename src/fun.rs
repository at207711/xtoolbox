use uuid::Uuid;
use crypto::digest::Digest;
use crypto::md5::Md5;

pub fn md5<S: Into<String>>(input: S) -> String {
    let mut md5 = Md5::new();
    md5.input_str(&input.into());
    md5.result_str()
}

pub fn get_guid() -> String {
    let guid = Uuid::new_v4();
    guid.to_string()
}



