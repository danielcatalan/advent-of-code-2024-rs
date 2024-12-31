use std::io::{BufRead, Lines};

pub fn parser<R: BufRead>(reader: R) -> (Vec<Vec<u8>>, Vec<u8>){
    let mut lines = reader.lines();

    let mut warehouse: Vec<Vec<u8>> = Vec::new();
    // parse warehouse
    while let Some(line) = lines.next() {
        if let Some(row) = parse_warehouse_line(&line.unwrap()) {
            warehouse.push(row);
        } else {
            break;
        }
    }
    // parse movements
    let movements = parse_movements(lines);
    (warehouse,movements)
}

fn parse_warehouse_line(line: &str) -> Option<Vec<u8>> {
    if line.len() == 0 {
        return None;
    }

    let row: Vec<_> = line.as_bytes().iter().map(|c| *c).collect();
    Some(row)
}

fn parse_movements<R: BufRead>(lines: Lines<R>) -> Vec<u8>{

    let x: Vec<u8> = lines.map(|line| line.unwrap()).flat_map(|line|{
        line.as_bytes().to_owned()
    }).collect();
    x
}
