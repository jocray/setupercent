use crate::battery::DeviceInfo;
use crate::config::{AppConfig, Language, Translations, tr};
use crate::tray::{TrayAction, TrayManager};
use eframe::egui::{
    self, Align, Color32, CornerRadius, FontFamily, FontId, Layout, Margin, RichText, Stroke,
    StrokeKind, Vec2,
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
const REPO_URL: &str = "https://github.com/jocray/setupercent";
const GITHUB_MARK_PATH: &str = "M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    Home,
    Settings,
    About,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Doc {
    License,
    Security,
    Contributing,
}

impl Doc {
    fn title(self) -> &'static str {
        match self {
            Doc::License => "LICENSE",
            Doc::Security => "SECURITY.md",
            Doc::Contributing => "CONTRIBUTING.md",
        }
    }

    fn content(self) -> &'static str {
        match self {
            Doc::License => include_str!("../LICENSE"),
            Doc::Security => include_str!("../SECURITY.md"),
            Doc::Contributing => include_str!("../CONTRIBUTING.md"),
        }
    }
}

pub struct SetupercentApp {
    devices: Arc<Mutex<Vec<DeviceInfo>>>,
    config: Arc<Mutex<AppConfig>>,
    tray: TrayManager,
    last_tooltip: Option<String>,
    last_lang: Language,
    active_tab: Tab,
    active_doc: Option<Doc>,
    app_icon: Option<egui::TextureHandle>,
    github_icon: egui::TextureHandle,
}

