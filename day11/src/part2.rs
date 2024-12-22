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
    for i in 0..75 {
        println!("Blink {i}");
        stones.blink();
    }
    stones.len()
}
