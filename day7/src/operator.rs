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

pub struct OpSetIter2 {
    length: usize,
    state: usize,
}

impl OpSetIter2 {
    pub fn new(length: usize) -> Self {
        let length = length - 1;
        OpSetIter2 { length, state: 0 }
    }
}

impl Iterator for OpSetIter2 {
    type Item = Vec<Ops>;

    fn next(&mut self) -> Option<Self::Item> {
        let limit = 3_usize.pow(self.length as u32);
        let mut state = self.state;
        if state >= limit {
            return None;
        }

        let mut ops = vec![Ops::Add];

        for _ in 0..self.length {
            let x = state % 3;
            let op = match x {
                0 => Ops::Add,
                1 => Ops::Mult,
                2 => Ops::ConCat,
                _ => panic!("unexpect value for state % 3"),
            };
            state = state / 3;
            ops.push(op);
        }

        self.state += 1;
        Some(ops)
    }
}

#[derive(Debug)]
pub enum Ops {
    Add,
    Mult,
    ConCat,
}
impl Ops {
    pub fn calc(&self, a: &usize, b: &usize) -> usize {
        match self {
            Ops::Add => a + b,
            Ops::Mult => a * b,
            Ops::ConCat => {
                let s = format!("{a}{b}");
                s.parse().unwrap()
            }
        }
    }
}
