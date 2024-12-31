use std::io::BufRead;

pub fn parser<R: BufRead>(reader: R) {
    let mut lines = reader.lines();

    let mut warehouse: Vec<Vec<u8>> = Vec::new();
    // parse warsehouse
    while let Some(line) = lines.next() {
        if let Some(row) = parse_warehouse_line(&line.unwrap()) {
            warehouse.push(row);
        } else {
            break;
        }
    }

    while let Some(line) = lines.next() {}
}

fn parse_warehouse_line(line: &str) -> Option<Vec<u8>> {
    if line.len() == 0 {
        return None;
    }

    let row: Vec<_> = line.as_bytes().iter().map(|c| *c).collect();
    Some(row)
}
