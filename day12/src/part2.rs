#[allow(unused_imports)]
use once_cell::sync::Lazy;
#[allow(unused_imports)]
use regex::Regex;
use std::io::BufRead;

use crate::{parse::parse_input, region::TotalRegions};

/* Notes
 *
 * for regex use Lazy struct.
 * eg:
 *  static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d").unwrap());
 *
 */

pub fn solve_solution<R: BufRead>(reader: R) -> usize {
    let garden = parse_input(reader);

    let pool = garden.generate_plant_pool();
    let mut total_regions = TotalRegions::new(&garden);

    total_regions.populate_regions(pool);

    total_regions.get_pricing2()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::BufReader, str::FromStr};

    #[test]
    fn test_solve1() {
        let input = String::from_str(
            "AAAA
BBCD
BBCC
EEEC",
        )
        .unwrap();
        test(input, 80)
    }
    #[test]
    fn test_solve2() {
        let input = String::from_str(
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
        )
        .unwrap();
        test(input, 436)
    }

    #[test]
    fn test_solve3() {
        let input = String::from_str(
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
        )
        .unwrap();
        test(input, 1206)
    }

    #[test]
    fn test_solve4() {
        let input = String::from_str(
            "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
        )
        .unwrap();
        test(input, 236)
    }

    #[test]
    fn test_solve5() {
        let input = String::from_str(
            "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
        )
        .unwrap();
        test(input, 368)
    }

    fn test(input: String, expected: usize) {
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution(reader);
        assert_eq!(expected, solution)
    }
}
