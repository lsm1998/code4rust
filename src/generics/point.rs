use std::ops::Add;

#[derive(Debug)]
// 限制类型T必须实现了Add特征，否则无法进行+操作。
struct Point<T: Add<T, Output=T>> {
    x: T,
    y: T,
}

impl<T: Add<T, Output=T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T> {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<T, Output=T>>(a: T, b: T) -> T {
    a + b
}

pub fn point_demo() {
    let p1 = Point { x: 1.1f32, y: 1.1f32 };
    let p2 = Point { x: 2.1f32, y: 2.1f32 };
    println!("{:?}", add(p1, p2));

    let p3 = Point { x: 1i32, y: 1i32 };
    let p4 = Point { x: 2i32, y: 2i32 };
    println!("{:?}", add(p3, p4));
}
