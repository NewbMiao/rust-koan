use std::{env, error, fs};

fn main() -> Result<(), Box<dyn error::Error>> {
    let url = env::args().nth(1).expect("Missing argument: url");
    let output = env::args().nth(2).expect("Missing argument: output");
    println!("Args=> Url: {}, Output: {}", url, output);
    return covert2md(&url, &output);
}

fn covert2md(url: &String, output: &String) -> Result<(), Box<dyn error::Error>> {
    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", output);
    Ok(())
}
