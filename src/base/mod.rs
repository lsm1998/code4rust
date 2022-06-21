pub mod tuple;
pub mod test;
pub mod string;
pub mod slice;
pub mod structs;
pub mod enums;
mod array;

pub fn base_demo() {
    string::string_demo();
    slice::slice_demo();
    tuple::tuple_test();
    enums::enums_demo();
    structs::student_demo();
    array::array_demo();
}
