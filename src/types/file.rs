#![allow(unused_variables)]

type File = String;

fn open(f: &mut File) -> bool {
    println!("open file {}", f);
    true
}

fn close(f: &mut File) -> bool {
    println!("close file {}", f);
    true
}

#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}

pub fn types_file_demo() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    close(&mut f1);
    // read(&mut f1, &mut vec![]);
}
