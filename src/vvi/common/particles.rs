use super::base::{Base, Triple};
use crate::vvi::common::{gen_id};

// name the types involved
pub struct Point{
    /*
        A single point in space, voxel independent.

        Points themselves do not have shapes, but they do have a radius.
    */
    origin: Base, 
    position: Triple, // position in local space after transformations
    // position goes through transformations through global to camera space for rendering
    radius: f32, // distance in meters from position at which the mesh will generate
    color: Triple, // base color of generated mesh
}

impl Point{
    pub fn new(position: Triple, color: Triple)->Self{
    let id: String;
        Self { 
            origin: Base::new(String::from("PNT"), position.copy()),
            position, radius: 1.0, color
        }
    }
}

pub struct Shape{
    /*
        Points combine to make shapes. When placed inside of a shape, the radii of each individual 
        point will determin where each voxel that makes up the shape will be placed.

    */
    origin: Base,        
    position: Triple, // position in local space after transformations
    points: Vec<Point>, // points that make up the shape
    is_trigger: bool, // collision will emit signal
    is_dynamic: bool, // object can move
    is_solid: bool, // constrain objects from passing through collision boundaries
}


impl Shape {
    pub fn new(position: Triple) -> Self{
        Self { 
            origin: Base::new(gen_id(String::from("SHP")), position), 
            points: Vec::new(),
            position: Triple::zero(), 
            is_trigger: false, is_dynamic: true, is_solid: true 
        }
    }
    pub fn add_points(&mut self, point: Point){
        self.points.push(point);
    }
}

pub struct Body{
    /*
        Bodies represent a collection of shapes meant to be seen as a whole
        Shapes that share the same body will have their meshes interpolated seamlessly together
    */
    origin: Base,
    name: String,
    position: Triple, // position in local space after transformations
    shapes: Vec<String>, // points to name in shape library
}