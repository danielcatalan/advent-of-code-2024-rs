use std::collections::HashSet;

use crate::garden::{Garden, Plant};

pub struct TotalRegions<'a> {
    regions: Vec<Region>,
    garden: &'a Garden,
}

impl<'a> TotalRegions<'a> {
    pub fn new(garden: &'a Garden) -> Self {
        TotalRegions {
            regions: vec![],
            garden: garden,
        }
    }

    pub fn populate_regions(&mut self, mut pool: HashSet<Plant>) {
        while pool.len() > 0 {
            let plant = pool.iter().next().unwrap().clone();
            let plant = pool.take(&plant).unwrap();
            let mut region = Region::new(plant.typ);
            region.populate(plant, &mut pool, &self.garden);
            self.regions.push(region);
        }
    }

    pub fn get_pricing(&self) -> usize {
        let mut price = 0;
        for region in &self.regions {
            let area = region.area();
            let parimeter = region.parimeter(self.garden);
            price += area * parimeter;
        }
        price
    }

    pub fn get_pricing2(&self) -> usize {
        let mut price = 0;
        for region in &self.regions {
            let area = region.area();
            let corner_count = region.corner_count(self.garden);
            price += area * corner_count;
        }
        price
    }
}

pub struct Region {
    typ: u8,
    plants: Vec<Plant>,
}

impl Region {
    fn new(typ: u8) -> Self {
        Region {
            typ: typ,
            plants: vec![],
        }
    }

    fn populate(&mut self, plant: Plant, pool: &mut HashSet<Plant>, garden: &Garden) {
        if self.typ != plant.typ {
            panic!("Expected plant typ of {}", self.typ)
        }

        let neighbors = plant.get_neighbors_in_kind(garden);
        self.plants.push(plant);
        for neighbor in neighbors {
            if let Some(neighbor) = pool.take(&neighbor) {
                self.populate(neighbor, pool, garden);
            }
        }
    }

    fn area(&self) -> usize {
        self.plants.len()
    }

    fn parimeter(&self, garden: &Garden) -> usize {
        let mut fence_count = 0;
        for plant in &self.plants {
            fence_count += plant.get_fence_count(garden);
        }
        fence_count
    }

    fn corner_count(&self, garden: &Garden) -> usize {
        let mut corner_count = 0;
        for plant in &self.plants {
            corner_count += plant.corner_count(garden, &self);
        }
        corner_count
    }
    pub fn contains(&self, plant: &Plant) -> bool {
        self.plants.contains(plant)
    }
}
