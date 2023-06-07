use crate::{object::Hit, Camera, Vec3};

pub struct Renderer {
    pub width: f32,
    pub height: f32,
    pub samples_per_pixel: usize,
    pub ray_depth: usize,
    pub camera: Camera,
    pub world: Hit,
}

impl Renderer {
    pub fn render_pixel(&self, x: f32, y: f32) -> Vec3 {
        let u = (x + fastrand::f32()) / (self.width - 1.0);
        let v = (y + fastrand::f32()) / (self.height - 1.0);
        let r = self.camera.get_ray(u, v);
        r.trace(&self.world, self.ray_depth)
    }
}
