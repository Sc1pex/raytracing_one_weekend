use eframe::{CreationContext, NativeOptions};
use std::sync::mpsc;

#[hot_lib_reloader::hot_module(dylib = "lib")]
mod hot_lib {
    hot_functions_from_file!("lib/src/lib.rs");

    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}

    use eframe::egui;
}

struct MainApp {
    reload_rx: mpsc::Receiver<()>,
    save_state_tx: mpsc::Sender<()>,
}
impl MainApp {
    pub fn new(
        _cc: &CreationContext,
        reload_rx: mpsc::Receiver<()>,
        save_state_tx: mpsc::Sender<()>,
    ) -> Self {
        Self {
            reload_rx,
            save_state_tx,
        }
    }
}
impl eframe::App for MainApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        let storage = frame
            .storage_mut()
            .expect("eframe::persitance feature should be enabled");

        if self.reload_rx.try_recv().is_ok() {
            hot_lib::reload_lib(storage);
            self.save_state_tx.send(()).expect("Channel closed");

            // Wait for lib to be reloaded
            self.reload_rx.recv().expect("Channel closed");
        }

        hot_lib::update(ctx, storage);
    }
}

fn main() {
    let (reload_tx, reload_rx) = mpsc::channel::<()>();
    let (save_state_tx, save_state_rx) = std::sync::mpsc::channel::<()>();

    std::thread::spawn(move || loop {
        let token = hot_lib::subscribe().wait_for_about_to_reload();

        // Tell app that a reload will happen
        // And wait for it to save the state
        reload_tx.send(()).expect("Channel closed");
        save_state_rx.recv().expect("Channel closed");
        drop(token);

        hot_lib::subscribe().wait_for_reload();
        // Tell app that lib has reloaded
        reload_tx.send(()).expect("Channel closed");
    });

    let native_options = NativeOptions::default();
    eframe::run_native(
        "Hot reloadable",
        native_options,
        Box::new(|cc| Box::new(MainApp::new(cc, reload_rx, save_state_tx))),
    )
    .expect("Failed to run app");
}
