use std::io::BufRead;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::{
    antenna::Antenna,
    map::{Map, WithResonance},
};

pub fn parse_map1<R: BufRead>(reader: R) -> Map {
    let lines = reader.lines();

    let mut row_limit = 0;
    let mut col_limit = 0;

    let mut map = Map::new(WithResonance::No);
    for (row, line) in lines.map(|s| s.unwrap()).enumerate() {
        row_limit = row + 1;
        col_limit = line.len();
        parse_to_antennas(&mut map, row, &line);
    }
    map.set_bounds((row_limit, col_limit));
    map
}
pub fn parse_map2<R: BufRead>(reader: R) -> Map {
    let lines = reader.lines();

    let mut row_limit = 0;
    let mut col_limit = 0;

    let mut map = Map::new(WithResonance::Yes);
    for (row, line) in lines.map(|s| s.unwrap()).enumerate() {
        row_limit = row + 1;
        col_limit = line.len();
        parse_to_antennas(&mut map, row, &line);
    }
    map.set_bounds((row_limit, col_limit));
    map
}

fn parse_to_antennas(map: &mut Map, row: usize, line: &str) {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[a-zA-Z0-9]").unwrap());

    for m in RE.find_iter(line) {
        let freq = m.as_str().chars().next().unwrap();
        let col = m.start();
        let antenna = Antenna::new(freq, (row, col));
        map.push(antenna);
    }
}