fn bold_font(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name("bold".into()))
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
        let app_icon =
            image::load_from_memory(include_bytes!("../assets/images/setupercent_icon.png"))
                .ok()
                .map(|img| {
                    let rgba = img.to_rgba8();
                    let size = [rgba.width() as usize, rgba.height() as usize];
                    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
                    cc.egui_ctx.load_texture(
                        "setupercent_app_icon",
                        color_image,
                        egui::TextureOptions::LINEAR,
                    )
                });
        let github_icon = cc.egui_ctx.load_texture(
            "github_mark",
            github_mark_image(64),
            egui::TextureOptions::LINEAR,
        );
        Self {
            devices,
            config,
            tray,
            last_tooltip: None,
            last_lang: lang,
            active_tab: Tab::Home,
            active_doc: None,
            app_icon,
            github_icon,
        }
    }

    fn setup_custom_font(ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "jocray_regular".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/fonts/JOCRAY-Regular.ttf"))
                .into(),
        );
        fonts.font_data.insert(
            "jocray_bold".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/fonts/JOCRAY-Bold.ttf")).into(),
        );
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, "jocray_regular".to_owned());
        let fallbacks = fonts
            .families
            .get(&FontFamily::Proportional)
            .cloned()
            .unwrap_or_default();
        let bold_family = fonts
            .families
            .entry(FontFamily::Name("bold".into()))
            .or_default();
        bold_family.insert(0, "jocray_bold".to_owned());
        bold_family.extend(fallbacks);
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

    fn version_badge(ui: &mut egui::Ui) {
        egui::Frame::new()
            .fill(Color32::from_rgba_unmultiplied(0x46, 0x6e, 0xce, 55))
            .corner_radius(CornerRadius::same(10))
            .inner_margin(Margin::symmetric(8, 3))
            .show(ui, |ui| {
                ui.label(
                    RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                        .font(bold_font(11.0))
                        .color(COLOR_ACCENT),
                );
            });
    }

    fn draw_tab_button(&mut self, ui: &mut egui::Ui, tab: Tab, label: &str) {
        let active = self.active_tab == tab;
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 34.0), egui::Sense::click());
        let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        let bg = if active {
            COLOR_CARD_BG_ACTIVE
        } else if response.hovered() {
            COLOR_CARD_BG_HOVER
        } else {
            COLOR_CARD_BG
        };
        let rounding = CornerRadius::same(8);
        ui.painter().rect_filled(rect, rounding, bg);
        if active {
            ui.painter().rect_stroke(
                rect,
                rounding,
                Stroke::new(1.0, COLOR_ACCENT),
                StrokeKind::Inside,
            );
        }
        let font = if active {
            bold_font(13.0)
        } else {
            FontId::proportional(13.0)
        };
        let color = if active || response.hovered() {
            COLOR_TEXT
        } else {
            COLOR_TEXT_MUTED
        };
        ui.painter().text(
            egui::pos2(rect.left() + 12.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            font,
            color,
        );
        if response.clicked() {
            self.active_tab = tab;
            self.active_doc = None;
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
        let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
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
        let pct_galley = ui
            .painter()
            .layout_no_wrap(pct_text.clone(), bold_font(13.0), pct_color);
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
        let fill_rect =
            egui::Rect::from_min_size(bar_rect.min, Vec2::new(bar_width * fill_ratio, bar_height));
        ui.painter()
            .rect_filled(fill_rect, CornerRadius::same(3), pct_color);
        ui.painter().text(
            egui::pos2(inner.right(), center_y),
            egui::Align2::RIGHT_CENTER,
            pct_text,
            bold_font(13.0),
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

    fn ui_home(&mut self, ui: &mut egui::Ui, devices: &[DeviceInfo], t: &Translations) {
        ui.label(
            RichText::new(t.devices_section)
                .font(bold_font(10.5))
                .color(COLOR_TEXT_MUTED),
        );
        ui.add_space(8.0);
        if devices.is_empty() {
            egui::Frame::new()
                .fill(COLOR_CARD_BG)
                .corner_radius(CornerRadius::same(8))
                .inner_margin(Margin::symmetric(12, 12))
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    ui.label(
                        RichText::new(t.no_devices)
                            .color(COLOR_TEXT_MUTED)
                            .size(12.0),
                    );
                });
        } else {
            egui::ScrollArea::vertical()
                .max_height(ui.available_height())
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    for dev in devices {
                        self.draw_device_card(ui, dev);
                        ui.add_space(6.0);
                    }
                });
        }
    }

    fn ui_settings(&mut self, ui: &mut egui::Ui, t: &Translations) {
        ui.label(
            RichText::new(t.interval_section)
                .font(bold_font(10.5))
                .color(COLOR_TEXT_MUTED),
        );
        ui.add_space(8.0);
        let mut cfg = self.config.lock().unwrap();
        let mut interval_changed = false;
        egui::Frame::new()
            .fill(COLOR_CARD_BG)
            .corner_radius(CornerRadius::same(8))
            .inner_margin(Margin::symmetric(12, 10))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(t.interval).color(COLOR_TEXT).size(12.0));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(
                            RichText::new(format!("{} {}", cfg.poll_interval_minutes, t.min_unit))
                                .font(bold_font(12.0))
                                .color(COLOR_ACCENT),
                        );
                    });
                });
                ui.add_space(6.0);
                ui.scope(|ui| {
                    ui.visuals_mut().widgets.inactive.bg_fill = Color32::from_rgb(0xdd, 0xdd, 0xdd);
                    ui.visuals_mut().widgets.inactive.fg_stroke = Stroke::new(1.0, COLOR_BG);
                    ui.visuals_mut().widgets.hovered.bg_fill = COLOR_ACCENT;
                    ui.visuals_mut().widgets.hovered.fg_stroke = Stroke::new(1.0, COLOR_TEXT);
                    ui.visuals_mut().widgets.active.bg_fill = COLOR_ACCENT;
                    ui.visuals_mut().widgets.active.fg_stroke = Stroke::new(1.0, COLOR_TEXT);
                    ui.spacing_mut().slider_width = ui.available_width();
                    ui.spacing_mut().slider_rail_height = 4.0;
                    let slider =
                        egui::Slider::new(&mut cfg.poll_interval_minutes, 1..=60).show_value(false);
                    if ui.add(slider).changed() {
                        interval_changed = true;
                    }
                });
            });
        if interval_changed {
            cfg.save();
        }
        ui.add_space(12.0);
        ui.label(
            RichText::new(t.notifications_section)
                .font(bold_font(10.5))
                .color(COLOR_TEXT_MUTED),
        );
        ui.add_space(8.0);
        let mut notifications_toggled = false;
        egui::Frame::new()
            .fill(COLOR_CARD_BG)
            .corner_radius(CornerRadius::same(8))
            .inner_margin(Margin::symmetric(12, 10))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(t.notifications_label)
                                .color(COLOR_TEXT)
                                .size(12.0),
                        );
                        ui.add_space(2.0);
                        ui.label(
                            RichText::new(t.notifications_hint)
                                .color(COLOR_TEXT_MUTED)
                                .size(10.5),
                        );
                    });
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if draw_toggle(ui, cfg.notifications_enabled).clicked() {
                            notifications_toggled = true;
                        }
                    });
                });
            });
        if notifications_toggled {
            cfg.notifications_enabled = !cfg.notifications_enabled;
            cfg.save();
        }
        ui.add_space(12.0);
        ui.label(
            RichText::new(t.language_section)
                .font(bold_font(10.5))
                .color(COLOR_TEXT_MUTED),
        );
        ui.add_space(8.0);
        let lang = cfg.language;
        let mut new_lang: Option<Language> = None;
        egui::Frame::new()
            .fill(COLOR_CARD_BG)
            .corner_radius(CornerRadius::same(8))
            .inner_margin(Margin::symmetric(12, 10))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(t.language_label).color(COLOR_TEXT).size(12.0));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.spacing_mut().button_padding = egui::vec2(10.0, 4.0);
                        if ui
                            .selectable_label(
                                lang == Language::Ru,
                                RichText::new("RU").font(bold_font(11.0)),
                            )
                            .clicked()
                        {
                            new_lang = Some(Language::Ru);
                        }
                        if ui
                            .selectable_label(
                                lang == Language::En,
                                RichText::new("EN").font(bold_font(11.0)),
                            )
                            .clicked()
                        {
                            new_lang = Some(Language::En);
                        }
                    });
                });
            });
        if let Some(l) = new_lang {
            if l != lang {
                cfg.language = l;
                cfg.save();
            }
        }
    }

    fn ui_about(&self, ui: &mut egui::Ui, t: &Translations) {
        ui.horizontal(|ui| {
            if let Some(icon) = &self.app_icon {
                ui.image((icon.id(), Vec2::splat(48.0)));
                ui.add_space(4.0);
            }
            ui.vertical(|ui| {
                ui.add_space(2.0);
                ui.label(
                    RichText::new("setupercent")
                        .font(bold_font(20.0))
                        .color(COLOR_TEXT),
                );
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(t.about_version)
                            .size(11.0)
                            .color(COLOR_TEXT_MUTED),
                    );
                    Self::version_badge(ui);
                });
            });
        });
        ui.add_space(12.0);
        ui.label(
            RichText::new(t.about_description)
                .size(12.5)
                .color(COLOR_TEXT_MUTED),
        );
        ui.add_space(12.0);
        egui::Frame::new()
            .fill(COLOR_CARD_BG)
            .corner_radius(CornerRadius::same(8))
            .inner_margin(Margin::symmetric(12, 10))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(t.about_license)
                            .size(12.0)
                            .color(COLOR_TEXT_MUTED),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(RichText::new("MIT").font(bold_font(12.0)).color(COLOR_TEXT));
                    });
                });
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(t.about_author)
                            .size(12.0)
                            .color(COLOR_TEXT_MUTED),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(
                            RichText::new("JOCRAY Media")
                                .font(bold_font(12.0))
                                .color(COLOR_TEXT),
                        );
                    });
                });
            });
    }

    fn ui_doc(&mut self, ui: &mut egui::Ui, doc: Doc, t: &Translations) {
        let font = bold_font(12.0);
        let galley = ui
            .painter()
            .layout_no_wrap(t.doc_back.to_owned(), font.clone(), COLOR_TEXT);
        let button_size = galley.size() + Vec2::new(24.0, 12.0);
        let (rect, response) = ui.allocate_exact_size(button_size, egui::Sense::click());
        let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        let bg = if response.hovered() {
            COLOR_CARD_BG_HOVER
        } else {
            COLOR_CARD_BG
        };
        ui.painter()
            .rect_filled(rect, CornerRadius::same(8), bg);
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            t.doc_back,
            font,
            COLOR_TEXT,
        );
        if response.clicked() {
            self.active_doc = None;
        }
        ui.add_space(12.0);
        ui.label(
            RichText::new(doc.title())
                .font(bold_font(16.0))
                .color(COLOR_TEXT),
        );
        ui.add_space(8.0);
        egui::Frame::new()
            .fill(COLOR_CARD_BG)
            .corner_radius(CornerRadius::same(8))
            .inner_margin(Margin::symmetric(12, 12))
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(ui.available_height())
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(doc.content())
                                .size(11.5)
                                .color(COLOR_TEXT_MUTED),
                        );
                    });
            });
    }
}

