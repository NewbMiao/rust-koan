fn main() {
    multi_mut_ref();
    one_mut_ref_with_multi_read_ref();
}
fn multi_mut_ref() {
    let mut data = vec![1, 2, 3];

    for item in data.iter_mut() {
        // error[E0499]: cannot borrow `data` as mutable more than once at a time
        data.push(*item + 1);
    }
}
fn one_mut_ref_with_multi_read_ref() {
    let mut data = vec![1, 2, 3];
    let data1 = vec![&data[0]];
    println!("data[0]: {:p}", &data[0]);
    for i in 0..100 {
        // error[E0502]: cannot borrow `data` as mutable because it is also borrowed as immutable
        data.push(i);
    }
    println!("data[0]: {:p}", &data[0]);
    println!("boxed: {:p}", &data1);
}
