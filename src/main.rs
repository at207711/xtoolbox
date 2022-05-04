mod fun;

use fun::get_guid;
use fun::md5;


fn main() {
    println!("{}",get_guid());
    println!("{}",md5("123456"));
    println!("hello,this test string...")
}