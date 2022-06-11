use std::vec::Vec;
pub mod base;
pub mod world;
pub mod particles;
pub use base::{Triple, DictEntry, Base};
pub use world::World;

use self::particles::Body;

pub trait SimRunTime {
    fn load(&mut self, data: String); // initializes simulation with data
    fn step(&mut self, time: f32); // advances simulation by one step
    fn freeze(&self, kwags: Vec<DictEntry<String>>) -> String; // returns raw data in its current state
    fn report(&self); // provides an overview of the simulation in human readable text
}

pub trait Dynamics {
    fn translate(body: Body){ // moves body irrespective to simulation

    }
    
    fn force(body: Body){ // applies simulated force to body
    
    }
}

pub trait ColDetection{
    fn detect_collisions(){}
}

pub trait ColHandling{
    fn handle_collisions(){}
}

pub fn gen_id(prefix: String)->String{
    prefix
}