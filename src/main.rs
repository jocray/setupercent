#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod battery;
mod config;
mod tray;

use app::SetupercentApp;
use battery::{DeviceInfo, fetch_connected_devices};
use config::AppConfig;
use eframe::egui;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() -> eframe::Result<()> {
    let config = Arc::new(Mutex::new(AppConfig::load()));
    let initial_devices = fetch_connected_devices();
    let devices: Arc<Mutex<Vec<DeviceInfo>>> = Arc::new(Mutex::new(initial_devices));

    let devices_bg = Arc::clone(&devices);
    let config_bg = Arc::clone(&config);
    thread::spawn(move || {
        let mut last_fetch = Instant::now();
        loop {
            thread::sleep(Duration::from_secs(1));
            let interval_secs = {
                let cfg = config_bg.lock().unwrap();
                cfg.poll_interval_minutes as u64 * 60
            };
            if last_fetch.elapsed() >= Duration::from_secs(interval_secs) {
                let updated = fetch_connected_devices();
                if let Ok(mut lock) = devices_bg.lock() {
                    *lock = updated;
                }
                last_fetch = Instant::now();
            }
        }
    });

    // v1.1.0: окно расширено под боковое меню (150 px), контент и нижний футер.
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([560.0, 380.0])
        .with_min_inner_size([560.0, 380.0])
        .with_resizable(false);

    if let Ok(img) =
        image::load_from_memory(include_bytes!("../assets/images/setupercent_icon.png"))
    {
        let rgba_img = img.to_rgba8();
        let (width, height) = rgba_img.dimensions();
        viewport = viewport.with_icon(egui::IconData {
            rgba: rgba_img.into_raw(),
            width,
            height,
        });
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    let config_app = Arc::clone(&config);
    eframe::run_native(
        "setupercent",
        options,
        Box::new(move |cc| Ok(Box::new(SetupercentApp::new(cc, devices, config_app)))),
    )
}
