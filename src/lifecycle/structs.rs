pub fn struct_demo()
{
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part)
}

struct ImportantExcerpt<'a> {
    part: &'a str
}
