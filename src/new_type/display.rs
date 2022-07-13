use std::fmt;

// 为 Vec 实现 Display 特征，
// 但是，Vec 类型定义在标准库中
// 使用 newtype 解决

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn vec_display() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
