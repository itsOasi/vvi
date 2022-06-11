use crate::vvi::common::{Triple, DictEntry};
use crate::vvi::common::particles::{Body, Shape, Point};
pub struct Shader{
    // define shader classes
    classes: Vec<DictEntry<String>>
}
pub mod Comp{
    pub struct Camera{} // the most important part of the engine
    pub struct Shader{}
    pub struct TexturePack{}
}
struct Cloud{
    // buffer: ImageData, // an image ready to be displayed to canvas
    cameras: Vec<DictEntry<Comp::Camera>>,
    shaders: Vec<DictEntry<Comp::Shader>>,
    textures: Vec<DictEntry<Comp::TexturePack>>, 
    active_cam: Option<String>,
    bodies: Vec<Body>,

}
// TODO: impl SimRunTime for cloud
impl Cloud{

    // configuration 
    pub fn config_scene(){
        // Config 1: get static snapshot of scene and get active cam
            // get all Bodies in the scene from Particles
            // scans bodies to find active camera
            // assign z-depth buffer to shapes based on distance from camera to closest point
    }
    pub fn gen_voxels(){
        // config 2: generate voxel mesh
        // generate mesh based on point radii and resolution
            // generate all values within and including the surface according to resolution
            // calculate voxels to be shown
            /*
                point: Point
                v: f8 // number of voxels
                m: f8 // number of meters
                res: f8 = v/m^3 // resolution
                rad: f8 = point.radius
                // ex. a minecraft block is 1 voxel per m^3
                //find surface area to set maximum number of voxels to be found
                surface_area: u32 = 4*3.14*(rad*rad);
                
                // start at the given point and find the surface point
                surface_point: cast_ray(point: Point, dir: Triple)
                
                // then find
                approx_voxel: Triple
            */
    }

    pub fn load_tex_pack(){
        /*
            TexturePack

            TexturePacks provide a concise and predictable pipeline
            for the camera to follow when compositing the image.

            The texturepack metadata provides foresight on if and how 
            the object will be rendered. 
            
            Without a TexturPack, the camera will just render the default 
            point colors and shaders

            TexturePacks are applied on a per shape basis. They can include UV wrap, height map and, albedo info.
        */
        
    }

    pub fn load_shaders(){
        // 
    }

    // optimization
    pub fn load_srz(){
        // Pass 1: selective raycast zoning
        /*
            One of the main tradeoffs of 3d rendering is showing as much necessary detail as possible 
            using energy as efficiently as possible
            
            A ray represents a bit of light that reaches the light sensor
            In the real world, more light captured means more detail in the image
            
            With the srz algorithm, rays are cast out in a texture defined zoning pattern that 
            prioritizes where the camera's rays should focus
            Areas that are more likely to be in focus receive more rays while peripheral areas receive 
            fewer rays, therefore reserving energy for where the main focus of the
            scene is

            in the areas that receive fewer rays, the area of effect (stroke_size) for each color hit is 
            calculated to fill in the blanks where light information is missing, much like how the brain does
            
            inspired by how different pupil shapes recieve light differently to enhance focus 
            on areas that matter the most and the brain fills in the rest
        */
        // read image 
            // grayscale, 0..1 float values
        // map image to context size 
            // stretch image to fit screen ex. 1920*1080 = 2,073,060
            // maybe apply smoothing
        // get number of light passes for scene 
            // ex. 8 passes * 2.07 m pixels = 16.6 m ray traces
        // calculate number of ray traces per zone 
            // total_rays: f32; // total number of rays scheduled to be sent
            // zone_weight: f32; // the average weight of all values in zone
            // zone_rays: u32; // the total number of rays allowed in this zone
            // proportion: f32 = (1-zone_weight) // inverts normalized value to get proportion
            // zone_rays = proportion/total_rays // amount of total rays the zone will get
        // calculate stroke sizes per zone
            // stroke_size = 
        // compile into array of ray traces per pixel and their weights

    }
    
    pub fn trace(){

    }

    // image generation
    pub fn project_mesh(){
        // pass 1: project mesh to canvas
            // get in camera perspective
                // transform meshes to camera space
                // store only the values that the rays collide with unless there are points closer
    }


}
