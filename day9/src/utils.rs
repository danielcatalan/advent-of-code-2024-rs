use std::io::BufRead;

pub fn generate_filesystem(diskmap: Vec<u8>) -> Vec<Option<usize>> {
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
    filesystem
}

pub fn parse_diskmap<R: BufRead>(reader: R) -> Vec<u8> {
    reader
        .bytes()
        .map(|b| b.unwrap())
        .filter(|b| *b >= b'0')
        .map(|b| b - b'0')
        .collect()
}

pub fn get_files(id: &usize, file_len: &u8) -> Vec<Option<usize>> {
    let x: Vec<Option<usize>> = (0..*file_len).map(|_| Some(*id)).collect();
    x
}

pub fn get_empty(empty_len: &u8) -> Vec<Option<usize>> {
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

pub fn rearange2(filesystem: &mut Vec<Option<usize>>) {
    todo!("implement function")
}
