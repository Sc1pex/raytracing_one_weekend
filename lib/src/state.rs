use crate::{camera::Camera, hitable::*, renderer::render, Vec3};
use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct State {
    world: HittableList,
    camera: Camera,

    #[serde(skip)]
    texture_handle: Option<egui::TextureHandle>,
}

impl State {
    pub fn new(storage: &mut dyn eframe::Storage) -> Self {
        let mut s: Self = eframe::get_value(storage, "dyn_lib_state").unwrap_or_default();

        s.world = HittableList::default();
        s.world.add(Hittable::Sphere(Sphere {
            center: Vec3::new(0., 0., -1.),
            radius: 0.5,
        }));
        s.world.add(Hittable::Sphere(Sphere {
            center: Vec3::new(0., -100.5, -1.),
            radius: 100.,
        }));

        s
    }

    pub fn save(&self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "dyn_lib_state", self);
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let size = ui.min_rect();
            let w = size.width() as usize;
            let h = size.height() as usize;
            self.camera.resize(w as u32, h as u32);
            let pixels = render(&self.camera, &self.world)
                .into_iter()
                .flat_map(|(r, g, b)| vec![r, g, b])
                .collect::<Vec<u8>>();
            let color_image = egui::ColorImage::from_rgb([w, h], &pixels);

            match &mut self.texture_handle {
                Some(handle) => handle.set(color_image, Default::default()),
                None => {
                    let handle = ctx.load_texture("texture", color_image, Default::default());
                    self.texture_handle = Some(handle);
                }
            }

            ui.image(self.texture_handle.as_ref().unwrap());
        });

        ctx.request_repaint();
    }
}
