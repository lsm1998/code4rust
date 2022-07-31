// 为什么需要RC和ARC？
// Rust 所有权机制要求一个值只能有一个所有者
// RC和ARC通过引用计数的方式，允许一个数据资源在同一时刻拥有多个所有者，前者单线程，后者线程安全

use std::rc::Rc;

pub fn rc_demo() {
    // 把a的所有权交给两个Rc
    let a = String::from("hello, world");
    let b = Rc::new(a);
    let c = Rc::clone(&b);
    // strong_count 返回2
    println!("{:?}",Rc::strong_count(&c));
}
