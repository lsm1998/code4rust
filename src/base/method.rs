pub fn method_demo() {
    let r = Rectangle { width: 1, height: 1 };
    println!("area={}",r.area());

    let r = Rectangle::new(5,5);
    println!("area={}",r.area());
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 关联函数，类似于构造函数
    fn new(w: u32, h: u32) -> Rectangle {
        Rectangle { width: w, height: h }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}
