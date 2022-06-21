use num::complex::Complex;

pub fn variables_demo() {
    // 变量绑定, a变量拥有"hello world"这块内存的所有权
    let a = "hello world";
    println!("{:?}", a);

    // 变量解构，类似于js
    let (b, c): (&str, bool) = ("hello", false);
    println!("{:?} , {:?}", b, c);

    // 使用元组完成变量解构
    let (d, e, f);
    (e, f, d) = ("lsm", 25, "man");
    println!("{:?} , {:?} , {:?}", d, e, f);

    // 类型转换
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{:?}", guess);

    // 复数
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);

    // NaN
    let x = (-42.0_f32).sqrt();
    println!("x = {}, x.is_nan() = {}", x, x.is_nan());

    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
    println!("{} {} {} {}", c, z, g, heart_eyed_cat);
    // rust 字符都是4字节，代表 Unicode标量值
    println!("字符'z'占用了{}字节的内存大小", std::mem::size_of_val(&c));
    println!("字符'国'占用了{}字节的内存大小", std::mem::size_of_val(&g));

    // 集合
    let mut b: Vec<f64> = Vec::new();
    b.insert(0, 1_f64);
    println!("{:?}", b);
}
