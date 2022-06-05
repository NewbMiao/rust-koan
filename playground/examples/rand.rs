use rand::Rng;

fn main() {
    let mut data :Vec<* const [u8]> = Vec::new();

    for _i in 0..5 {
        let mut num: Vec<u8>= Vec::new();
        for _j in 0..16 {
            let rand_num :u8 = rand::thread_rng().gen();
            num.push(rand_num);
        }
        println!("num({:p}) is : {:?}", &*num, num);
        let boxed = num.into_boxed_slice();
        data.push(Box::into_raw(boxed) as _);
    }
    println!("data is: {:?}",data);
}