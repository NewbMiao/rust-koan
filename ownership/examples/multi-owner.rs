use std::cell::RefCell;

fn main() {
    interior_mut_ref();
    active_mut_ref();
}

// 内部可变借用 运行期检查，生命周期和作用域相同
fn interior_mut_ref() {
    let data = RefCell::new(1);
    {
        // 获得 RefCell 内部数据的可变借用
        let mut v = data.borrow_mut();
        *v += 1;
        // 提前 drop 可变引用
        // drop(v);
    }

    // 同一个作用域下，我们不能同时有活跃的可变借用和不可变借用
    println!("interior_mut_ref data: {:?}", data.borrow());

    // 隐含的 drop
    // 原本 drop(v) 发生在这里
    // drop(data);
}
// 普通借用 编译期检查，编译器的优化后生命周期粒度更小，小于作用域
fn active_mut_ref() {
    let mut v = vec![1, 2, 3];
    let data1 = &mut v[1];
    *data1 = 2;
    let data2 = &v[1];
    println!("active_mut_ref data2 {}", data2);
}
