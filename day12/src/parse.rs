use std::io::BufRead;

use crate::garden::Garden;

pub fn parse_input<R: BufRead>(reader: R) -> Garden {
    let mut garden = Garden::new();
    for line in reader.lines() {
        let row: Vec<u8> = line.unwrap().into();
        garden.add_row(row);
    }
    garden
}
