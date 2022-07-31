// Box 的使用场景
// 1.使用 Box<T> 将数据存储在堆上
// 2.避免栈上数据的拷贝
// 3.将动态大小类型变为 Sized 固定大小类型
// 4.特征对象

pub fn box_test()
{
    // a 的值存储在堆上，并且是个i32指针类型
    let a = Box::new(3);
    let b = *a + 1;
}


enum List {
    Cons(i32, Box<List>),
    Nil,
}

