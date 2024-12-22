use std::collections::LinkedList;

pub struct PlutoStones {
    stones: LinkedList<usize>,
}

impl PlutoStones {
    pub fn new<I: Iterator<Item = usize>>(stones: I) -> Self {
        PlutoStones {
            stones: LinkedList::from_iter(stones),
        }
    }

    pub fn blink(&mut self) {
        let mut new_set = LinkedList::new();
        for stone in self.stones.iter() {
            let mut children = change(stone);
            new_set.append(&mut children);
        }
        self.stones = new_set;
    }

    pub fn len(&self) -> usize {
        self.stones.len()
    }
}

fn change(stone: &usize) -> LinkedList<usize> {
    if *stone == 0 {
        return LinkedList::from([1]);
    }

    let number = stone.to_string();
    let length = number.len();
    if (length % 2) == 1 {
        return LinkedList::from([stone * 2024]);
    }
    let x = length / 2;
    let left: usize = number[0..x].parse().unwrap();
    let right: usize = number[x..length].parse().unwrap();

    return LinkedList::from([left, right]);
}
