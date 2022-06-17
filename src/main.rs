mod string;
mod struct_;
mod map;

use std::mem;

fn main() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    let _ss: [&str; 1] = ["hello"];

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    println!("{:?}", _ss);

    string::string_demo();

    struct_::student_demo();

    map::map_demo();
}
