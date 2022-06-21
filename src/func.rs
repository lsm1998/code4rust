pub fn func_demo() {
    println!("add() = {}", add(1, 2));

    // forever();

    // dead_end();
}

fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn dead_end() -> ! {
    panic!("你已经到了穷途末路，崩溃吧！");
}

fn forever() -> ! {
    loop {
        println!("hello");
    };
}
