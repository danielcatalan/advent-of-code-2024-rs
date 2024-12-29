#[derive(Hash, Eq, PartialEq)]
pub enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub struct QuadFinder {
    map: (usize, usize),
}

impl QuadFinder {
    pub fn new(map: &(usize, usize)) -> Self {
        QuadFinder { map: *map }
    }

    pub(crate) fn get_quad(&self, pos: (usize, usize)) -> Option<Quadrant> {
        let middle_x = self.map.0 / 2;
        let middle_y = self.map.1 / 2;

        let left = (0..middle_x).contains(&pos.0);
        let right = ((middle_x + 1)..self.map.0).contains(&pos.0);
        let top = (0..middle_y).contains(&pos.1);
        let bottom = ((middle_y + 1)..self.map.1).contains(&pos.1);
        if left && top {
            return Some(Quadrant::TopLeft);
        } else if top && right {
            return Some(Quadrant::TopRight);
        } else if bottom && left {
            return Some(Quadrant::BottomLeft);
        } else if bottom && right {
            return Some(Quadrant::BottomRight);
        } else {
            None
        }
    }
}
