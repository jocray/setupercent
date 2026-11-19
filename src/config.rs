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
    pub tray_open: &'static str,
    pub tray_quit: &'static str,
}

pub const EN: Translations = Translations {
    devices_section: "DEVICES",
    interval_section: "UPDATE INTERVAL",
    interval: "Interval",
    min_unit: "min",
    no_devices: "No wireless devices with battery sensor found",
    no_selection_tooltip: "setupercent — no selected devices",
    tray_open: "Open setupercent",
    tray_quit: "Quit",
};

pub const RU: Translations = Translations {
    devices_section: "УСТРОЙСТВА",
    interval_section: "ИНТЕРВАЛ ОБНОВЛЕНИЯ",
    interval: "Интервал",
    min_unit: "мин",
    no_devices: "Беспроводные устройства с датчиком заряда не найдены",
    no_selection_tooltip: "setupercent — нет выбранных устройств",
    tray_open: "Открыть setupercent",
    tray_quit: "Выход",
};

pub fn tr(lang: Language) -> &'static Translations {
    match lang {
        Language::En => &EN,
        Language::Ru => &RU,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub enabled_device_ids: HashSet<String>,
    pub poll_interval_minutes: u32,
    #[serde(default)]
    pub language: Language,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            enabled_device_ids: HashSet::new(),
            poll_interval_minutes: 5,
            language: Language::default(),
        }
    }
}

impl AppConfig {
    fn config_path() -> PathBuf {
        if let Ok(mut exe_path) = std::env::current_exe() {
            exe_path.set_file_name("setupercent.json");
            return exe_path;
        }
        PathBuf::from("setupercent.json")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(mut cfg) = serde_json::from_str::<AppConfig>(&content) {
                cfg.poll_interval_minutes = cfg.poll_interval_minutes.clamp(1, 60);
                return cfg;
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        let path = Self::config_path();
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }
}