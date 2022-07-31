mod r#box;
mod my_box;
mod drop;
mod rc_arc;
mod cell;

pub fn smart_ptr_demo()
{
    drop::drop_demo();
    rc_arc::rc_demo();
    cell::cell_demo();
}
