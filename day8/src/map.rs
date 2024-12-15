use std::collections::{HashMap, HashSet, LinkedList};

use crate::antenna::Antenna;

pub enum WithResonance {
    Yes,
    No,
}
pub struct Map {
    antennas_map: HashMap<char, Vec<Antenna>>,
    map_bounds: (usize, usize),
    with_resonance: WithResonance,
}

impl Map {
    pub fn new(with_resonance: WithResonance) -> Self {
        Map {
            antennas_map: HashMap::new(),
            map_bounds: (0, 0),
            with_resonance,
        }
    }

    pub fn push(&mut self, antenna: Antenna) {
        let freq = antenna.freq();
        self.antennas_map
            .entry(freq)
            .or_insert(Vec::new())
            .push(antenna);
    }
    pub fn set_bounds(&mut self, bounds: (usize, usize)) {
        self.map_bounds = bounds;
    }

    pub fn total_anitinode(&self) -> usize {
        //for each antenna in a specific freq
        let mut total_antinodes = HashSet::new();
        for (_, antennas_by_freq) in self.antennas_map.iter() {
            let antinodes =
                count_antinodes(antennas_by_freq, &self.map_bounds, &self.with_resonance);
            for antinode in antinodes {
                total_antinodes.insert(antinode);
            }
        }
        total_antinodes.len()
    }
}

fn count_antinodes(
    antennas: &Vec<Antenna>,
    bounds: &(usize, usize),
    with_resonance: &WithResonance,
) -> HashSet<(usize, usize)> {
    let mut antennas: LinkedList<&Antenna> = antennas.iter().collect();

    let mut total_antinodes: HashSet<(usize, usize)> = HashSet::new();
    while antennas.len() > 1 {
        let antenna_a = antennas.pop_front().unwrap();

        for antenna_b in antennas.iter() {
            let nodes = get_antinodes(antenna_a, antenna_b, bounds, with_resonance);
            for node in nodes {
                total_antinodes.insert(node);
            }
        }
    }
    total_antinodes
}

fn get_antinodes(
    a: &Antenna,
    b: &Antenna,
    bounds: &(usize, usize),
    with_resonance: &WithResonance,
) -> HashSet<(usize, usize)> {
    let mut antinodes = HashSet::new();

    let get_antinode = match with_resonance {
        WithResonance::No => get_antinode1,
        WithResonance::Yes => get_antinode2,
    };

    for nodes in get_antinode(a, b, bounds) {
        antinodes.insert(nodes);
    }
    for nodes in get_antinode(b, a, bounds) {
        antinodes.insert(nodes);
    }
    antinodes
}

fn get_antinode1(a: &Antenna, b: &Antenna, bounds: &(usize, usize)) -> Vec<(usize, usize)> {
    let pos_a = ((a.position().0 as i64), (a.position().1 as i64));
    let pos_b = ((b.position().0 as i64), (b.position().1 as i64));

    let row_diff = pos_a.0 - pos_b.0;
    let col_diff = pos_a.1 - pos_b.1;

    let antinode = ((pos_a.0 + row_diff), (pos_a.1 + col_diff));
    if antinode.0 >= 0
        && antinode.0 < bounds.0 as i64
        && antinode.1 >= 0
        && antinode.1 < bounds.1 as i64
    {
        return vec![(antinode.0 as usize, antinode.1 as usize)];
    } else {
        return vec![];
    }
}
fn get_antinode2(a: &Antenna, b: &Antenna, bounds: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut nodes = vec![a.position().to_owned(), b.position().to_owned()];
    let pos_a = ((a.position().0 as i64), (a.position().1 as i64));
    let pos_b = ((b.position().0 as i64), (b.position().1 as i64));

    let row_diff = pos_a.0 - pos_b.0;
    let col_diff = pos_a.1 - pos_b.1;

    let mut prev_pos = pos_a;
    loop {
        let antinode = ((prev_pos.0 + row_diff), (prev_pos.1 + col_diff));
        if antinode.0 >= 0
            && antinode.0 < bounds.0 as i64
            && antinode.1 >= 0
            && antinode.1 < bounds.1 as i64
        {
            nodes.push((antinode.0 as usize, antinode.1 as usize));
            prev_pos = antinode;
        } else {
            return nodes;
        }
    }
}
