pub struct Antenna {
    freq: char,
    position: (usize, usize),
}

impl Antenna {
    pub fn new(freq: char, position: (usize, usize)) -> Self {
        Antenna { freq, position }
    }
    pub fn freq(&self) -> char {
        self.freq
    }
    pub fn position(&self) -> &(usize, usize) {
        &self.position
    }
}
