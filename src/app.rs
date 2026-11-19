use crate::battery::DeviceInfo;
use crate::config::{tr, AppConfig, Language};
use crate::tray::{TrayAction, TrayManager};
use eframe::egui::{
    self, Align, Color32, CornerRadius, FontId, Layout, Margin, RichText, Stroke, StrokeKind, Vec2,
};
use std::sync::{Arc, Mutex};

const COLOR_BG: Color32 = Color32::from_rgb(0x1b, 0x1b, 0x1b);
const COLOR_TEXT: Color32 = Color32::from_rgb(0xff, 0xff, 0xff);
const COLOR_ACCENT: Color32 = Color32::from_rgb(0x46, 0x6e, 0xce);
const COLOR_TEXT_MUTED: Color32 = Color32::from_rgb(0x88, 0x88, 0x88);
const COLOR_CARD_BG: Color32 = Color32::from_rgb(0x24, 0x24, 0x24);
const COLOR_CARD_BG_HOVER: Color32 = Color32::from_rgb(0x2c, 0x2c, 0x2c);
const COLOR_CARD_BG_ACTIVE: Color32 = Color32::from_rgb(0x28, 0x33, 0x4d);
const COLOR_DANGER: Color32 = Color32::from_rgb(0xeb, 0x57, 0x57);
const COLOR_OK: Color32 = Color32::from_rgb(0x6f, 0xcf, 0x97);
const COLOR_TRACK: Color32 = Color32::from_rgb(0x33, 0x33, 0x33);

pub struct SetupercentApp {
    devices: Arc<Mutex<Vec<DeviceInfo>>>,
    config: Arc<Mutex<AppConfig>>,
    tray: TrayManager,
    last_tooltip: Option<String>,
    last_lang: Language,
}

