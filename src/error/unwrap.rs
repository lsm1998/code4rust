use std::fs::File;
use std::net::IpAddr::{V4};
use std::net::IpAddr;

pub fn unwrap_demo() {
    // unwrap 直接返回值，失败则 panic
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    match home {
        V4(v4) => println!("v4 {}", v4),
        _ => {}
    }

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    };

    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    let f = File::open("hello.txt").unwrap();
    let f = open_file();
    println!("{:?}", f);
}


fn open_file() -> Result<File, Box<dyn std::error::Error>> {
    let f = File::open("hello.txt")?;
    Ok(f)
}
