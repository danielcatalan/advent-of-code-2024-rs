use crate::space::Space;

pub struct Warehouse{
    robot_position: (usize,usize),
    warehouse_space:Vec<Vec<Space>>
}

impl Warehouse{
    pub fn new(robot_position: (usize,usize), warehouse_space: Vec<Vec<Space>>) -> Self{
        Warehouse{robot_position,warehouse_space}
    }
}