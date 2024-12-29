#[allow(unused_imports)]
use once_cell::sync::Lazy;
#[allow(unused_imports)]
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

use crate::{
    parse::parse_input,
    quadrant::{QuadFinder, Quadrant},
    robot::Robot,
};

/* Notes
 *
 * for regex use Lazy struct.
 * eg:
 *  static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d").unwrap());
 *
 */

pub fn solve_solution<R: BufRead>(reader: R) -> usize {
    let robots = parse_input(reader);
    let map = (101, 103);
    solve_solution_impl(&robots, &map)
}

pub fn solve_solution_impl(robots: &Vec<Robot>, map: &(usize, usize)) -> usize {
    let quadrant_finder = QuadFinder::new(map);
    let mut seconds = 0;
    loop {
        let mut printer: Vec<Vec<u8>> = vec![vec![b'.'; map.0]; map.1];

        let position_iter: Vec<_> = robots
            .iter()
            .map(|robot| {
                let pos = robot.get_position_at_x_sec(map, seconds);
                printer[pos.1][pos.0] = b'*';
                // quadrant_finder.get_quad(pos)
                pos
            })
            .collect();
        let mut set = HashSet::new();
        let mut is_unique = true;
        for position in position_iter {
            if !set.insert(position) {
                is_unique = false;
                break;
            }
        }
        if !is_unique {
            seconds += 1;
            continue;
        } else {
            for line in printer {
                println!("{}", std::str::from_utf8(&line).unwrap());
            }
            return seconds;
        }
    }
}
