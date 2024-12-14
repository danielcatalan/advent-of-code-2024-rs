use std::{collections::HashSet, io::BufRead};

use once_cell::sync::Lazy;
use regex::Regex;

use crate::{antenna::Antenna, map::Map};

pub fn parse_map<R: BufRead>(reader: R) -> Map {
    let lines = reader.lines();

    let mut antennas = Vec::new();
    let mut set_of_freqs = HashSet::new();
    let mut row_limit = 0;
    let mut col_limit = 0;
    for (row, line) in lines.map(|s| s.unwrap()).enumerate() {
        row_limit = row + 1;
        col_limit = line.len();
        let mut antennas_in_row = parse_to_antennas(row, &line);

        let _ = antennas_in_row
            .iter()
            .map(|a| set_of_freqs.insert(a.freq()));
        antennas.append(&mut antennas_in_row);
    }
    Map::new(set_of_freqs, antennas, (row_limit, col_limit))
}

fn parse_to_antennas(row: usize, line: &str) -> Vec<Antenna> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[a-zA-Z0-9]").unwrap());

    let antennas: Vec<Antenna> = RE
        .find_iter(line)
        .map(|m| {
            let freq = m.as_str().chars().next().unwrap();
            let col = m.start();
            Antenna::new(freq, (row, col))
        })
        .collect();
    antennas
}
