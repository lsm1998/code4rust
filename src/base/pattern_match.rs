#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}

enum IpAddr {
    Ipv4,
    Ipv6,
}

pub fn pattern_match_demo() {
    let dire = Direction::South;
    // match 的匹配必须要穷举出所有可能，因此这里用 _ 来代表未列出的所有可能性
    // match 跟其他语言中的 switch 非常像，_ 类似于 switch 中的 default
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        }
        _ => println!("West"),
    };

    let ip1 = IpAddr::Ipv6;
    // match 本身也是一个表达式，因此可以用它来赋
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };
    println!("{}", ip_str);

    show_three(Option::Some(3u8));

    let v = vec![Direction::East, Direction::West, Direction::North];
    // matches! 宏封装了match，可以用于枚举值的比较
    let v = matches!(v[0], Direction::East);
    println!("{:?}", v);

    let mut stack = vec![1,2,3];
    // 只要模式匹配就一直进行 while 循环
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn show_three(s: Option<u8>) {
    // 使用if let处理模式的某一个值
    if let Some(3) = s {
        println!("three");
    } else {
        println!("not three")
    }
}
