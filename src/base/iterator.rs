pub(crate) fn iterator_demo() {
    let arr = [1, 2, 3];
    for v in arr.into_iter() {
        println!("{}", v);
    }
}


pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
