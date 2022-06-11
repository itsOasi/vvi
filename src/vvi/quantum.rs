mod particles;
pub mod Physics{
    use super::particles;
    use crate::vvi::common::{Triple, self};

    pub struct World{
        bodies: Vec<particles::Body>,
        shapes: Vec<particles::Shape>,
    }
    impl common::SimRunTime for World{
        fn load(&mut self) {
            
        }
        fn step(&mut self) {
            
        }
        fn freeze(&mut self) {
            
        }
        fn report(&self) {
            
        }
    }
    // TODO: impl SimRunTime for World
    // design how the types will interact
    pub fn cast_radius(from: Particles::Point, dir: Triple) -> Triple{
        // cast ray from particle to radius in given direction and returns a Position
        let res = Triple::zero();
        res
    }
    pub fn cast_ray(from: Particles::Point, dist: f32,  dir: Option<Triple> ) -> Option<Particles::Body>{
        // casts a ray into the scene and returns the first body hit
        let res: Option<Particles::Body> = None;
        res
    }
}
