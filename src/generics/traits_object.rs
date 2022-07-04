use std::ops::Deref;

// 特征对象
pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 绘制按钮的代码
        println!("Button Draw!")
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 绘制SelectBox的代码
        println!("SelectBox Draw!")
    }
}

fn draw(x: &dyn Draw) {
    x.draw()
}

fn create_draw(switch: bool) -> Box<dyn Draw> {
    if switch {
        Box::new(Button {
            width: 0,
            height: 0,
            label: "".to_string(),
        })
    } else {
        Box::new(SelectBox {
            width: 0,
            height: 0,
            options: vec![],
        })
    }
}

pub fn traits_object_demo() {
    let button = create_draw(true);
    let select_box = create_draw(false);
    draw(button.deref());
    draw(select_box.deref());
}
