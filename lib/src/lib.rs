use eframe::egui;
use state::State;

mod camera;
mod color;
mod hitable;
mod interval;
mod ray;
mod renderer;
mod state;

pub type Vec3 = nalgebra_glm::TVec3<f64>;

static mut STATE: Option<Box<State>> = None;
fn state(storage: &mut dyn eframe::Storage) -> &'static mut State {
    unsafe {
        match &mut STATE {
            Some(state) => state,
            None => {
                STATE = Some(Box::new(State::new(storage)));
                STATE.as_mut().unwrap()
            }
        }
    }
}

#[no_mangle]
pub fn update(ctx: &egui::Context, storage: &mut dyn eframe::Storage) {
    state(storage).update(ctx);
}

#[no_mangle]
pub fn reload_lib(storage: &mut dyn eframe::Storage) {
    state(storage).save(storage);
}
