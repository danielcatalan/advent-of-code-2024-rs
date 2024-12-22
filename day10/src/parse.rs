use std::io::BufRead;

pub fn parse_map<R: BufRead>(reader: R) -> Vec<Vec<u8>> {
    let map = reader
        .lines()
        .map(|line| {
            let row: Vec<_> = line.unwrap().bytes().map(|b| b - b'\x30').collect();
            row
        })
        .collect();

    map
}
