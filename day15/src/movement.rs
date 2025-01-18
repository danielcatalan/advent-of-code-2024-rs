pub enum Movement {
    Up,
    Down,
    Left,
    Right,
}
impl Movement {
    pub fn next_pos(&self, (row, col): &(usize, usize)) -> (usize, usize) {
        match self {
            Movement::Up => (row - 1, *col),
            Movement::Down => (row + 1, *col),
            Movement::Left => (*row, col - 1),
            Movement::Right => (*row, col + 1),
        }
    }
}
