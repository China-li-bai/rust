use std::fs;
use std::io;
fn main() {
    let mut url = "https://zh-hans.react.dev/learn";
    io::stdin().read_line(&mut url).expect("you not set url")
    let output = "rust.md";
    println!("{}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    println!("converting html to md");
    let md = html2md::parse_html(&body);
    fs::write(output, md.as_bytes()).unwrap();
}
