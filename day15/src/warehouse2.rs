use crate::{map::Map, movement::Movement, space::Space};

pub struct Warehouse2 {
    robot_position: (usize, usize),
    map: Map,
}

impl Warehouse2 {
    pub fn new(robot_position: (usize, usize), warehouse_space: Vec<Vec<Space>>) -> Self {
        Warehouse2 {
            robot_position,
            map: Map::new(warehouse_space),
        }
    }

    pub fn move_robot(&mut self, movement: Movement) {
        let mut robot = self.map.get_object(&self.robot_position);
        if robot.is_movable(&movement) {
            let new_pos = robot.do_move(&movement);
            self.robot_position = new_pos;
        }
    }

    pub fn sum_of_gps_coordinates(&self) -> usize {
        let mut sum = 0;
        for (row, line) in self.map.iter().enumerate() {
            for (col, object) in line.iter().enumerate() {
                if let Space::BoxHead = object {
                    sum += 100 * row + col;
                }
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    fn create_warehouse(input: &str) -> Warehouse2 {
        let input = String::from_str(input).unwrap();
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
        let input = "####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################";
        let warehouse = create_warehouse(input);

        assert_eq!(9021, warehouse.sum_of_gps_coordinates())
    }
}
