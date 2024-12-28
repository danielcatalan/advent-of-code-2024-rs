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
    let claw_machines = parse_input(reader, true);
    let mut token_sum = 0;
    for claw_machine in claw_machines {
        if let Some(tokens) = claw_machine.cheapest_win() {
            token_sum += tokens;
        }
    }
    token_sum
}
