// 声明一个特征，总结
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Weibo {
    pub username: String,
    // 作者
    pub content: String, // 内容
}

// 结构体Weibo 实现特征Summary
impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博，内容是：{}", self.username, self.content)
    }
}

pub fn trait_demo() {
    let weibo = Weibo { username: "sunface".to_string(), content: "好像微博没Tweet好用".to_string() };
    println!("{}", weibo.summarize());
}
