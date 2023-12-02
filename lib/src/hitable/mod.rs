use crate::{ray::Ray, Vec3};
use serde::{Deserialize, Serialize};

mod sphere;

pub use sphere::Sphere;

#[derive(Serialize, Deserialize)]
pub enum Hittable {
    Sphere(sphere::Sphere),
}

impl Hittable {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        match self {
            Hittable::Sphere(s) => s.hit(ray, t_min, t_max),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct HittableList {
    pub list: Vec<Hittable>,
}

impl HittableList {
    pub fn add(&mut self, hitable: Hittable) {
        self.list.push(hitable);
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut closest_so_far = t_max;
        let mut hit = None;

        for h in &self.list {
            if let Some(h) = h.hit(ray, t_min, closest_so_far) {
                closest_so_far = h.t;
                hit = Some(h);
            }
        }

        hit
    }
}

pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl Hit {
    pub fn new(ray: &Ray, t: f64, normal: Vec3) -> Self {
        let front_face = ray.dir.dot(&normal) < 0.;
        let normal = if front_face { normal } else { -normal };

        Self {
            point: ray.at(t),
            normal,
            t,
            front_face,
        }
    }
}
