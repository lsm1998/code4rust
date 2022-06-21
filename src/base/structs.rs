#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
}

impl Student {
    pub fn set_name(&mut self, name: String)
    {
        self.name = name;
    }

    pub fn set_age(&mut self, age: u8) {
        self.age = age;
    }
}

fn build_student(name: String) -> Student {
    return Student {
        name,
        age: 0,
    };
}

pub fn student_demo()
{
    let mut s = Student { name: String::from("lsm"), age: 0 };
    s.set_name(String::from("hello lsm"));
    s.set_age(25);
    println!("{:#?}", s);

    let s = build_student(String::from("mdy"));
    dbg!(s);
}
