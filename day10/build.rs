use std::{
    fs::File,
    io::{Read, Write},
};

fn main() {
    let year = 2024;
    let day = 10;
    let url_day = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let mut f = File::open("../cookie.txt").unwrap();

    let mut cookie_value = String::new();
    let _ = f.read_to_string(&mut cookie_value);

    let header_value = format!(
        "session={}; Domain=.adventofcode.com",
        cookie_value.as_str()
    );

    let body = ureq::get(url_day.as_str())
        .set("Cookie", header_value.as_str())
        .call()
        .unwrap()
        .into_string()
        .unwrap();
    let mut f = File::create("input/input.txt").unwrap();

    f.write_all(body.as_bytes()).unwrap();
}
