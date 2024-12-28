use std::io::BufRead;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::claw_machine::{AButton, BButton, ClawMachine};

pub fn parse_input<R: BufRead>(reader: R, corrected: bool) -> Vec<ClawMachine> {
    let mut lines = reader.lines();
    let mut claw_machines = vec![];

    let cm_construct = if corrected {
        ClawMachine::new_corrected
    } else {
        ClawMachine::new
    };

    loop {
        if let Some(line_butt_a) = lines.next() {
            if let Some(line_butt_b) = lines.next() {
                if let Some(line_prize) = lines.next() {
                    let (movex, movey) = parse_button(&line_butt_a.unwrap());
                    let button_a = AButton(movex, movey);

                    let (movex, movey) = parse_button(&line_butt_b.unwrap());
                    let button_b = BButton(movex, movey);

                    let prize_pos = parse_prize(&line_prize.unwrap());

                    claw_machines.push(cm_construct(button_a, button_b, prize_pos));
                    let _ = lines.next(); // should be empty
                } else {
                    break claw_machines;
                }
            } else {
                break claw_machines;
            }
        } else {
            break claw_machines;
        }
    }
}

fn parse_button(line: &str) -> (usize, usize) {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"Button .: X\+(?P<x>\d+), Y\+(?P<y>\d+)").unwrap());
    let caps = RE.captures(line).unwrap();
    let x: usize = caps["x"].parse().unwrap();
    let y: usize = caps["y"].parse().unwrap();
    (x, y)
}

fn parse_prize(line: &str) -> (usize, usize) {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"Prize: X=(?P<x>\d+), Y=(?P<y>\d+)").unwrap());
    let caps = RE.captures(line).unwrap();
    let x: usize = caps["x"].parse().unwrap();
    let y: usize = caps["y"].parse().unwrap();
    (x, y)
}
