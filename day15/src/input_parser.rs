use std::io::{BufRead, Lines};

use crate::{movement::Movement, space::Space, warehouse::Warehouse};

pub fn parser<R: BufRead>(reader: R) -> (Warehouse, Vec<Movement>) {
    let mut lines = reader.lines();

    //parse warehouse
    let (warehouse, position) = parse_warehouse(&mut lines);

    // parse movements
    let movements = parse_movements(lines);
    let warehouse = Warehouse::new(position, warehouse);
    (warehouse, movements)
}

fn parse_warehouse<R: BufRead>(lines: &mut Lines<R>) -> (Vec<Vec<Space>>, (usize, usize)) {
    let mut lines = lines.enumerate();
    let mut warehouse: Vec<Vec<Space>> = Vec::new();
    let mut position = Option::None;

    // parse warehouse
    while let Some((r, Ok(line))) = lines.next() {
        if line.len() == 0 {
            break;
        }
        let mut row = Vec::new();
        for (c, ch) in line.as_bytes().iter().enumerate() {
            match &ch {
                b'.' => row.push(Space::Empty),
                b'O' => row.push(Space::Box),
                b'#' => row.push(Space::Wall),
                b'@' => {
                    position = Some((r, c));
                    row.push(Space::Empty)
                }
                &a => panic!("Expected [. O # @], got {a}"),
            }
        }
        warehouse.push(row);
    }

    (warehouse, position.unwrap())
}

fn parse_movements<R: BufRead>(lines: Lines<R>) -> Vec<Movement> {
    let x: Vec<Movement> = lines
        .map(|line| line.unwrap())
        .flat_map(|line| {
            let line: Vec<Movement> = line
                .as_bytes()
                .iter()
                .map(|c| match c {
                    b'^' => Movement::Up,
                    b'<' => Movement::Left,
                    b'>' => Movement::Right,
                    b'v' => Movement::Down,
                    _ => panic!(),
                })
                .collect();
            line
        })
        .collect();
    x
}
