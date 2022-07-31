// 通过Cell 和 RefCell，可以做到拥有不可变引用的同时修改目标数据
use std::cell::Cell;

pub fn cell_demo() {
    let c = Cell::new("修改前");
    let one = c.get();
    c.set("修改后");
    let two = c.get();
    println!("{},{}", one, two);
}

