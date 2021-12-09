use std::sync::{Arc, RwLock};

fn main() {
    join_thread_single_owner();
    join_thread_multi_owner();
}

fn join_thread_single_owner() {
    let arr = vec![1];

    std::thread::spawn(move || {
        println!("{:?}", arr);
    })
    .join()
    .unwrap();
}
fn join_thread_multi_owner() {
    let s = Arc::new(RwLock::new("Hello"));

    let r = s.clone();
    std::thread::spawn(move || {
        println!("{:?}", r.as_ref().read().unwrap());
    })
    .join()
    .unwrap();

    println!("{:?}", s.as_ref().read().unwrap());
}
