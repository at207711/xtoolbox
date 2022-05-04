pub mod fun;
pub use fun::get_guid;
pub use fun::md5;


#[cfg(test)]
mod tests {
    use fun::get_guid;
    use fun::md5;
    use crate::fun;

    #[test]
    fn test_md5() {
        println!("{}", md5("123456"));
    }

    #[test]
    fn test_guid() {
        println!("{}", get_guid());
    }
}
