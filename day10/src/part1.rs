#[allow(unused_imports)]
use once_cell::sync::Lazy;
#[allow(unused_imports)]
use regex::Regex;
use std::{collections::HashSet, io::BufRead};

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

    let mut sum: usize = 0;
    for start in tailheads.iter() {
        let mut heights_reached = HashSet::new();
        traverse(&map, 0, start, &mut heights_reached);
        sum += heights_reached.len();
    }

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

fn traverse(
    map: &Vec<Vec<u8>>,
    current_height: u8,
    position: &(usize, usize),
    heights_reached: &mut HashSet<(usize, usize)>,
) {
    if current_height == 9 {
        heights_reached.insert(*position);
    }
    let choices = get_choices(map, current_height, position);

    for choice in choices {
        traverse(&map, current_height + 1, &choice, heights_reached)
    }
}

fn get_choices(
    map: &Vec<Vec<u8>>,
    current_height: u8,
    position: &(usize, usize),
) -> Vec<(usize, usize)> {
    let mut choices = Vec::new();
    let row_last = map.len() - 1;
    let col_last = map[0].len() - 1;
    let row = position.0;
    let col = position.1;
    let next_height = current_height + 1;
    // check right
    if (col < col_last) && map[row][col + 1] == next_height {
        choices.push((row, col + 1));
    }
    //check down
    if (row < row_last) && map[row + 1][col] == next_height {
        choices.push((row + 1, col));
    }
    //check left
    if (col > 0) && map[row][col - 1] == next_height {
        choices.push((row, col - 1));
    }
    //check up
    if (row > 0) && map[row - 1][col] == next_height {
        choices.push((row - 1, col));
    }
    choices
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