fn draw_toggle(ui: &mut egui::Ui, on: bool) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::new(40.0, 22.0), egui::Sense::click());
    let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
    let track_color = if on { COLOR_ACCENT } else { COLOR_TRACK };
    ui.painter()
        .rect_filled(rect, CornerRadius::same(11), track_color);
    let knob_x = if on {
        rect.right() - 11.0
    } else {
        rect.left() + 11.0
    };
    ui.painter()
        .circle_filled(egui::pos2(knob_x, rect.center().y), 8.0, COLOR_TEXT);
    response
}

fn footer_link(ui: &mut egui::Ui, text: &str, active: bool) -> bool {
    let font = FontId::proportional(11.0);
    let galley = ui
        .painter()
        .layout_no_wrap(text.to_owned(), font.clone(), COLOR_TEXT_MUTED);
    let (rect, response) = ui.allocate_exact_size(galley.size(), egui::Sense::click());
    let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
    let color = if active || response.hovered() {
        COLOR_ACCENT
    } else {
        COLOR_TEXT_MUTED
    };
    ui.painter().text(
        rect.left_center(),
        egui::Align2::LEFT_CENTER,
        text,
        font,
        color,
    );
    response.clicked()
}

fn github_icon(ui: &mut egui::Ui, texture: &egui::TextureHandle, size: f32) {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), egui::Sense::click());
    let response = response
        .on_hover_text("GitHub")
        .on_hover_cursor(egui::CursorIcon::PointingHand);
    let color = if response.hovered() {
        COLOR_ACCENT
    } else {
        COLOR_TEXT_MUTED
    };
    ui.painter().image(
        texture.id(),
        rect,
        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
        color,
    );
    if response.clicked() {
        let _ = open::that(REPO_URL);
    }
}

