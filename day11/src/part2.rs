#[allow(unused_imports)]
use once_cell::sync::Lazy;
#[allow(unused_imports)]
use regex::Regex;
use std::{collections::HashMap, io::BufRead};

use crate::{
    parse::parse_input,
    pluto_stones::{change, change2},
};

/* Notes
 *
 * for regex use Lazy struct.
 * eg:
 *  static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d").unwrap());
 *
 */

pub fn solve_solution<R: BufRead>(reader: R) -> usize {
    let line = reader.lines().next().unwrap().unwrap();

    let stones = parse_input(&line);
    let blink_limit = 75;
    let mut stone_count = 0;
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();
    for (i, stone) in stones.stones.iter().enumerate() {
        println!("checking stone {i}");
        stone_count += blink(*stone, blink_limit, &mut map);
    }
    stone_count
}

fn blink(stone: usize, blink_limit: usize, map: &mut HashMap<(usize, usize), usize>) -> usize {
    if blink_limit == 0 {
        return 1;
    }
    if let Some(x) = map.get(&(stone, blink_limit)) {
        return *x;
    }

    let stones = change2(&stone);
    let mut count = 0;
    for stone in stones {
        if let Some(stone) = stone {
            count += blink(stone, blink_limit - 1, map)
        }
    }
    map.insert((stone, blink_limit), count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = [125, 17];

        let count: usize = input.iter().map(|stone| blink(*stone, 25)).sum();
        assert_eq!(55312, count);
    }
}
