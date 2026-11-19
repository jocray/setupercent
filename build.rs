use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=assets/images/setupercent_icon.ico");

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        let icon_path = "assets/images/setupercent_icon.ico";

        if Path::new(icon_path).exists() {
            let mut res = winresource::WindowsResource::new();
            res.set_icon(icon_path);
            res.set("ProductName", "setupercent");
            res.set("FileDescription", "Wireless devices battery percentage in system tray");
            res.compile().expect("Не удалось вшить ресурсы в .exe");
        } else {
            println!("cargo:warning=Файл assets/images/setupercent_icon.ico не найден, сборка пройдет со стандартной иконкой.");
        }
    }
}