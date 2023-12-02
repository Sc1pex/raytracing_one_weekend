use crate::{color::*, hitable::HittableList, ray::Ray, Vec3};

pub fn render(w: usize, h: usize, world: &HittableList) -> Vec<(u8, u8, u8)> {
    let aspect_ratio = w as f64 / h as f64;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;

    let focal_length = 1.0;
    let camera_origin = Vec3::zeros();

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, viewport_height, 0.0);

    let pixel_du = viewport_u / (w as f64);
    let pixel_dv = viewport_v / (h as f64);

    let viewport_upper_left_corner =
        camera_origin - viewport_u / 2.0 - viewport_v / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    let pixel_00 = viewport_upper_left_corner + pixel_du / 2.0 + pixel_dv / 2.0;

    let per_pixel = |x: usize, y: usize| {
        let pixel = pixel_00 + pixel_du * (x as f64) + pixel_dv * (y as f64);
        let ray = Ray {
            origin: camera_origin,
            dir: pixel - camera_origin,
        };

        let color = ray_color(&ray, world);
        (
            (255. * color.x) as u8,
            (255. * color.y) as u8,
            (255. * color.z) as u8,
        )
    };

    (0..h)
        .map(|y| (0..w).map(move |x| per_pixel(x, y)))
        .rev()
        .flatten()
        .collect()
}

pub fn ray_color(ray: &Ray, world: &HittableList) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.0, std::f64::MAX) {
        return 0.5 * (hit.normal + Vec3::new(1., 1., 1.));
    }

    let unit_direction = ray.dir.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    mix(WHITE, SKY_BLUE, t)
}
