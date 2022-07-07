pub fn func_demo()
{
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// 编译器无法得知函数的返回值到底引用 x 还是 y，无法自动推导生命周期
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// 使用生命周期标注语法，禁止编译器提示错误
// 表示这两个参数 x 和 y 至少活得和'a 一样久
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

