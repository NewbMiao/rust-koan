fn main() {
    let mut data: Vec<&u32> = Vec::new();
    push_local_ref_to_heap_failed(&mut data);
    println!("data: {:?}", data);

    let r = local_ref_on_stack();
    println!("r: {:p}", r);

    local_ref_to_heap_succeed();
}
fn push_local_ref_to_heap_failed(data: &mut Vec<&u32>) {
    let v = 42;
    // borrowed value does not live long enough
    data.push(&v);
}

fn local_ref_on_stack<'a>() -> &'a i32 {
    let a = 42;
    // returns a reference to data owned by the current function
    &a
}

fn local_ref_to_heap_succeed() {
    let mut data: Vec<&u32> = Vec::new();
    let v = 42;
    data.push(&v);
    println!("data: {:?}", data);
}
