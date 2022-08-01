mod thread_base;
mod thread_mutex;

pub fn thread_demo()
{
    thread_base::join_thread_demo();
    thread_base::move_thread_demo();
    thread_base::barrier_thread_demo();
    thread_mutex::thread_add_demo();
}
