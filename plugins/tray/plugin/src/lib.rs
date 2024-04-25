use bevy::app::{App, AppExit, Plugin, Update};
use bevy::prelude::{EventReader, EventWriter};
use tray_api::{TrayClick, TrayItemClick};
use tray_icon::{TrayIcon, TrayIconBuilder, TrayIconEvent};
use tray_icon::menu::{Menu, MenuEvent, MenuItem};

const TRAY_PLUGIN_NAME: &str = "tray";
const EXIT_TRAY_ITEM_ID: &str = "Exit";

#[derive(Default)]
pub struct TrayPlugin;

impl Plugin for TrayPlugin {
    fn build(&self, app: &mut App) {
        app
            // .init_resource::<Tray>()
            .add_event::<TrayClick>()
            .add_event::<TrayItemClick>()
            // .add_event::<CreateTrayItem>()
            .add_systems(Update, (
                // create_tray,
                emmit_tray_event,
                emmit_tray_item_event,
                exit,
            ));
    }

    fn name(&self) -> &str {
        TRAY_PLUGIN_NAME
    }
}

// #[derive(Resource)]
// pub struct Tray {
//     items: Vec<String>,
// }
// 
// impl Tray {
//     fn add_item(&mut self, item: &str) {
//         self.items.push(item.into());
//     }
// 
//     fn update(&mut self) {
//         // let (s_chan, r_chan) = unbounded();
//         let items = self.items.clone();
//         println!("{items:?}"); 
//         let _tray_thread = thread::spawn(move|| {
//             let tray_icon = build_tray(&items);
//             loop {}
//         });
//     }
// }

// impl Default for Tray {
//     fn default() -> Self {
//         let mut tray = Self {
//             items: Vec::new()
//         };
//         // tray.update();
//         tray
//     }
// }

// fn create_tray(mut create_tray_item: EventReader<CreateTrayItem>, mut tray: ResMut<Tray>) {
//     for item in create_tray_item.read() {
//         tray.add_item(&item.id);
//         tray.update();
//     }
// }

fn emmit_tray_event(mut tray_click: EventWriter<TrayClick>) {
    if let Ok(_event) = TrayIconEvent::receiver().try_recv() {
        println!("{_event:?}");
        tray_click.send(TrayClick);
    }
}

fn emmit_tray_item_event(mut tray_item_click: EventWriter<TrayItemClick>) {
    if let Ok(event) = MenuEvent::receiver().try_recv() {
        println!("{event:?}");
        let event = TrayItemClick::new(&event.id.0);
        tray_item_click.send(event);
    }
}

fn exit(mut events: EventReader<TrayItemClick>, mut exit: EventWriter<AppExit>) {
    for event in events.read() {
        if event.id == EXIT_TRAY_ITEM_ID {
            exit.send(AppExit);
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
