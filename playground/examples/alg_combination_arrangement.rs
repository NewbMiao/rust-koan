/// https://www.cnblogs.com/1024th/p/10623541.html
fn get_combination_with_repeat(n: usize, chars: &str, result: String, all: &mut Vec<String>) {
    if result.len() == n {
        all.push(result);
    } else {
        for (i, c) in chars.chars().enumerate() {
            let mut new_res = result.clone();
            new_res.push(c);
            get_combination_with_repeat(n, &chars[i..], new_res, all);
        }
    }
}
fn get_combination_without_repeat(n: usize, chars: &str, result: String, all: &mut Vec<String>) {
    if result.len() == n {
        all.push(result);
    } else {
        for (i, c) in chars.chars().enumerate() {
            let mut new_res = result.clone();
            new_res.push(c);
            get_combination_without_repeat(n, &chars[i + 1..], new_res, all);
        }
    }
}
fn get_combination_without_repeat1(
    n: usize,
    chars: &str,
    result: &mut String,
    all: &mut Vec<String>,
) {
    if result.len() == n {
        all.push(result.clone());
    } else {
        for (i, c) in chars.chars().enumerate() {
            result.push(c);
            get_combination_without_repeat1(n, &chars[i + 1..], result, all);
            result.pop();
        }
    }
}
fn get_arrangement_with_repeat(n: usize, chars: &str, result: String, all: &mut Vec<String>) {
    if result.len() == n {
        all.push(result);
    } else {
        for c in chars.chars() {
            let mut new_res = result.clone();
            new_res.push(c);
            get_arrangement_with_repeat(n, chars, new_res, all);
        }
    }
}
fn get_arrangement_without_repeat(n: usize, chars: &str, result: String, all: &mut Vec<String>) {
    if result.len() == n {
        all.push(result);
    } else {
        for (i, c) in chars.chars().enumerate() {
            let mut new_res = result.clone();
            new_res.push(c);
            let mut new_chars = chars.to_string();
            new_chars.remove(i);
            get_arrangement_without_repeat(n, &new_chars, new_res, all);
        }
    }
}
fn main() {
    let num = 2;
    let chars = "abcd";
    let mut all: Vec<String> = Vec::new();
    get_combination_with_repeat(num, chars.clone(), String::new(), &mut all);
    println!("get_combination_with_repeat: {:?}", all);

    let mut all: Vec<String> = Vec::new();
    get_arrangement_with_repeat(num, chars.clone(), String::new(), &mut all);
    println!("get_arrangement_with_repeat: {:?}", all);

    let mut all: Vec<String> = Vec::new();
    get_combination_without_repeat(num, chars.clone(), String::new(), &mut all);
    println!("get_combination_without_repeat: {:?}", all);

    let mut all: Vec<String> = Vec::new();
    get_combination_without_repeat1(num, chars.clone(), &mut String::new(), &mut all);
    println!("get_combination_without_repeat: {:?}", all);

    let mut all: Vec<String> = Vec::new();
    get_arrangement_without_repeat(num, chars.clone(), String::new(), &mut all);
    println!("get_arrangement_without_repeat: {:?}", all);
}
