#[allow(unused_imports)]
use once_cell::sync::Lazy;
#[allow(unused_imports)]
use regex::Regex;
use std::io::BufRead;

use crate::{equation::Equation, operator::OpSetIter, parse::parse_to_equations};

/* Notes
 *
 * for regex use Lazy struct.
 * eg:
 *  static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d").unwrap());
 *
 */

pub fn solve_solution<R: BufRead>(reader: R) -> u32 {
    let x: usize = reader
        .lines()
        .map(|line| parse_to_equations(&line.unwrap()))
        .filter(|e| possibly_true(&e))
        .map(|e| e.expected)
        .sum();

    x as u32
}

fn possibly_true(eq: &Equation) -> bool {
    let len = eq.num_len();

    let opset_iter = OpSetIter::new(len);
    for ops in opset_iter {
        if eq.valid_ops(&ops) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::BufReader, str::FromStr};

    #[test]
    fn test_solve() {
        let input = String::from_str(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        )
        .unwrap();
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution(reader);
        assert_eq!(3749, solution)
    }
}
