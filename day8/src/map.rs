use std::collections::HashSet;

use crate::antenna::Antenna;

pub struct Map {
    set_of_freqs: HashSet<char>,
    antennas: Vec<Antenna>,
    map_bounds: (usize, usize),
}

impl Map {
    pub fn new(
        set_of_freqs: HashSet<char>,
        antennas: Vec<Antenna>,
        map_bounds: (usize, usize),
    ) -> Self {
        Map {
            set_of_freqs,
            antennas,
            map_bounds,
        }
    }

    pub fn total_anitinode(&self) -> usize {
        for freq in self.set_of_freqs.iter() {}
        todo!("Finish Function")
    }
}
