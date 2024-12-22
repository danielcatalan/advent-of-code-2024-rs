#[allow(unused_imports)]
use once_cell::sync::Lazy;
#[allow(unused_imports)]
use regex::Regex;
use std::io::BufRead;

use crate::parse::parse_map;

/* Notes
 *
 * for regex use Lazy struct.
 * eg:
 *  static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d").unwrap());
 *
 */

pub fn solve_solution<R: BufRead>(reader: R) -> usize {
    let map = parse_map(reader);
    let tailheads = get_trailhead_positions(&map);

    let sum: usize = tailheads.iter().map(|start| traverse(&map, 0, start)).sum();

    sum
}

fn get_trailhead_positions(map: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let x = map
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(move |(col, height)| (row, col, *height))
        })
        .filter(|(_, _, height)| *height == 0)
        .map(|(row, col, _)| (row, col))
        .collect();
    x
}

fn traverse(map: &Vec<Vec<u8>>, current_height: usize, position: &(usize, usize)) -> usize {
    let row = position.0 as isize;
    let col = position.1 as isize;

    let choices = [
        (row, col + 1),
        (row + 1, col),
        (row, col - 1),
        (row - 1, col),
    ];

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::BufReader, str::FromStr};

    fn get_input() -> String {
        let input = String::from_str(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        )
        .unwrap();
        input
    }

    #[test]
    fn test_solve() {
        let input = get_input();
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution(reader);
        assert_eq!(36, solution)
    }

    #[test]
    fn test_get_trailheads() {
        let input = get_input();
        let reader = BufReader::new(input.as_bytes());
        let map = parse_map(reader);
        let trailheads = get_trailhead_positions(&map);
        assert_eq!(9, trailheads.len());
        assert!(trailheads.contains(&(0, 2)));
        assert!(trailheads.contains(&(2, 4)));
    }
}
