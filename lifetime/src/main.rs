fn main() {
    let s1 = String::from("Lindsey");
    let s2 = String::from("Rosie");

    let result = max(&s1, &s2);

    println!("bigger one: {}", result);

    let result = get_max(&s1);
    println!("bigger one: {}", result);
}

fn get_max(s1: &str) -> &str {
    // s1 动态生命周期，而字符串字面量是静态
    max(s1, "Cynthia")
}

// 生命周期参数，描述的是参数和参数之间、参数和返回值之间的关系，并不改变原有的生命周期
fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    match s1 > s2 {
        true => s1,
        false => s2,
    }
}
