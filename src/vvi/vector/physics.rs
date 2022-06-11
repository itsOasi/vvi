use std::process::Output;

use crate::vvi::common::{self};
pub enum Primitives{
    Square(f32, f32), // scale x, y
}

impl common::SimRunTime for common::world::World {
    fn load(&mut self, data: String) {
        
    }

    fn step(&mut self, time: f32) {
        
    }
    fn report(&self) {
        
    }
    fn freeze(&self, kwargs: Vec<common::DictEntry<String>>)->String{
        let output = String::new();
        output
    }
}

fn handle_collisions(){}