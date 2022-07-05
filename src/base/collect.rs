use std::collections::HashMap;

pub fn collect_demo() {
    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("第三个元素是 {}", third);

    // get内部处理了数组越界
    match v.get(2) {
        Some(third) => println!("第三个元素是 {}", third),
        None => println!("去你的第三个元素，根本没有！"),
    }

    // 如下编译报错，first是不可变借用，而push是可变借用
    // 尽管操作元素不同，但是如果引发扩容，则导致first指向无效内存
    // let first = &v[0];
    // v.push(6);

    // 遍历vec
    for i in &v {
        println!("{}", i);
    }
    for i in &mut v {
        *i += 10
    }

    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert(String::from("语文"), 90);
    map.insert(String::from("数学"), 80);
    map.insert(String::from("英语"), 70);
    for k in &map {
        println!("{}:{}", k.0, k.1);
    }
    map.remove(&*String::from("英语"));

    let a: i32 = 10;
    let b: u16 = 100;
    if a < b as i32 {
        println!("Ten is less than one hundred.");
    }
}
