use crate::operator::Ops;

pub struct Equation {
    pub expected: usize,
    pub numbers: Vec<usize>,
}

impl Equation {
    pub fn new(expected: usize, numbers: Vec<usize>) -> Self {
        Equation { expected, numbers }
    }

    pub fn num_len(&self) -> usize {
        self.numbers.len()
    }

    pub fn valid_ops(&self, ops: &Vec<Ops>) -> bool {
        if self.numbers.len() != ops.len() {
            panic!("Expected length of numbers and operations to be same")
        }

        let mut ops = ops.iter();
        let result: usize = self
            .numbers
            .iter()
            .map(|num| *num)
            .reduce(|acc, num| {
                let op = ops.next().unwrap();
                op.calc(&acc, &num)
            })
            .unwrap();

        if self.expected == result {
            return true;
        } else {
            return false;
        }
    }
}
