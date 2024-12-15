#[allow(unused_imports)]
use once_cell::sync::Lazy;
#[allow(unused_imports)]
use regex::Regex;
use std::io::BufRead;

/* Notes
 *
 * for regex use Lazy struct.
 * eg:
 *  static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d").unwrap());
 *
 */

pub fn solve_solution<R: BufRead>(reader: R) -> usize {
    let diskmap: Vec<_> = reader
        .bytes()
        .map(|b| b.unwrap())
        .filter(|b| *b >= b'0')
        .map(|b| b - b'0')
        .collect();

    let space: usize = diskmap.iter().map(|b| *b as usize).sum();
    let mut filesystem: Vec<Option<usize>> = Vec::with_capacity(space);

    //Generate file-system
    let mut iter = diskmap.iter();
    let mut id = 0;
    loop {
        if let Some(file_len) = iter.next() {
            let mut files = get_files(&id, file_len);
            filesystem.append(&mut files);
            id += 1;
        } else {
            break;
        }

        if let Some(empty_len) = iter.next() {
            let mut empty = get_empty(empty_len);
            filesystem.append(&mut empty);
        } else {
            break;
        }
    }

    // rearange
    let mut start = 0;
    let mut end = filesystem.len() - 1;
    loop {
        // Find first None
        loop {
            if filesystem[start].is_some() {
                start += 1;
            } else {
                break; // found None
            }
        }
        // Find last Some
        loop {
            if filesystem[end].is_none() {
                end -= 1;
            } else {
                break; // found Some
            }
        }
        if start < end {
            filesystem.swap(start, end);
        } else {
            break;
        }
    }
    let sum: usize = filesystem
        .iter()
        .filter(|x| x.is_some())
        .enumerate()
        .map(|(i, x)| i * x.unwrap())
        .sum();

    sum
}

fn get_files(id: &usize, file_len: &u8) -> Vec<Option<usize>> {
    let x: Vec<Option<usize>> = (0..*file_len).map(|_| Some(*id)).collect();
    x
}

fn get_empty(empty_len: &u8) -> Vec<Option<usize>> {
    let x: Vec<Option<usize>> = (0..*empty_len).map(|_| None).collect();
    x
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
        assert_eq!(2858, solution)
    }
}
