use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    En,
    Ru,
}

impl Default for Language {
    fn default() -> Self {
        Language::En
    }
}

pub struct Translations {
    pub devices_section: &'static str,
    pub interval_section: &'static str,
    pub interval: &'static str,
    pub min_unit: &'static str,
    pub no_devices: &'static str,
    pub no_selection_tooltip: &'static str,
    pub no_device_name: &'static str,
    pub tray_open: &'static str,
    pub tray_quit: &'static str,
    pub tab_home: &'static str,
    pub tab_settings: &'static str,
    pub tab_about: &'static str,
    pub language_section: &'static str,
    pub language_label: &'static str,
    pub notifications_section: &'static str,
    pub notifications_label: &'static str,
    pub notifications_hint: &'static str,
    pub toast_title: &'static str,
    pub toast_below: &'static str,
    pub doc_back: &'static str,
    pub about_description: &'static str,
    pub about_version: &'static str,
    pub about_license: &'static str,
    pub about_author: &'static str,
    pub footer_license: &'static str,
    pub footer_security: &'static str,
    pub footer_contributing: &'static str,
}

pub const EN: Translations = Translations {
    devices_section: "DEVICES",
    interval_section: "UPDATE INTERVAL",
    interval: "Interval",
    min_unit: "min",
    no_devices: "No wireless devices with battery sensor found",
    no_selection_tooltip: "setupercent — no selected devices",
    no_device_name: "Unknown device",
    tray_open: "Open setupercent",
    tray_quit: "Quit",
    tab_home: "Home",
    tab_settings: "Settings",
    tab_about: "About",
    language_section: "LANGUAGE",
    language_label: "Language",
    notifications_section: "NOTIFICATIONS",
    notifications_label: "Low battery alerts",
    notifications_hint: "Notify at 15%, 10% and 5%",
    toast_title: "Low battery",
    toast_below: "Battery is at {}% or lower",
    doc_back: "← Back",
    about_description: "A lightweight Windows system-tray utility that shows the battery level of your wireless mice, headphones and keyboards right on the tray icon.",
    about_version: "Version",
    about_license: "License",
    about_author: "Author",
    footer_license: "License",
    footer_security: "Security",
    footer_contributing: "Contributing",
};

pub const RU: Translations = Translations {
    devices_section: "УСТРОЙСТВА",
    interval_section: "ИНТЕРВАЛ ОБНОВЛЕНИЯ",
    interval: "Интервал",
    min_unit: "мин",
    no_devices: "Беспроводные устройства с датчиком заряда не найдены",
    no_selection_tooltip: "setupercent — нет выбранных устройств",
    no_device_name: "Неизвестное устройство",
    tray_open: "Открыть setupercent",
    tray_quit: "Выход",
    tab_home: "Главная",
    tab_settings: "Настройки",
    tab_about: "О проекте",
    language_section: "ЯЗЫК",
    language_label: "Язык",
    notifications_section: "УВЕДОМЛЕНИЯ",
    notifications_label: "Уведомления о низком заряде",
    notifications_hint: "Оповещать при 15%, 10% и 5%",
    toast_title: "Низкий заряд",
    toast_below: "Заряд {}% или ниже",
    doc_back: "← Назад",
    about_description: "Лёгкая утилита для системного трея Windows: показывает уровень заряда беспроводных мышей, наушников и клавиатур прямо на иконке в трее.",
    about_version: "Версия",
    about_license: "Лицензия",
    about_author: "Автор",
    footer_license: "Лицензия",
    footer_security: "Безопасность",
    footer_contributing: "Участие",
};

pub fn tr(lang: Language) -> &'static Translations {
    match lang {
        Language::En => &EN,
        Language::Ru => &RU,
    }
}

fn default_notifications_enabled() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub enabled_device_ids: HashSet<String>,
    pub poll_interval_minutes: u32,
    #[serde(default)]
    pub language: Language,
    #[serde(default = "default_notifications_enabled")]
    pub notifications_enabled: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            enabled_device_ids: HashSet::new(),
            poll_interval_minutes: 5,
            language: Language::default(),
            notifications_enabled: true,
        }
    }
}

impl AppConfig {
    fn ads_path() -> Option<String> {
        std::env::current_exe()
            .ok()
            .map(|path| format!("{}:config", path.display()))
    }

    fn fallback_path() -> Option<PathBuf> {
        std::env::var_os("APPDATA").map(|appdata| {
            PathBuf::from(appdata)
                .join("setupercent")
                .join("config.json")
        })
    }

    fn legacy_path() -> Option<PathBuf> {
        std::env::current_exe().ok().map(|mut path| {
            path.set_file_name("setupercent.json");
            path
        })
    }

    fn parse(content: &str) -> Option<Self> {
        let mut cfg = serde_json::from_str::<AppConfig>(content).ok()?;
        cfg.poll_interval_minutes = cfg.poll_interval_minutes.clamp(1, 60);
        Some(cfg)
    }

    pub fn load() -> Self {
        if let Some(path) = Self::ads_path() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Some(cfg) = Self::parse(&content) {
                    return cfg;
                }
            }
        }
        if let Some(path) = Self::fallback_path() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Some(cfg) = Self::parse(&content) {
                    return cfg;
                }
            }
        }
        if let Some(path) = Self::legacy_path() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Some(cfg) = Self::parse(&content) {
                    cfg.save();
                    let _ = fs::remove_file(&path);
                    return cfg;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        let Ok(json) = serde_json::to_string_pretty(self) else {
            return;
        };
        if let Some(path) = Self::ads_path() {
            if fs::write(&path, &json).is_ok() {
                return;
            }
        }
        if let Some(path) = Self::fallback_path() {
            if let Some(parent) = path.parent() {
                if fs::create_dir_all(parent).is_ok() {
                    let _ = fs::write(path, json);
                }
            }
        }
    }
}