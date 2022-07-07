// 闭包是一种匿名函数，它可以赋值给变量也可以作为参数传递给其它函数，不同于函数的是，它允许捕获调用者作用域中的值
pub fn closure_demo()
{
    let x = 1;
    let sum = |y| x + y;
    println!("a=>{:?}", sum(2));
    println!("a=>{:?}", sum(20));
}
