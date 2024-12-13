use once_cell::sync::Lazy;
use regex::Regex;

use crate::equation::Equation;

pub fn parse_to_equations(line: &str) -> Equation {
    let (exp, numbers) = parse(line);
    Equation::new(exp, numbers)
}

fn parse(line: &str) -> (usize, Vec<usize>) {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?P<exp>\d+):|(?P<num>\d+)").unwrap());

    let mut matches = RE.captures_iter(line);

    let exp: usize = matches.next().unwrap()["exp"].parse().unwrap();
    let numbers: Vec<usize> = matches.map(|s| s["num"].parse().unwrap()).collect();
    (exp, numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let (expected, numbers) = parse("161011: 16 10 13");
        assert_eq!(161011, expected);
        assert_eq!([16usize, 10usize, 13usize], numbers.as_slice());
    }
}
