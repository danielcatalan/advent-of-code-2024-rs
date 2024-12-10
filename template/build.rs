use std::{fs::File, io::Write};


const COOKIE_VALUE: &str = "place-cookie-here";

fn main(){
    let year = {{year}};
    let day = {{day}};
    let url_day = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let header_value = format!("session={}; Domain=.adventofcode.com", COOKIE_VALUE);

    let body = ureq::get(url_day.as_str())
        .set("Cookie", header_value.as_str())
        .call().unwrap()
        .into_string().unwrap();
    let mut f = File::create("input/input.txt").unwrap();

    f.write_all(body.as_bytes()).unwrap();
}
