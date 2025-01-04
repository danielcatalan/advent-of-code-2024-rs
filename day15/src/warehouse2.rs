use crate::{movement::Movement, space::Space};

pub struct Warehouse2 {
    robot_position: (usize, usize),
    warehouse_space: Vec<Vec<Space>>,
}

impl Warehouse2 {
    pub fn new(robot_position: (usize, usize), warehouse_space: Vec<Vec<Space>>) -> Self {
        Warehouse2 {
            robot_position,
            warehouse_space,
        }
    }

    pub fn move_robot(&mut self, movement: Movement) {
        let pos = self.robot_position;

        if let Ok(new_pos) = self.move_object(pos, movement) {
            self.robot_position = new_pos
        }
    }

    fn move_object(
        &mut self,
        current_position: (usize, usize),
        movement: Movement,
    ) -> Result<(usize, usize), ()> {
        let new_position = movement.next_pos(current_position);
        let object = self.get_object(&new_position);

        if let Space::Wall = object {
            return Err(()); // next position is Wall, do nothing
        } else if let Space::Empty = object {
            self.swap(current_position, new_position);
            return Ok(new_position); // next position is Empty. do swap
        } else {
            // Box
            //check if Box's next position is empty
            if let Ok(_) = self.move_object(new_position, movement) {
                self.swap(current_position, new_position);
                return Ok(new_position); // next position is Empty. do swap
            } else {
                return Err(());
            }
        }
    }

    pub fn sum_of_gps_coordinates(&self) -> usize {
        let mut sum = 0;
        for (row, line) in self.warehouse_space.iter().enumerate() {
            for (col, object) in line.iter().enumerate() {
                if let Space::BoxHead = object {
                    sum += 100 * row + col;
                }
            }
        }
        sum
    }

    fn get_object(&self, (row, col): &(usize, usize)) -> &Space {
        &self.warehouse_space[*row][*col]
    }

    fn swap(&mut self, (r1, c1): (usize, usize), (r2, c2): (usize, usize)) {
        let temp = self.warehouse_space[r1][c1].clone();
        self.warehouse_space[r1][c1] = self.warehouse_space[r2][c2].clone();
        self.warehouse_space[r2][c2] = temp
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::warehouse;

    use super::*;

    fn create_warehouse() -> Warehouse2 {
        let input = String::from_str(
            "####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################",
        )
        .unwrap();
        let mut spaces = Vec::new();
        let mut robot_pos = None;
        for (row, line) in input.lines().enumerate() {
            let mut buf = Vec::new();
            for (col, ch) in line.as_bytes().iter().enumerate() {
                let space = match ch {
                    b'#' => Space::Wall,
                    b'.' => Space::Empty,
                    b'[' => Space::BoxHead,
                    b']' => Space::BoxTail,
                    b'@' => {
                        robot_pos = Some((row, col));
                        Space::Empty
                    }
                    _ => panic!("Unexpected character found"),
                };
                buf.push(space);
            }
            spaces.push(buf);
        }
        let warehouse = Warehouse2::new(robot_pos.unwrap(), spaces);
        warehouse
    }

    #[test]
    fn test_sum_of_gps_coordinates() {
        let warehouse = create_warehouse();

        assert_eq!(9021, warehouse.sum_of_gps_coordinates())
    }
}
