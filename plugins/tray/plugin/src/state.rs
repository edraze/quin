use tray_icon::{TrayIcon, TrayIconBuilder};
use tray_icon::menu::{Menu, MenuItem};

pub const EXIT_TRAY_ITEM_ID: &str = "Exit";

pub struct Tray {
    items: Vec<String>,
    icon: TrayIcon,
}

impl Tray {
    pub fn add_item(&mut self, item: &str) {
        self.items.push(item.into());
    }

    pub fn update(&mut self) {
        let items = self.items.clone();
        self.icon = build_tray(&items);
    }
}

impl Default for Tray {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            icon: build_tray(&[EXIT_TRAY_ITEM_ID.to_string()]),
        }
    }
}

pub fn build_tray(items: &[String]) -> TrayIcon {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/icon.png"); // todo embedded asset
    let icon = load_icon(std::path::Path::new(path));
    let tray_menu = Menu::new();
    for item in items {
        tray_menu.append(&MenuItem::with_id(item, item, true, None)).unwrap();
    }
    TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("quin")
        .with_icon(icon)
        .build()
        .unwrap()
}

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
