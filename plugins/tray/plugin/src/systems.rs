use bevy::app::AppExit;
use bevy::prelude::{EventReader, EventWriter, NonSendMut, World};
use tray_icon::menu::MenuEvent;
use tray_icon::TrayIconEvent;

use tray_api::{CreateTrayItem, TrayClick, TrayItemClick};

use crate::state::{EXIT_TRAY_ITEM_ID, Tray};


pub fn init_tray_system(world: &mut World) {
    let tray = Tray::default();
    world.insert_non_send_resource(tray)
}

pub fn create_tray_system(mut create_tray_item: EventReader<CreateTrayItem>, mut tray: NonSendMut<Tray>) {
    for item in create_tray_item.read() {
        tray.add_item(&item.id);
        tray.update();
    }
}

pub fn emmit_tray_event_system(mut tray_click: EventWriter<TrayClick>) {
    if let Ok(_event) = TrayIconEvent::receiver().try_recv() {
        println!("{_event:?}");
        tray_click.send(TrayClick);
    }
}

pub fn emmit_tray_item_event_system(mut tray_item_click: EventWriter<TrayItemClick>) {
    if let Ok(event) = MenuEvent::receiver().try_recv() {
        println!("{event:?}");
        let event = TrayItemClick::new(&event.id.0);
        tray_item_click.send(event);
    }
}

pub fn exit_system(mut events: EventReader<TrayItemClick>, mut exit: EventWriter<AppExit>) {
    for event in events.read() {
        if event.id == EXIT_TRAY_ITEM_ID {
            exit.send(AppExit);
        }
    }
}