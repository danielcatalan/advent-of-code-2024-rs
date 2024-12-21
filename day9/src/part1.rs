#[allow(unused_imports)]
use once_cell::sync::Lazy;
#[allow(unused_imports)]
use regex::Regex;
use std::io::BufRead;

use crate::utils::{calc_checksum, generate_filesystem, parse_diskmap, rearange1};

/* Notes
 *
 * for regex use Lazy struct.
 * eg:
 *  static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d").unwrap());
 *
 */

pub fn solve_solution<R: BufRead>(reader: R) -> usize {
    let diskmap: Vec<_> = parse_diskmap(reader);

    let (mut filesystem, _) = generate_filesystem(diskmap);

    // rearange
    rearange1(&mut filesystem);

    calc_checksum(&filesystem)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::BufReader, str::FromStr};

    #[test]
    fn test_solve() {
        let input = String::from_str("2333133121414131402").unwrap();
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution(reader);
        assert_eq!(1928, solution)
    }
}
