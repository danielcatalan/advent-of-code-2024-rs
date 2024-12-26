use std::collections::{HashSet, LinkedList};

pub struct Garden {
    map: Vec<Vec<u8>>,
}

impl Garden {
    pub fn new() -> Self {
        Garden { map: Vec::new() }
    }

    pub fn add_row(&mut self, row: Vec<u8>) {
        self.map.push(row);
    }

    pub fn row_len(&self) -> usize {
        self.map.len()
    }

    pub fn col_len(&self) -> usize {
        self.map[0].len()
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Plant> {
        if let Some(line) = self.map.get(row) {
            if let Some(typ) = line.get(col) {
                return Some(Plant {
                    typ: *typ,
                    position: (row, col),
                });
            }
        }
        None
    }

    pub fn generate_plant_pool(&self) -> HashSet<Plant> {
        let x = self
            .map
            .iter()
            .enumerate() //get row
            .flat_map(|(r, line)| {
                line.iter()
                    .enumerate() // get col
                    .map(move |(c, item)| Plant {
                        typ: *item,
                        position: (r, c),
                    })
            })
            .collect();
        x
    }
}

#[derive(Clone)]
pub struct Plant {
    pub typ: u8,
    pub position: (usize, usize),
}

impl Plant {
    pub(crate) fn get_neighbors(&self, garden: &Garden) -> Vec<Plant> {
        let mut neighbors = vec![];
        let (row, col) = self.position;
        // get above
        if row != 0 {
            if let Some(neighbor) = garden.get(row - 1, col) {
                neighbors.push(neighbor)
            }
        }
        // get left
        if col != 0 {
            if let Some(neighbor) = garden.get(row, col - 1) {
                neighbors.push(neighbor)
            }
        }
        // get below
        if let Some(neighbor) = garden.get(row + 1, col) {
            neighbors.push(neighbor)
        }
        // get right
        if let Some(neighbor) = garden.get(row, col + 1) {
            neighbors.push(neighbor)
        }
        neighbors
    }
    pub(crate) fn get_neighbors_in_kind(&self, garden: &Garden) -> Vec<Plant> {
        let binding = self.get_neighbors(garden);
        let neighbors = binding
            .iter()
            .filter(|plant| plant.typ == self.typ)
            .map(|plant| plant.clone());
        neighbors.collect()
    }

    pub fn get_fence_count(&self, garden: &Garden) -> usize {
        let neighbors = self.get_neighbors_in_kind(garden);
        4 - neighbors.len()
    }
}

impl std::hash::Hash for Plant {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.typ.hash(state);
        self.position.hash(state);
    }
}

impl PartialEq for Plant {
    fn eq(&self, other: &Self) -> bool {
        self.typ == other.typ && self.position == other.position
    }
}
impl Eq for Plant {}
