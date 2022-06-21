use std::fs;
use std::fs::File;

// pub fn path_exists(path: &str) {
//     let metadata = fs::metadata(path);
//     match metadata {
//         Ok(w) => println!("{:?}", w),
//         Err(e) => println!("error={:?}", e),
//     }
//     // return metadata.is_file();
// }

// #[derive(Debug,Copy,Clone)]
struct MyFile {
    filename: String,
}

impl MyFile {
    pub fn new(filename: String) -> MyFile {
        MyFile { filename }
    }

    fn crate_file(self) -> std::io::Result<()> {
        File::create(&self.filename)?;
        Ok(())
    }

    fn crate_dir(self) -> std::io::Result<()> {
        fs::create_dir_all(&self.filename)?;
        Ok(())
    }

    fn exists(self) -> bool {
        fs::metadata(&self.filename).is_ok()
    }
}

pub fn file_demo() {
    let file = MyFile { filename: String::from("demo.txt") };
    println!("{:?}", file.exists());
    // if !file.exists() {
    //     file.crate_file().expect("TODO: panic message");
    // }
}