impl SetupercentApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        devices: Arc<Mutex<Vec<DeviceInfo>>>,
        config: Arc<Mutex<AppConfig>>,
    ) -> Self {
        Self::setup_custom_font(&cc.egui_ctx);
        Self::setup_visuals(&cc.egui_ctx);

        let lang = config.lock().unwrap().language;
        let tray = TrayManager::new(lang).expect("Не удалось инициализировать иконку в трее");

        Self {
            devices,
            config,
            tray,
            last_tooltip: None,
            last_lang: lang,
        }
    }

    fn setup_custom_font(ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "custom_font".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/fonts/JOCRAY-Regular.ttf")).into(),
        );
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "custom_font".to_owned());
        ctx.set_fonts(fonts);
    }

    fn setup_visuals(ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();

        visuals.panel_fill = COLOR_BG;
        visuals.window_fill = COLOR_BG;
        visuals.extreme_bg_color = Color32::from_rgb(0x15, 0x15, 0x15);
        visuals.faint_bg_color = Color32::from_rgb(0x1f, 0x1f, 0x1f);
        visuals.override_text_color = Some(COLOR_TEXT);
        visuals.selection.bg_fill = COLOR_ACCENT;
        visuals.selection.stroke = Stroke::new(1.0, COLOR_TEXT);
        visuals.hyperlink_color = COLOR_ACCENT;

        visuals.window_corner_radius = CornerRadius::same(10);

        let wr = CornerRadius::same(6);
        visuals.widgets.inactive.bg_fill = COLOR_TRACK;
        visuals.widgets.inactive.weak_bg_fill = COLOR_TRACK;
        visuals.widgets.inactive.corner_radius = wr;
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, COLOR_TEXT);

        visuals.widgets.hovered.bg_fill = Color32::from_rgb(0x40, 0x40, 0x40);
        visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(0x40, 0x40, 0x40);
        visuals.widgets.hovered.corner_radius = wr;
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, COLOR_ACCENT);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, COLOR_TEXT);

        visuals.widgets.active.bg_fill = COLOR_ACCENT;
        visuals.widgets.active.weak_bg_fill = COLOR_ACCENT;
        visuals.widgets.active.corner_radius = wr;
        visuals.widgets.active.fg_stroke = Stroke::new(1.0, COLOR_TEXT);

        visuals.widgets.noninteractive.bg_fill = Color32::TRANSPARENT;
        visuals.widgets.noninteractive.weak_bg_fill = Color32::TRANSPARENT;
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, COLOR_TEXT);

        ctx.set_visuals(visuals);
    }

    fn sync_tray_icon(&mut self, current_devices: &[DeviceInfo]) {
        let config = self.config.lock().unwrap();
        let lang = config.language;
        let t = tr(lang);
        let active_device = current_devices
            .iter()
            .find(|d| config.enabled_device_ids.contains(&d.id));
        let current_percent = active_device.map(|d| d.battery_percent);
        let tooltip = match active_device {
            Some(dev) => {
                let name: &str = if dev.name.is_empty() {
                    t.no_device_name
                } else {
                    dev.name.as_str()
                };
                format!("setupercent — {}: {}%", name, dev.battery_percent)
            }
            None => t.no_selection_tooltip.to_string(),
        };
        drop(config);

        if self.last_tooltip.as_deref() != Some(tooltip.as_str()) {
            self.tray.update_display(current_percent, &tooltip);
            self.last_tooltip = Some(tooltip);
        }
    }

    fn draw_device_card(&mut self, ui: &mut egui::Ui, dev: &DeviceInfo) {
        let (mut is_enabled, lang) = {
            let cfg = self.config.lock().unwrap();
            (cfg.enabled_device_ids.contains(&dev.id), cfg.language)
        };
        let t = tr(lang);

        let row_height = 44.0;
        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(ui.available_width(), row_height),
            egui::Sense::click(),
        );

        let hovered = response.hovered();
        let bg = if is_enabled {
            COLOR_CARD_BG_ACTIVE
        } else if hovered {
            COLOR_CARD_BG_HOVER
        } else {
            COLOR_CARD_BG
        };

        let rounding = CornerRadius::same(8);
        ui.painter().rect_filled(rect, rounding, bg);

        if is_enabled {
            ui.painter().rect_stroke(
                rect,
                rounding,
                Stroke::new(1.0, COLOR_ACCENT),
                StrokeKind::Inside,
            );
        }

        let inner = rect.shrink2(Vec2::new(12.0, 0.0));
        let center_y = inner.center().y;

        let circle_center = egui::pos2(inner.left() + 7.0, center_y);
        if is_enabled {
            ui.painter().circle_filled(circle_center, 6.0, COLOR_ACCENT);
            ui.painter()
                .circle_filled(circle_center, 2.0, Color32::from_rgb(0xff, 0xff, 0xff));
        } else {
            ui.painter()
                .circle_stroke(circle_center, 6.0, Stroke::new(1.5, COLOR_TEXT_MUTED));
        }

        let display_name: &str = if dev.name.is_empty() {
            t.no_device_name
        } else {
            dev.name.as_str()
        };

        ui.painter().text(
            egui::pos2(inner.left() + 24.0, center_y),
            egui::Align2::LEFT_CENTER,
            display_name,
            FontId::proportional(13.0),
            COLOR_TEXT,
        );

        let pct_color = if dev.battery_percent <= 20 {
            COLOR_DANGER
        } else {
            COLOR_OK
        };

        let pct_text = format!("{}%", dev.battery_percent);
        let pct_galley =
            ui.painter()
                .layout_no_wrap(pct_text, FontId::proportional(13.0), pct_color);
        let pct_width = pct_galley.size().x;

        let bar_width = 44.0;
        let bar_height = 6.0;
        let bar_right = inner.right() - pct_width - 10.0;
        let bar_rect = egui::Rect::from_min_size(
            egui::pos2(bar_right - bar_width, center_y - bar_height / 2.0),
            Vec2::new(bar_width, bar_height),
        );

        ui.painter().rect_filled(
            bar_rect,
            CornerRadius::same(3),
            Color32::from_rgb(0x38, 0x38, 0x38),
        );
        let fill_ratio = (dev.battery_percent.min(100) as f32) / 100.0;
        let fill_rect = egui::Rect::from_min_size(
            bar_rect.min,
            Vec2::new(bar_width * fill_ratio, bar_height),
        );
        ui.painter()
            .rect_filled(fill_rect, CornerRadius::same(3), pct_color);

        ui.painter().text(
            egui::pos2(inner.right(), center_y),
            egui::Align2::RIGHT_CENTER,
            pct_text,
            FontId::proportional(13.0),
            pct_color,
        );

        if response.clicked() {
            is_enabled = !is_enabled;
            let mut cfg = self.config.lock().unwrap();
            if is_enabled {
                cfg.enabled_device_ids.insert(dev.id.clone());
            } else {
                cfg.enabled_device_ids.remove(&dev.id);
            }
            cfg.save();
        }
    }
}

