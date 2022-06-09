pub mod Particles{
    use crate::vvi::common::{DictEntry, Triple, Base};
    // name the types involved
    pub struct Point{
        /*
            A single point in space, voxel independent.

            Points themselves do not have shapes, but they do have a radius.
        */
        origin: Base, 
        // position goes through transformations through global to camera space for rendering
        radius: f32, // distance in meters from position at which the mesh will generate
        color: Triple, // base color of generated mesh
    }
    pub enum Primitive{
        Box(Triple), // scale x, y, z
    }
    pub struct Shape{
        /*
            Points combine to make shapes. When placed inside of a shape, the radii of each individual 
            point will determin where each voxel that makes of the shape will be placed.

        */
        origin: Base,
        points: Vec<Point>,
        class: Vec<String>, // cascading shader classes
        comps: Vec<DictEntry<String>>, // things like textures, shaders, and Transform functionality
        props: Vec<DictEntry<String>>, // custom values
        is_trigger: bool, // collision will emit signal
        is_dynamic: bool, // object can move
        is_solid: bool, // constrain objects from passing through collision boundaries
    }
    impl Shape {
        pub fn from_primitive(shape: Primitive){
            
        }

        fn calculate_box(){}
        fn calculate_pyr(){}
        fn calculate_cyl(){}
    }
    pub struct Body{
        /*
            Bodies represent a collection of shapes meant to be seen as a whole
            Shapes that share the same body will have their meshes interpolated seamlessly together
            Bodies can also have parent-child relationships
        */
        origin: Base,
        shapes: Vec<String>,
        children: Vec<DictEntry<String>>,
    }
}
pub mod Physics{
    use crate::vvi::common::Triple;
    struct World{
        
    }
    // TODO: impl SimRunTime for World
    // design how the types will interact
    use super::Particles;
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
    pub fn resolve_constraints(){
        // makes sure all points are in non constrained areas and report collisions
    }
}
