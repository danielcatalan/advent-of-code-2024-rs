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

    total_regions.get_pricing()
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
        test(input, 140)
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
        test(input, 772)
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
        test(input, 1930)
    }

    fn test(input: String, expected: usize) {
        let reader = BufReader::new(input.as_bytes());
        let solution = solve_solution(reader);
        assert_eq!(expected, solution)
    }
}
