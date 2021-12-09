fn main() {
    // 可变数组 传参时move
    let data = vec![10, 42, 9, 8];
    let v = 42; // u32 实现了 copy，传参时copy
    if let Some(pos) = find_pos(data, v) {
        println!("Found {} at {}", v, pos);
    }
    println!("v {:?}", v);
    // println!("data {:?}", data); // move 后不可引用访问（borrow）
}
fn find_pos(data: Vec<u32>, v: u32) -> Option<usize> {
    for (pos, item) in data.iter().enumerate() {
        if *item == v {
            return Some(pos);
        }
    }
    None
}
