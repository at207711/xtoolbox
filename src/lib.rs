mod fun;


#[cfg(test)]
mod tests {
    use fun::get_guid;
    use fun::md5;

    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use crate::fun;

    #[test]
    fn test_md5() {
        println!("{}", md5("123456"));
    }

    #[test]
    fn test_guid() {
        // 这个断言会导致测试失败。注意私有的函数也可以被测试！
        println!("{}", get_guid());
    }
}
