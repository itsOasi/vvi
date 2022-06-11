use super::particles::{Body, Shape};
use super::base::{DictEntry};
pub struct World{
    archive: Vec<Body>, // bodies available for instantiation
    bodies: Vec<DictEntry<Shape>>, // active bodies in the system
    shapes: Vec<DictEntry<Shape>>, // shapes available for use in the system
}

// functions dealing with bodies and shapes
impl World{
    pub fn add_body(&mut self){}
    pub fn arc_body(&mut self){}
    pub fn load_body(&mut self){}
    
    // 
    pub fn add_shape(&mut self){}
    pub fn arc_shape(&mut self){}
    pub fn load_shape(&mut self){}
}