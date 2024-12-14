pub struct OpSetIter {
    length: usize,
    state: usize,
}

impl OpSetIter {
    pub fn new(length: usize) -> Self {
        let length = length - 1;
        OpSetIter { length, state: 0 }
    }
}

impl Iterator for OpSetIter {
    type Item = Vec<Ops>;

    fn next(&mut self) -> Option<Self::Item> {
        let limit = 1 << self.length;
        let mut state = self.state;
        if state >= limit {
            return None;
        }

        let mut ops = vec![Ops::Add];

        for i in 0..self.length {
            if (state >> i) & 1 == 0 {
                ops.push(Ops::Add);
            } else {
                ops.push(Ops::Mult)
            }
        }
        state = state + 1;
        self.state = state;
        Some(ops)
    }
}

#[derive(Debug)]
pub enum Ops {
    Add,
    Mult,
}
impl Ops {
    pub fn calc(&self, a: &usize, b: &usize) -> usize {
        match self {
            Ops::Add => a + b,
            Ops::Mult => a * b,
        }
    }
}
