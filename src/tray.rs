use crate::config::{Language, tr};
use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use image::{Rgba, RgbaImage};
use tray_icon::{
    Icon as TrayIconData, MouseButton, TrayIcon, TrayIconBuilder, TrayIconEvent,
    menu::{Menu, MenuEvent, MenuId, MenuItem},
};

pub enum TrayAction {
    ShowWindow,
    Quit,
    None,
}

pub struct TrayManager {
    tray_icon: TrayIcon,
    show_item: MenuItem,
    quit_item: MenuItem,
    show_id: MenuId,
    quit_id: MenuId,
    font: FontRef<'static>,
}

impl TrayManager {
    pub fn new(lang: Language) -> Result<Self, Box<dyn std::error::Error>> {
        let t = tr(lang);
        let font_data = include_bytes!("../assets/fonts/JOCRAY-Bold.ttf");
        let font = FontRef::try_from_slice(font_data)?;

        let menu = Menu::new();
        let show_item = MenuItem::new(t.tray_open, true, None);
        let quit_item = MenuItem::new(t.tray_quit, true, None);
        menu.append(&show_item)?;
        menu.append(&quit_item)?;

        let initial_icon = Self::render_percent_icon(&font, None)?;
        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("setupercent")
            .with_icon(initial_icon)
            .build()?;

        let show_id = show_item.id().clone();
        let quit_id = quit_item.id().clone();

        Ok(Self {
            tray_icon,
            show_item,
            quit_item,
            show_id,
            quit_id,
            font,
        })
    }

    pub fn set_language(&self, open: &str, quit: &str) {
        let _ = self.show_item.set_text(open);
        let _ = self.quit_item.set_text(quit);
    }

    pub fn poll_events(&self) -> TrayAction {
        let mut show = false;
        while let Ok(event) = MenuEvent::receiver().try_recv() {
            if event.id == self.quit_id {
                return TrayAction::Quit;
            }
            if event.id == self.show_id {
                show = true;
            }
        }
        while let Ok(event) = TrayIconEvent::receiver().try_recv() {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                ..
            } = event
            {
                show = true;
            }
        }
        if show {
            TrayAction::ShowWindow
        } else {
            TrayAction::None
        }
    }

    pub fn update_display(&self, percent: Option<u8>, tooltip: &str) {
        if let Ok(icon) = Self::render_percent_icon(&self.font, percent) {
            let _ = self.tray_icon.set_icon(Some(icon));
        }
        let _ = self.tray_icon.set_tooltip(Some(tooltip));
    }

    fn render_percent_icon(
        font: &FontRef<'static>,
        percent: Option<u8>,
    ) -> Result<TrayIconData, Box<dyn std::error::Error>> {
        let size = 32u32;
        let mut img = RgbaImage::from_pixel(size, size, Rgba([0, 0, 0, 0]));

        let (text, color) = match percent {
            Some(p) if p <= 20 => (p.to_string(), [245, 85, 85]),
            Some(100) => ("100".to_string(), [115, 225, 155]),
            Some(p) => (p.to_string(), [255, 255, 255]),
            None => ("--".to_string(), [170, 170, 170]),
        };

        let font_px = if text.len() >= 3 { 18.0 } else { 25.0 };
        let scale = PxScale::from(font_px);
        let scaled_font = font.as_scaled(scale);

        let mut total_width = 0.0f32;
        let mut glyphs = Vec::new();
        for ch in text.chars() {
            let glyph_id = scaled_font.glyph_id(ch);
            let advance = scaled_font.h_advance(glyph_id);
            glyphs.push((glyph_id, total_width));
            total_width += advance;
        }

        let start_x = ((size as f32 - total_width) / 2.0).max(0.0);
        let baseline_y = (size as f32 - (scaled_font.ascent() - scaled_font.descent())) / 2.0
            + scaled_font.ascent();

        for (glyph_id, offset_x) in glyphs {
            let glyph = glyph_id
                .with_scale_and_position(scale, ab_glyph::point(start_x + offset_x, baseline_y));
            if let Some(outlined) = font.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                outlined.draw(|gx, gy, coverage| {
                    let px = bounds.min.x as i32 + gx as i32;
                    let py = bounds.min.y as i32 + gy as i32;
                    if px >= 0 && px < size as i32 && py >= 0 && py < size as i32 {
                        let alpha = (coverage * 255.0).round() as u8;
                        if alpha > 0 {
                            img.put_pixel(
                                px as u32,
                                py as u32,
                                Rgba([color[0], color[1], color[2], alpha]),
                            );
                        }
                    }
                });
            }
        }

        Ok(TrayIconData::from_rgba(img.into_raw(), size, size)?)
    }
}
