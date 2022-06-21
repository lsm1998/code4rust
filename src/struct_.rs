struct Student {
    name: String,
    age: u8,
}

impl Student {
    pub fn set_name(&mut self, name: String)
    {
        self.name = name;
    }

    pub fn to_string(&mut self) -> String
    {
        return self.name.to_string();
    }
}

pub fn student_demo()
{
    let mut s = Student { name: String::from("lsm"), age: 0 };
    s.set_name(String::from("hello lsm"));
    println!("{:?}", s.name);
    // println!("{:?}", s);
}
