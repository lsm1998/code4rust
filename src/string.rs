pub fn string_demo()
{
    let mut my_str = "hello";
    let my_string = String::from(my_str);
    println!("{:?}", my_string);
    my_str = "world";
    println!("{:?}", my_str);
}
