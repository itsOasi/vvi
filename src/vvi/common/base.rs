pub struct DictEntry<T>{
    key: String,
    value: T
}
impl<T> DictEntry<T>{
    fn new(key:String, value: T)->Self{
        Self{key, value}
    }
}

pub struct Triple{
    x: f32,
    y: f32,
    z: f32,
}

// functions to do math with triples
impl Triple{
    pub fn add(&mut self, b:Triple){self.x += b.x;self.y += b.y;self.z += b.z;}
    pub fn sub(&mut self, b:Triple){self.x -= b.x;self.y -= b.y;self.z -= b.z;}
    pub fn mult(&mut self, b:Triple){self.x *= b.x;self.y *= b.y;self.z *= b.z;}
    pub fn div(&mut self, b:Triple){self.x /= b.x;self.y /= b.y;self.z /= b.z;}
}

// functions to generate Triples
impl Triple {
    pub fn new(x: f32, y: f32, z: f32)->Self{
        Self{x, y, z}
    }
    pub fn zero()->Self{
        Self{x: 0.0, y: 0.0, z: 0.0}
    }
    pub fn left(amnt: f32)->Self{Self{x: -amnt, y: 0.0, z:0.0}}
    pub fn right(amnt: f32)->Self{Self{x: amnt, y: 0.0, z:0.0}}
    pub fn up(amnt: f32)->Self{Self{y: -amnt, x: 0.0, z:0.0}}
    pub fn down(amnt: f32)->Self{Self{y: amnt, x: 0.0, z:0.0}}
    pub fn front(amnt: f32)->Self{Self{z: -amnt, x: 0.0, y:0.0}}
    pub fn back(amnt: f32)->Self{Self{z: amnt, x: 0.0, y:0.0}}
    
    pub fn copy(&self)->Self{Self{z: self.z, x: self.x, y:self.y}}
}

pub struct Base{
    /*
        holds information and metadata commonly needed for all objects in system 
    */
    id: String, // for use within the model
    origin: Triple, // object position before transformations
    classes: Vec<DictEntry<String>>, // any user defined shader classes
    comps: Vec<DictEntry<String>>, // any user defined components
}

impl Base {
    pub fn new(id: String, origin: Triple) -> Self{
        Self { id, origin, classes: Vec::new(), comps: Vec::new() }
    }

    pub fn get_id(&self){}
    pub fn add_class(&mut self){}
    pub fn add_comp(&mut self){}
}