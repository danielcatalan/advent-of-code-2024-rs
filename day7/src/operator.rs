pub struct OpSetIter {
    combo_lim: usize,
    state: usize,
}

impl OpSetIter {
    pub fn new(length: usize) -> Self {
        let length = length - 1;
        let combo_lim = 1 << length;
        OpSetIter {
            combo_lim,
            state: 0,
        }
    }
}

impl Iterator for OpSetIter {
    type Item = Vec<Ops>;

    fn next(&mut self) -> Option<Self::Item> {
        let state = self.state;

        todo!()
    }
}

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
