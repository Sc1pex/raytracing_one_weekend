use super::*;

#[derive(Serialize, Deserialize)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn hit(&self, ray: &Ray, range: Interval) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.dir.dot(&ray.dir);
        let b = 2. * oc.dot(&ray.dir);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let root = (-b - sqrtd) / (2. * a);
        if !range.surrounds(root) {
            let root = (-b + sqrtd) / (2. * a);
            if !range.surrounds(root) {
                return None;
            }
        }

        let normal = (ray.at(root) - self.center) / self.radius;
        Some(Hit::new(ray, root, normal))
    }
}
