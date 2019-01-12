use reqwest::header::{self, HeaderMap};

use std::fs::File;
use std::io::{Read, Write, Error, ErrorKind};
use std::path::Path;

pub fn get_input(day: i8) -> std::io::Result<String> {
    let filename = format!("{}.txt", day);
    let path = Path::new(&filename);
    if path.exists() {
        let mut f = File::open(path)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        return Ok(s);
    }
    let mut headers = HeaderMap::new();
    headers.insert(
        header::COOKIE,
        ("session=".to_owned() + &std::env::var("SESSION").expect("env SESSION"))
            .parse().unwrap(),
    );
    let url = format!("https://adventofcode.com/2018/day/{}/input", day);
    let url = reqwest::Url::parse(&url).unwrap();
    let s = reqwest::Client::new()
        .get(url)
        .headers(headers)
        .send()
        .and_then(|mut r| r.text())
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?
        ;
    File::create(path)?.write_all(s.as_bytes())?;
    Ok(s)
}
