use std::io::BufRead;

pub fn generate_filesystem(diskmap: Vec<u8>) -> (Vec<Option<usize>>, usize) {
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
    (filesystem, id)
}

pub fn parse_diskmap<R: BufRead>(reader: R) -> Vec<u8> {
    reader
        .bytes()
        .map(|b| b.unwrap())
        .filter(|b| *b >= b'0')
        .map(|b| b - b'0')
        .collect()
}

fn get_files(id: &usize, file_len: &u8) -> Vec<Option<usize>> {
    let x: Vec<Option<usize>> = (0..*file_len).map(|_| Some(*id)).collect();
    x
}

fn get_empty(empty_len: &u8) -> Vec<Option<usize>> {
    let x: Vec<Option<usize>> = (0..*empty_len).map(|_| None).collect();
    x
}

pub fn calc_checksum(filesystem: &[Option<usize>]) -> usize {
    let sum: usize = filesystem
        .iter()
        .filter(|x| x.is_some())
        .enumerate()
        .map(|(i, x)| i * x.unwrap())
        .sum();

    sum
}

pub fn rearange1(filesystem: &mut Vec<Option<usize>>) {
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
}

pub fn rearange2(filesystem: &mut Vec<Option<usize>>, ids: usize) {
    for id in (0..ids).rev() {
        let (start_f, end_f) = get_start_end(&filesystem, id);

        if let Some((start_e, end_e)) = find_empty(&filesystem, start_f, end_f) {
            let it = (start_f..end_f).zip(start_e..end_e);
            for (f, e) in it {
                filesystem.swap(f, e);
            }
        }
    }
}

fn get_start_end(filesystem: &Vec<Option<usize>>, id: usize) -> (usize, usize) {
    let mut i = filesystem.len() - 1;

    // Find end
    let end = loop {
        if i == 0 {
            panic!("unexpected end at 0");
        }
        if let Some(x) = filesystem[i] {
            if x == id {
                break i;
            }
        }
        i -= 1;
    };

    // Find start
    i = end;
    let start = loop {
        if i == 0 {
            break 0;
        }
        if let Some(x) = filesystem[i] {
            if x == id {
                i -= 1;
                continue;
            }
        }
        break i + 1;
    };

    (start, end + 1)
}

fn find_empty(filesystem: &Vec<Option<usize>>, start: usize, end: usize) -> Option<(usize, usize)> {
    let mut i = 0;
    let length = end - start;
    loop {
        let start_e = loop {
            if i >= filesystem.len() {
                return None;
            }
            if i >= start {
                return None;
            }
            if let None = filesystem[i] {
                break i;
            }
            i += 1;
        };
        i += 1;
        let end_e = loop {
            if i >= filesystem.len() {
                return None;
            }
            if i - start_e == length {
                break i;
            }
            if let Some(_) = filesystem[i] {
                break i;
            }
            i += 1;
        };
        if end_e - start_e >= length {
            return Some((start_e, end_e));
        }
    }
}
