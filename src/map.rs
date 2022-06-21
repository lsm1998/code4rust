use std::collections::HashMap;

struct Todo {
    // 使用 Rust 内置的 HashMap 来保存 key - val 键值对。
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String, value: bool) {
        // 在我们的 map 中新增一个新的元素。
        // 我们默认将其状态值设置为 true
        self.map.insert(key, value);
    }

    // fn get(&mut self, key: String) -> bool {
    //     return self.map.get(key);
    // }

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
    let mut todo = Todo {
        map: HashMap::new(),
    };
    todo.insert(String::from("lsm"), true);
    todo.insert(String::from("mdy"), true);
    // match todo.save() {
    //     Ok(_) => println!("todo saved"),
    //     Err(why) => println!("An error occurred: {}", why),
    // }
}
