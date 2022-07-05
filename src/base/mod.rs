mod tuple;
mod test;
mod string;
mod slice;
mod structs;
mod enums;
mod array;
mod pattern_match;
mod method;
mod collect;
mod convert;

pub fn base_demo() {
    string::string_demo();
    slice::slice_demo();
    tuple::tuple_test();
    enums::enums_demo();
    structs::student_demo();
    array::array_demo();
    pattern_match::pattern_match_demo();
    method::method_demo();
    collect::collect_demo();
    convert::convert_demo();
}
