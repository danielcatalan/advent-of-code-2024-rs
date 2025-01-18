use std::slice::Iter;

use crate::{object::Object, space::Space};

pub struct Map {
    pub value: Vec<Vec<Space>>,
}

impl Map {
    pub fn new(value: Vec<Vec<Space>>) -> Self {
        Map { value }
    }

    pub fn iter(&self) -> Iter<'_, Vec<Space>> {
        let x = self.value.iter();
        x
    }

    pub fn get_object(&mut self, (row, col): &(usize, usize)) -> Object {
        let space = &self.value[*row][*col];
        match space {
            Space::Robot => Object::Robot(self, (*row, *col)),
            Space::Empty => Object::Empty((*row, *col)),
            Space::Box => panic!("was not expecting part1 box"),
            Space::Wall => Object::Wall,
            Space::BoxHead => Object::WideBox(self, (*row, *col)),
            Space::BoxTail => Object::WideBox(self, (*row - 1, *col)),
        }
    }
    pub fn get_space(&self, (row, col): &(usize, usize)) -> &Space {
        &self.value[*row][*col]
    }

    pub fn swap(&mut self, (r1, c1): (usize, usize), (r2, c2): (usize, usize)) {
        let temp = self.value[r1][c1].clone();
        self.value[r1][c1] = self.value[r2][c2].clone();
        self.value[r2][c2] = temp
    }
}
