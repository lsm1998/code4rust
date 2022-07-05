pub fn convert_demo() {
    // as 转换
    let a: i8 = 10;
    let b: i32 = 100;
    if (a as i32) < b {
        println!();
    }
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    // 将p1内存地址转换为一个整数
    let first_address = p1 as usize;
    let second_address = first_address + 4;
    let p2 = second_address as *mut i32; // 访问该地址指向的下一个整数p2
    unsafe {
        *p2 += 1;
    }

    // TryInto 转换
    let a: i16 = 1500;
    let b: u8 = match a.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };
    println!("{}", b);
}
