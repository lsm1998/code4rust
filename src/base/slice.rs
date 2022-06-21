pub fn slice_demo()
{
    let mut s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("slice: {:?} , {:?}", hello, world);
    println!("s: {:?}", s);
    let c = s.pop();
    match c {
        None => {}
        Some(c) => {
            println!("{}", c);
        }
    };
}
