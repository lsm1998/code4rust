pub fn string_demo()
{
    let mut my_str = "hello";
    let my_string = String::from(my_str);
    println!("{:?}", my_string);
    my_str = "world";
    println!("{:?}", my_str);

    let s1 = String::from("hello,");
    let s2 = String::from("world!");

    // 使用 + 或者 += 连接字符串，要求右边的参数必须为字符串的切片引用（Slice)类型
    // 等价于使用 std::string 标准库中的 add() 方法
    let s3 = s1 + &*s2;
    // s1的所有权被转移走了，因此不能再使用s1
    // println!("{}",s1);
    println!("{},{}", s2, s3);
}

// String 与 &str 的转换

// &str -> String
// String::from("hello,world")
// "hello,world".to_string()

// String -> &str
// (&s);
// (&s[..]);
// (s.as_str());
