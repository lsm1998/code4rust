mod map;
mod file;
mod variables;
mod func;
mod base;
mod ownership;
mod r#macro;
mod types;
mod generics;
mod error;
mod lifecycle;

use std::mem;

fn main() {
    variables::variables_demo();

    func::func_demo();

    ownership::ownership_demo();

    types::file::types_file_demo();

    base::base_demo();

    generics::generics_demo();

    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    let _ss: [&str; 1] = ["hello"];

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    println!("{:?}", _ss);

    map::map_demo();

    file::file_demo();

    error::error_demo();

    lifecycle::lifecycle_demo();
}
