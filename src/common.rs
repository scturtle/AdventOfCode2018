use reqwest::header::{self, HeaderMap};

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub fn get_input(day: i8) -> String {
    let filename = format!("{}.txt", day);
    let path = Path::new(&filename);
    if path.exists() {
        let mut f = File::open(path).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        return s;
    }
    let mut headers = HeaderMap::new();
    headers.insert(
        header::COOKIE,
        ("session=".to_owned() + &std::env::var("SESSION").expect("setup SESSION"))
            .parse()
            .unwrap(),
    );
    let url = format!("https://adventofcode.com/2018/day/{}/input", day);
    let url = reqwest::Url::parse(&url).unwrap();
    let s = reqwest::Client::new()
        .get(url)
        .headers(headers)
        .send()
        .unwrap()
        .text()
        .unwrap();
    File::create(path).unwrap().write_all(s.as_bytes()).unwrap();
    s
}
