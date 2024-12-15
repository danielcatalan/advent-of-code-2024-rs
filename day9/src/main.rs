mod part1;
mod part2;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[allow(dead_code)]
enum SolutionSolver {
    Part1,
    Part2,
}

fn main() {
    let part = SolutionSolver::Part1;

    println!("Calculating solution...\n");
    let f = File::open("input/input.txt").unwrap();

    let reader = BufReader::new(f);
    solution(reader, part);

    println!("\n...Done.")
}

fn solution<R: BufRead>(reader: R, part: SolutionSolver) {
    match part {
        SolutionSolver::Part1 => {
            let solution = part1::solve_solution(reader);
            println!("Solution1: {solution}");
        }
        SolutionSolver::Part2 => {
            let solution = part2::solve_solution(reader);
            println!("Solution2: {solution}");
        }
    }
}

// #[cfg(not(feature = "dpart1"))]
// fn solution<R: BufRead>(_reader: R) -> u32 {
//     part2::solve_solution(reader)
// }
