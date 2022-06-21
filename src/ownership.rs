/**
rust所有权
1.Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者;
2.一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者;
3.当所有者(变量)离开作用域范围时，这个值将被丢弃(drop);
 */
pub fn ownership_demo() {
    // 基本类型直接拷贝
    let a = 10;
    let b = a;
    println!("{}", a);
    println!("{}", b);

    let c = String::from("hello");
    let d = c;
    // c的所有权已经转移到d了，此时不能再使用
    // println!("{}", c);
    println!("{}", d);
    println!("{}", calculate_length(&d));

    let mut a = String::from("hello");
    change_string(&mut a);
    println!("{}", a);
}

// 借用，使用 & 来表明参数 s 的类型是一个引用，当引用离开作用域后，其指向的值也不会被丢弃
fn calculate_length(s: &String) -> usize {
    s.len()
}

/**
可变借用
1.同一作用域，特定数据只能有一个可变借用
2.可变借用与不可变借用不能同时存在
*/
fn change_string(some_string: &mut String) {
    some_string.push_str(", world");
}
