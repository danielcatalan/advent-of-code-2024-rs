use crate::{garden::Plant, region::Region};

pub struct CornerHelper {
    value: u8,
}

impl CornerHelper {
    pub fn new() -> Self {
        CornerHelper { value: 0 }
    }

    pub fn push(&mut self, plant: Option<Plant>, region: &Region) {
        let bit: u8 = if let Some(plant) = plant {
            if region.contains(&plant) {
                1
            } else {
                0
            }
        } else {
            0
        };

        self.value = ((self.value << 1) | bit) & 0b111;
    }

    pub fn is_corner(&self) -> bool {
        match self.value {
            0b000 | 0b101 | 0b010 => true,
            _ => false,
        }
    }
}
