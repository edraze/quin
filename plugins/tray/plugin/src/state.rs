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
    let icon = load_icon();
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

fn load_icon() -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(include_bytes!("../assets/icon.png"))
            .expect("Failed to load icon from bytes")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to load icon")
}