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
    let s = Student { name: String::from("lsm"), age: 0 };
    println!("{:?}", s.name);
}
