use crate::{interval::Interval, ray::Ray, Vec3};
use serde::{Deserialize, Serialize};

mod sphere;

pub use sphere::Sphere;

#[derive(Serialize, Deserialize)]
pub enum Hittable {
    Sphere(sphere::Sphere),
}

impl Hittable {
    pub fn hit(&self, ray: &Ray, range: Interval) -> Option<Hit> {
        match self {
            Hittable::Sphere(s) => s.hit(ray, range),
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

    pub fn hit(&self, ray: &Ray, mut range: Interval) -> Option<Hit> {
        let mut best_hit = None;

        for obj in &self.list {
            if let Some(hit) = obj.hit(ray, range) {
                range.end = hit.t;
                best_hit = Some(hit);
            }
        }

        best_hit
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
