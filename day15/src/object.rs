use crate::{map::Map, movement::Movement, space::Space};

pub enum Object<'a> {
    Robot(&'a mut Map, (usize, usize)),
    WideBox(&'a mut Map, (usize, usize)),
    Empty((usize, usize)),
    Wall,
}
impl Object<'_> {
    pub(crate) fn is_movable(&mut self, movement: &Movement) -> bool {
        match movement {
            Movement::Up | Movement::Down => self.is_movable_ud(movement),
            Movement::Left | Movement::Right => self.is_movable_lr(movement),
        }
    }

    fn is_movable_lr(&mut self, movement: &Movement) -> bool {
        match self {
            Object::Empty(..) => true,
            Object::Wall => false,
            Object::Robot(map, position) => {
                let next_pos = movement.next_pos(&position);
                let mut object = map.get_object(&next_pos);
                object.is_movable(movement)
            }
            Object::WideBox(map, position) => {
                let mut next_pos = movement.next_pos(&position);
                if let Movement::Right = movement {
                    next_pos.1 += 1; // compensate for right of widebox
                }
                let mut object = map.get_object(&next_pos);
                object.is_movable(movement)
            }
        }
    }
    fn is_movable_ud(&mut self, movement: &Movement) -> bool {
        match self {
            Object::Empty(..) => true,
            Object::Wall => false,
            Object::Robot(map, position) => {
                let next_pos = movement.next_pos(&position);
                let mut object = map.get_object(&next_pos);
                object.is_movable(movement)
            }
            Object::WideBox(map, position) => {
                let next_pos1 = movement.next_pos(&position);
                if let Space::BoxHead = map.get_space(&next_pos1) {
                    // next object is a wide-box directly above/below
                    let mut object = map.get_object(&next_pos1);
                    return object.is_movable(movement);
                } else {
                    let next_pos2 = (next_pos1.0, next_pos1.1 + 1);
                    let movable1 = map.get_object(&next_pos1).is_movable(movement);
                    let movable2 = map.get_object(&next_pos2).is_movable(movement);
                    movable1 && movable2
                }
            }
        }
    }

    pub(crate) fn do_move(&mut self, movement: &Movement) -> (usize, usize) {
        match movement {
            Movement::Up | Movement::Down => self.do_move_ud(movement),
            Movement::Left | Movement::Right => self.do_move_lr(movement),
        }
    }

    fn do_move_lr(&mut self, movement: &Movement) -> (usize, usize) {
        match self {
            Object::Robot(map, curr_position) => {
                let next_position = movement.next_pos(&curr_position);
                let mut object = map.get_object(&next_position);
                let _ = object.do_move_lr(movement);
                map.swap(*curr_position, next_position);
                return next_position;
            }
            Object::WideBox(map, curr_position) => {
                todo!()
            }
            Object::Empty(position) => *position,
            Object::Wall => todo!(),
        }
    }

    fn do_move_ud(&mut self, movement: &Movement) -> (usize, usize) {
        todo!()
    }
}
