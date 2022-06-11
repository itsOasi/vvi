use self::common::{SimRunTime, DictEntry};

/* 
    Vector Voxel Imaging Library
*/
mod common; // basic functions and types
mod vector; // 2d physics engine
// mod quantum; // 3d physics engine
mod nebulizer; // ooooh, pretty voxels, always 3d equiv
mod rasterizer; // functions to convert nebulizer imagery to bitmap

use common::world::World;

pub struct VectorPhysics{
    world: common::world::World,
    debug: bool,
}

impl VectorPhysics {
    pub fn load(&mut self, file: String){
        // make sure file exists
        // read as string
        self.world.load(file);
    }
    pub fn run(&mut self){
        loop {
            self.world.step(0.016);
           if self.debug { self.world.report() } else {};
        }
    }
    pub fn freeze(&self, kwargs: Option<Vec<DictEntry<String>>>)->String{
        self.world.freeze(kwargs.unwrap_or(Vec::new()))
    }
}

// pub struct QuantumPhysics{
//     world: quantum::Physics::World,
// }
struct NebulizerCloud{

}