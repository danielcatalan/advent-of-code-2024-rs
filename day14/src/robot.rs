pub struct Robot {
    position: (usize, usize),
    velocity: (isize, isize),
}

impl Robot {
    pub fn new(position: (usize, usize), velocity: (isize, isize)) -> Self {
        Robot { position, velocity }
    }

    pub fn get_position_at_x_sec(&self, (mx, my): &(usize, usize), sec: usize) -> (usize, usize) {
        let x = (self.position.0 as isize) + self.velocity.0 * (sec as isize);
        let x = modulo(x, *mx);

        let y = (self.position.1 as isize) + self.velocity.1 * (sec as isize);
        let y = modulo(y, *my);

        (x, y)
    }
}

fn modulo(a: isize, b: usize) -> usize {
    let b = b as isize;
    if a >= 0 {
        (a % b) as usize
    } else {
        (((a % b) + b) % b) as usize
    }
}
