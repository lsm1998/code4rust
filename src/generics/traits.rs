// 声明一个特征，总结
pub trait Summary {
    fn summarize(&self) -> String;
}

struct Weibo {
    // 作者
    pub username: String,
    // 内容
    pub content: String,
}

// 结构体Weibo 实现特征Summary
impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博，内容是：{}", self.username, self.content)
    }
}

struct Post {
    // 标题
    pub title: String,
    // 作者
    pub author: String,
    // 内容
    pub content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}

// 编译报错，只能有一个具体的类型
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         Post {
//             title: "".to_string(),
//             author: "".to_string(),
//             content: "".to_string(),
//         }
//     } else {
//         Weibo {
//             username: "".to_string(),
//             content: "".to_string(),
//         }
//     }
// }

pub fn trait_demo() {
    let weibo = Weibo { username: "sunface".to_string(), content: "好像微博没Tweet好用".to_string() };
    println!("{}", weibo.summarize());
}
