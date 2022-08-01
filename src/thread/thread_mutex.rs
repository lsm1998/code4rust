use std::sync::{Arc, Mutex};
use std::thread;

pub fn thread_add_demo()
{
    let count = 10;
    let rc = Arc::new(Mutex::new(count));
    let mut handles = Vec::with_capacity(10);
    for _ in 0..10 {
        let c = rc.clone();
        let handler = thread::spawn(move || {
            let mut a = c.lock().unwrap();
            *a = *a + 1;
        });
        handles.push(handler);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("count = {:?}", count)
}
