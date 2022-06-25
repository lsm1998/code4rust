use num::Num;

fn add<T: Num>(a: T, b: T) -> T {
    a + b
}

pub fn add_demo() {
    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));
    println!("add f64: {}", add(1.23, 1.23));
}
