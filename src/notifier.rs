use crate::battery::DeviceInfo;
use crate::config::{Language, tr};
use std::collections::{HashMap, HashSet};
use winrt_notification::{Duration, Sound, Toast};

const THRESHOLDS: [u8; 3] = [15, 10, 5];
const RESET_THRESHOLD: u8 = 15;

pub struct LowBatteryNotifier {
    fired: HashMap<String, HashSet<u8>>,
}

impl LowBatteryNotifier {
    pub fn new() -> Self {
        Self {
            fired: HashMap::new(),
        }
    }

    pub fn seed(&mut self, devices: &[DeviceInfo]) {
        for dev in devices {
            let fired = self.fired.entry(dev.id.clone()).or_default();
            for threshold in THRESHOLDS {
                if dev.battery_percent <= threshold {
                    fired.insert(threshold);
                }
            }
        }
    }

    pub fn process(
        &mut self,
        devices: &[DeviceInfo],
        enabled_ids: &HashSet<String>,
        notifications_enabled: bool,
        lang: Language,
    ) {
        let present: HashSet<&str> = devices.iter().map(|dev| dev.id.as_str()).collect();
        self.fired.retain(|id, _| present.contains(id.as_str()));
        let t = tr(lang);
        for dev in devices {
            if dev.battery_percent > RESET_THRESHOLD {
                self.fired.insert(dev.id.clone(), HashSet::new());
                continue;
            }
            let first_seen = !self.fired.contains_key(&dev.id);
            let fired = self.fired.entry(dev.id.clone()).or_default();
            let pending: Vec<u8> = THRESHOLDS
                .iter()
                .copied()
                .filter(|threshold| {
                    dev.battery_percent <= *threshold && !fired.contains(threshold)
                })
                .collect();
            let Some(&lowest) = pending.last() else {
                continue;
            };
            if !first_seen && notifications_enabled && enabled_ids.contains(&dev.id) {
                let name = if dev.name.is_empty() {
                    t.no_device_name
                } else {
                    dev.name.as_str()
                };
                let line1 = format!("{}: {}%", name, dev.battery_percent);
                let line2 = t.toast_below.replace("{}", &lowest.to_string());
                let _ = Toast::new(Toast::POWERSHELL_APP_ID)
                    .title(t.toast_title)
                    .text1(line1.as_str())
                    .text2(line2.as_str())
                    .sound(Some(Sound::Default))
                    .duration(Duration::Short)
                    .show();
            }
            for threshold in pending {
                fired.insert(threshold);
            }
        }
    }
}