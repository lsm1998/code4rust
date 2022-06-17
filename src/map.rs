use std::collections::HashMap;

struct Todo {
    // 使用 Rust 内置的 HashMap 来保存 key - val 键值对。
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
        // 在我们的 map 中新增一个新的元素。
        // 我们默认将其状态值设置为 true
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }
}

pub fn map_demo()
{
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    let mut todo = Todo {
        map: HashMap::new(),
    };
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    }
}
