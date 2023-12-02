use crate::{camera::Camera, color::*, hitable::HittableList, interval::Interval, ray::Ray, Vec3};

pub fn render(camera: &Camera, world: &HittableList) -> Vec<(u8, u8, u8)> {
    let per_pixel = |x: u32, y: u32| {
        let pixel = camera.pixel_center(x, y);
        let ray = Ray {
            origin: camera.origin,
            dir: pixel - camera.origin,
        };

        let color = ray_color(&ray, world);
        (
            (255. * color.x) as u8,
            (255. * color.y) as u8,
            (255. * color.z) as u8,
        )
    };

    (0..camera.height)
        .map(|y| (0..camera.width).map(move |x| per_pixel(x, y)))
        .rev()
        .flatten()
        .collect()
}

pub fn ray_color(ray: &Ray, world: &HittableList) -> Vec3 {
    if let Some(hit) = world.hit(ray, Interval::CAMERA) {
        return 0.5 * (hit.normal + Vec3::new(1., 1., 1.));
    }

    let unit_direction = ray.dir.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    mix(WHITE, SKY_BLUE, t)
}
