#[allow(unused_imports)]
use once_cell::sync::Lazy;
#[allow(unused_imports)]
use regex::Regex;
use std::{collections::HashMap, io::BufRead};

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
    let quadrants = robots.iter().map(|robot| {
        let pos = robot.get_position_at_x_sec(map, 100);
        quadrant_finder.get_quad(pos)
    });

    let mut quadrant_counter: HashMap<Quadrant, usize> = HashMap::new();
    for quad in quadrants {
        if let Some(quad) = quad {
            let counter = quadrant_counter.entry(quad).or_insert(0);
            *counter += 1;
        }
    }
    let safty_score = quadrant_counter
        .iter()
        .map(|(_, count)| *count)
        .fold(1, |acc, x| acc * x);
    safty_score
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::BufReader, str::FromStr};

    #[test]
    fn test_solve() {
        let input = String::from_str(
            "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
        )
        .unwrap();
        let reader = BufReader::new(input.as_bytes());
        let robots = parse_input(reader);
        let map = (11, 7);
        let solution = solve_solution_impl(&robots, &map);
        assert_eq!(12, solution);
    }
}