fn github_mark_points() -> Vec<egui::Pos2> {
    fn next_number(it: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Option<f32> {
        while matches!(it.peek(), Some(' ' | ',')) {
            it.next();
        }
        let mut s = String::new();
        if matches!(it.peek(), Some('-' | '+')) {
            s.push(*it.peek()?);
            it.next();
        }
        let mut digits = 0usize;
        while let Some(&c) = it.peek() {
            if c.is_ascii_digit() {
                s.push(c);
                it.next();
                digits += 1;
            } else {
                break;
            }
        }
        if matches!(it.peek(), Some('.')) {
            s.push('.');
            it.next();
            while let Some(&c) = it.peek() {
                if c.is_ascii_digit() {
                    s.push(c);
                    it.next();
                    digits += 1;
                } else {
                    break;
                }
            }
        }
        if digits == 0 {
            return None;
        }
        s.parse().ok()
    }
    let mut it = GITHUB_MARK_PATH.chars().peekable();
    let mut points = Vec::new();
    let mut cur = egui::pos2(0.0, 0.0);
    let mut cmd = 'M';
    loop {
        while matches!(it.peek(), Some(' ' | ',')) {
            it.next();
        }
        match it.peek() {
            None => break,
            Some(&c) if c.is_ascii_alphabetic() => {
                cmd = c;
                it.next();
            }
            _ => {}
        }
        match cmd {
            'M' => {
                let (Some(x), Some(y)) = (next_number(&mut it), next_number(&mut it)) else {
                    break;
                };
                cur = egui::pos2(x, y);
                points.push(cur);
                cmd = 'L';
            }
            'C' | 'c' => {
                let (Some(x1), Some(y1), Some(x2), Some(y2), Some(x3), Some(y3)) = (
                    next_number(&mut it),
                    next_number(&mut it),
                    next_number(&mut it),
                    next_number(&mut it),
                    next_number(&mut it),
                    next_number(&mut it),
                ) else {
                    break;
                };
                let (p1, p2, p3) = if cmd == 'c' {
                    (
                        cur + egui::vec2(x1, y1),
                        cur + egui::vec2(x2, y2),
                        cur + egui::vec2(x3, y3),
                    )
                } else {
                    (egui::pos2(x1, y1), egui::pos2(x2, y2), egui::pos2(x3, y3))
                };
                let p0 = cur;
                const STEPS: u32 = 16;
                for i in 1..=STEPS {
                    let t = i as f32 / STEPS as f32;
                    let mt = 1.0 - t;
                    points.push(egui::pos2(
                        mt * mt * mt * p0.x
                            + 3.0 * mt * mt * t * p1.x
                            + 3.0 * mt * t * t * p2.x
                            + t * t * t * p3.x,
                        mt * mt * mt * p0.y
                            + 3.0 * mt * mt * t * p1.y
                            + 3.0 * mt * t * t * p2.y
                            + t * t * t * p3.y,
                    ));
                }
                cur = p3;
            }
            _ => break,
        }
    }
    points
}

fn github_mark_image(size: usize) -> egui::ColorImage {
    const SS: usize = 4;
    let pts = github_mark_points();
    let n = size * SS;
    let scale = n as f32 / 24.0;
    let poly: Vec<egui::Pos2> = pts
        .iter()
        .map(|p| egui::pos2(p.x * scale, p.y * scale))
        .collect();
    let mut coverage = vec![0u8; n * n];
    if poly.len() >= 3 {
        for row in 0..n {
            let y = row as f32 + 0.5;
            let mut xs: Vec<f32> = Vec::new();
            let mut prev = poly[poly.len() - 1];
            for &cur in &poly {
                if (cur.y > y) != (prev.y > y) {
                    xs.push(prev.x + (y - prev.y) * (cur.x - prev.x) / (cur.y - prev.y));
                }
                prev = cur;
            }
            xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            for pair in xs.chunks_exact(2) {
                let start = (pair[0] - 0.5).ceil().max(0.0) as usize;
                let end = ((pair[1] - 0.5).ceil().max(0.0) as usize).min(n);
                for col in start..end {
                    coverage[row * n + col] = 1;
                }
            }
        }
    }
    let mut rgba = vec![255u8; size * size * 4];
    for py in 0..size {
        for px in 0..size {
            let mut sum = 0u32;
            for dy in 0..SS {
                for dx in 0..SS {
                    sum += u32::from(coverage[(py * SS + dy) * n + px * SS + dx]);
                }
            }
            rgba[(py * size + px) * 4 + 3] = (sum * 255 / (SS * SS) as u32) as u8;
        }
    }
    egui::ColorImage::from_rgba_unmultiplied([size, size], &rgba)
}

impl eframe::App for SetupercentApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
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
        let t = tr(self.config.lock().unwrap().language);
        egui::Panel::bottom("footer")
            .frame(
                egui::Frame::new()
                    .fill(COLOR_BG)
                    .inner_margin(Margin::symmetric(14, 8)),
            )
            .show(ui, |ui| {
                let (line, _) = ui.allocate_exact_size(
                    Vec2::new(ui.available_width(), 1.0),
                    egui::Sense::hover(),
                );
                ui.painter()
                    .rect_filled(line, CornerRadius::ZERO, COLOR_TRACK);
                ui.add_space(7.0);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 14.0;
                    if footer_link(ui, t.footer_license, self.active_doc == Some(Doc::License)) {
                        self.active_doc = Some(Doc::License);
                    }
                    if footer_link(ui, t.footer_security, self.active_doc == Some(Doc::Security)) {
                        self.active_doc = Some(Doc::Security);
                    }
                    if footer_link(
                        ui,
                        t.footer_contributing,
                        self.active_doc == Some(Doc::Contributing),
                    ) {
                        self.active_doc = Some(Doc::Contributing);
                    }
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        github_icon(ui, &self.github_icon, 18.0);
                    });
                });
            });
        egui::Panel::left("nav")
            .exact_size(150.0)
            .resizable(false)
            .frame(
                egui::Frame::new()
                    .fill(COLOR_BG)
                    .inner_margin(Margin::symmetric(10, 12)),
            )
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 6.0;
                self.draw_tab_button(ui, Tab::Home, t.tab_home);
                self.draw_tab_button(ui, Tab::Settings, t.tab_settings);
                self.draw_tab_button(ui, Tab::About, t.tab_about);
            });
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(COLOR_BG)
                    .inner_margin(Margin::same(14)),
            )
            .show(ui, |ui| {
                if let Some(doc) = self.active_doc {
                    self.ui_doc(ui, doc, t);
                    return;
                }
                ui.label(
                    RichText::new("setupercent")
                        .font(bold_font(20.0))
                        .color(COLOR_TEXT),
                );
                ui.add_space(14.0);
                match self.active_tab {
                    Tab::Home => self.ui_home(ui, &current_devices, t),
                    Tab::Settings => self.ui_settings(ui, t),
                    Tab::About => self.ui_about(ui, t),
                }
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