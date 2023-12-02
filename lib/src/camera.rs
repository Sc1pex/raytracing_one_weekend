use crate::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Camera {
    pub width: u32,
    pub height: u32,

    pub pixel00: Vec3,
    pub origin: Vec3,
    pub d_u: Vec3,
    pub d_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,

            pixel00: Vec3::zeros(),
            origin: Vec3::zeros(),
            d_u: Vec3::zeros(),
            d_v: Vec3::zeros(),
        }
    }
}

impl Camera {
    pub fn new(image_width: u32, image_height: u32) -> Self {
        Self {
            width: image_width,
            height: image_height,
            ..Default::default()
        }
    }

    pub fn resize(&mut self, image_width: u32, image_height: u32) {
        if self.width == image_width && self.height == image_height {
            return;
        }

        self.width = image_width;
        self.height = image_height;
        self.initialize();
    }

    pub fn pixel_center(&self, x: u32, y: u32) -> Vec3 {
        self.pixel00 + self.d_u * (x as f64) + self.d_v * (y as f64) - self.origin
    }
}

impl Camera {
    pub fn initialize(&mut self) {
        let aspect_ratio = self.width as f64 / self.height as f64;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., viewport_height, 0.);

        self.d_u = viewport_u / (self.width - 1) as f64;
        self.d_v = viewport_v / (self.height - 1) as f64;

        let focal_length = 1.0;

        self.origin = Vec3::zeros();
        let viewport_up_left =
            self.origin - Vec3::new(0., 0., focal_length) - viewport_v / 2. - viewport_u / 2.;
        self.pixel00 = viewport_up_left + self.d_u / 2. + self.d_v / 2.;
    }
}
