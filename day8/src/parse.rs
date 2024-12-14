use std::{collections::HashSet, io::BufRead};

use once_cell::sync::Lazy;
use regex::Regex;

use crate::{antenna::Antenna, map::Map};

pub fn parse_map<R: BufRead>(reader: R) -> Map {
    let lines = reader.lines();

    let mut antennas = Vec::new();
    let mut set_of_freqs = HashSet::new();
    for (row, line) in lines.enumerate() {
        let mut antennas_in_row = parse_to_antennas(row, &line.unwrap());

        let _ = antennas_in_row
            .iter()
            .map(|a| set_of_freqs.insert(a.freq()));
        antennas.append(&mut antennas_in_row);
    }
    Map::new(set_of_freqs, antennas)
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
