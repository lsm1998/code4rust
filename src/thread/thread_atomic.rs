use std::ops::Sub;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

const N_TIMES: u64 = 10000000;
const N_THREADS: usize = 10;

static R: AtomicU64 = AtomicU64::new(0);

fn add_n_times(n: u64) -> JoinHandle<()>
{
    thread::spawn(move || {
        for _ in 0..n {
            // Ordering::Relaxed 用于控制原子操作使用的内存顺序
            R.fetch_add(1, Ordering::Relaxed);
        }
    })
}

pub fn atomic_demo()
{
    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);

    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }

    for thread in threads {
        thread.join().unwrap();
    }
    println!("{:?} {:?}", N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));
    println!("{:?}", Instant::now().sub(s));
}
