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

        let mut ops_iter = ops.iter();
        let result: usize = self.numbers.iter().map(|num| *num).fold(0, |acc, num| {
            let op = ops_iter.next().unwrap();
            op.calc(&acc, &num)
        });

        if self.expected == result {
            return true;
        } else {
            return false;
        }
    }
}
