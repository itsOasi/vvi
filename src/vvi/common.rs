use std::vec::Vec;


pub struct DictEntry<T>{
    key: String,
    value: T
}
impl<T> DictEntry<T>{
    fn new(key:String, value: T)->Self{
        Self{key, value}
    }
}

pub struct Base{
    /*
        a base holds initial information and metadata about objects
    */
    id: String, // for use within the model
    origin: Triple, // origin in global space prior to transformations
    position: Triple, // position in local space after transformations

}

impl Base {
    pub fn new(id: String, origin: Triple, position: Triple) -> Self{
        Self{id, origin, position}
    }
}

pub struct Triple{
    x: f32,
    y: f32,
    z: f32,
}

impl Triple {
    pub fn new(x: f32, y: f32, z: f32)->Self{
        Self{x, y, z}
    }
    pub fn zero()->Self{
        Self{x: 0.0, y: 0.0, z: 0.0}
    }
}

pub trait SimRunTime {
        fn load(&mut self); // initializes simulation with data
        fn step(&mut self); // advances simulation by one step
        fn freeze(&mut self); // returns raw data in its current state
        fn report(&self); // provides an overview of the simulation in human readable text
}
