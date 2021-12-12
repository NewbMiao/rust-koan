pub fn strtok<'a>(s: &mut &'a str, delimiter: char) -> &'a str {
    // &mut &str 是 s的可变引用， 若可变引用生命周期标注和返回值一样（&'a mut）, 这在main中打印 s1 的只读引用 作用域会冲突
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        // 由于 delimiter 可以是 utf8，所以我们需要获得其 utf8 长度，
        // 直接使用 len 返回的是字节长度，会有问题
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        // 如果没找到，返回整个字符串，把原字符串指针 s 指向空串
        let prefix = *s;
        *s = "";
        prefix
    }
}

fn main() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ' ');
    println!("hello is: {}, s1: {} s: {}", hello, s1, s);
}