impl eframe::App for SetupercentApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.tray.poll_events() {
            TrayAction::ShowWindow => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            }
            TrayAction::Quit => {
                std::process::exit(0);
            }
            TrayAction::None => {}
        }

        if ctx.input(|i| i.viewport().close_requested()) {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
        }

        let current_devices = self.devices.lock().unwrap().clone();
        self.sync_tray_icon(&current_devices);

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(COLOR_BG).inner_margin(Margin::same(14)))
            .show(ctx, |ui| {
                let lang = self.config.lock().unwrap().language;
                let t = tr(lang);

                ui.horizontal(|ui| {
                    ui.label(RichText::new("setupercent").color(COLOR_TEXT).size(20.0).strong());

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        egui::Frame::new()
                            .fill(Color32::from_rgba_unmultiplied(0x46, 0x6e, 0xce, 55))
                            .corner_radius(CornerRadius::same(10))
                            .inner_margin(Margin::symmetric(8, 3))
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                                        .color(COLOR_ACCENT)
                                        .size(11.0)
                                        .strong(),
                                );
                            });

                        ui.add_space(8.0);

                        let mut new_lang: Option<Language> = None;
                        if ui
                            .selectable_label(
                                lang == Language::Ru,
                                RichText::new("RU").size(11.0).strong(),
                            )
                            .clicked()
                        {
                            new_lang = Some(Language::Ru);
                        }
                        if ui
                            .selectable_label(
                                lang == Language::En,
                                RichText::new("EN").size(11.0).strong(),
                            )
                            .clicked()
                        {
                            new_lang = Some(Language::En);
                        }

                        if let Some(l) = new_lang {
                            if l != lang {
                                let mut cfg = self.config.lock().unwrap();
                                cfg.language = l;
                                cfg.save();
                            }
                        }
                    });
                });

                ui.add_space(14.0);

                ui.label(
                    RichText::new(t.devices_section)
                        .color(COLOR_TEXT_MUTED)
                        .size(10.5)
                        .strong(),
                );
                ui.add_space(8.0);

                if current_devices.is_empty() {
                    egui::Frame::new()
                        .fill(COLOR_CARD_BG)
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(Margin::symmetric(12, 12))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new(t.no_devices)
                                    .color(COLOR_TEXT_MUTED)
                                    .size(12.0),
                            );
                        });
                } else {
                    egui::ScrollArea::vertical()
                        .max_height(150.0)
                        .auto_shrink([false, true])
                        .show(ui, |ui| {
                            for dev in &current_devices {
                                self.draw_device_card(ui, dev);
                                ui.add_space(6.0);
                            }
                        });
                }

                ui.add_space(12.0);

                ui.label(
                    RichText::new(t.interval_section)
                        .color(COLOR_TEXT_MUTED)
                        .size(10.5)
                        .strong(),
                );
                ui.add_space(8.0);

                let mut cfg = self.config.lock().unwrap();
                let mut changed = false;

                egui::Frame::new()
                    .fill(COLOR_CARD_BG)
                    .corner_radius(CornerRadius::same(8))
                    .inner_margin(Margin::symmetric(12, 10))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(t.interval).color(COLOR_TEXT).size(12.0));
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.label(
                                    RichText::new(format!(
                                        "{} {}",
                                        cfg.poll_interval_minutes, t.min_unit
                                    ))
                                    .color(COLOR_ACCENT)
                                    .size(12.0)
                                    .strong(),
                                );
                            });
                        });
                        ui.add_space(6.0);
                        ui.scope(|ui| {
                            ui.visuals_mut().widgets.inactive.bg_fill =
                                Color32::from_rgb(0xdd, 0xdd, 0xdd);
                            ui.visuals_mut().widgets.inactive.fg_stroke =
                                Stroke::new(1.0, COLOR_BG);
                            ui.visuals_mut().widgets.hovered.bg_fill = COLOR_ACCENT;
                            ui.visuals_mut().widgets.hovered.fg_stroke =
                                Stroke::new(1.0, COLOR_TEXT);
                            ui.visuals_mut().widgets.active.bg_fill = COLOR_ACCENT;
                            ui.visuals_mut().widgets.active.fg_stroke =
                                Stroke::new(1.0, COLOR_TEXT);

                            ui.spacing_mut().slider_width = ui.available_width();
                            let slider =
                                egui::Slider::new(&mut cfg.poll_interval_minutes, 1..=60)
                                    .show_value(false);
                            if ui.add(slider).changed() {
                                changed = true;
                            }
                        });
                    });

                if changed {
                    cfg.save();
                }
                drop(cfg);
            });

        let current_lang = self.config.lock().unwrap().language;
        if current_lang != self.last_lang {
            let t = tr(current_lang);
            self.tray.set_language(t.tray_open, t.tray_quit);
            self.last_lang = current_lang;
            self.last_tooltip = None;
        }

        ctx.request_repaint_after(std::time::Duration::from_millis(250));
    }
}