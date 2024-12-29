use std::io::BufRead;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::robot::Robot;

pub fn parse_input<R: BufRead>(reader: R) -> Vec<Robot> {
    let lines = reader.lines();

    let robots = lines.map(|line| parse_line(&line.unwrap())).collect();

    robots
}

pub fn parse_line(line: &str) -> Robot {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"p=(?P<px>\d+),(?P<py>\d+) v=(?P<vx>-?\d+),(?P<vy>-?\d+)").unwrap()
    });

    let caps = RE.captures(line).unwrap();
    let px: usize = caps["px"].parse().unwrap();
    let py: usize = caps["py"].parse().unwrap();
    let vx: isize = caps["vx"].parse().unwrap();
    let vy: isize = caps["vy"].parse().unwrap();
    Robot::new((px, py), (vx, vy))
}
