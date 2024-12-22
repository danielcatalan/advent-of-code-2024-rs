#[allow(unused_imports)]
use once_cell::sync::Lazy;
#[allow(unused_imports)]
use regex::Regex;
use std::io::BufRead;

use crate::parse::parse_input;

/* Notes
 *
 * for regex use Lazy struct.
 * eg:
 *  static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d").unwrap());
 *
 */

pub fn solve_solution<R: BufRead>(reader: R) -> usize {
    let line = reader.lines().next().unwrap().unwrap();

    let mut stones = parse_input(&line);
    for _ in 0..25 {
        stones.blink();
    }
    stones.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::BufReader, str::FromStr};

    #[test]
    fn test_solve() {
        let input = String::from_str("125 17").unwrap();
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution(reader);
        assert_eq!(55312, solution)
    }
}
