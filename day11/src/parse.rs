use std::io::BufRead;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::pluto_stones::PlutoStones;

pub fn parse_input(line: &str) -> PlutoStones {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

    let stones = RE
        .find_iter(line)
        .map(|s| s.as_str().parse().unwrap())
        .collect();

    PlutoStones::new(stones)
}
