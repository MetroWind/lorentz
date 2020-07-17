use std::sync::Arc;

use crate::camera::Camera;
use crate::primitive::{PrimitiveList};
use crate::material;

pub struct Scene
{
    pub width: u32,
    pub height: u32,
    pub camera: Camera,
    pub primitives: PrimitiveList,
    pub materials: Vec<Arc<dyn material::Material + Send + Sync>>,
}
