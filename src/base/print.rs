pub fn print_demo() {
    // 二进制 => 0b11011!
    println!("{:#b}!", 27);
    // 八进制 => 0o33!
    println!("{:#o}!", 27);
    // 十进制 => 27!
    println!("{}!", 27);
    // 小写十六进制 => 0x1b!
    println!("{:#x}!", 27);
    // 大写十六进制 => 0x1B!
    println!("{:#X}!", 27);
    // 不带前缀的十六进制 => 1b!
    println!("{:x}!", 27);
    // 使用0填充二进制，宽度为10 => 0b00011011!
    println!("{:#010b}!", 27);
    let v = vec![1, 2, 3];
    println!("{:p}", v.as_ptr()); // => 0x600002324050
    let value = get_value();
    println!("{value}");
    // 保留小数点后两位 => 3.14
    println!("{:.2}", std::f32::consts::PI);
    // 通过参数来设定精度 => 3.1416，相当于{:.4}
    println!("{:.1$}", std::f32::consts::PI, 4);
}

fn get_value() -> String {
    String::from("the value")
}
