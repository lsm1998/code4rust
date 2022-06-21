mod struct_;
mod map;
mod file;
mod variables;
mod func;
mod base;
mod ownership;
mod r#macro;
mod types;

use std::mem;

fn main() {
    variables::variables_demo();

    func::func_demo();

    ownership::ownership_demo();

    types::file::types_file_demo();

    base::string::string_demo();

    base::slice::slice_demo();

    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    let _ss: [&str; 1] = ["hello"];

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    println!("{:?}", _ss);

    struct_::student_demo();

    map::map_demo();

    file::file_demo();
}
