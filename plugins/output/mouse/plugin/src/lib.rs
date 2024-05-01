use bevy::app::Update;
use bevy::prelude::{App, EventReader, Plugin};
use enigo::{Enigo, MouseControllable};
use rdev::EventType;
use mouse_output_api::{Direction, DragAndDrop, DragAndDropAction, MouseClick, MoveMouseRelatively, MoveMouseToPosition, Scroll};
use mouse_output_api::mouse::Button;

const MOUSE_OUTPUT_PLUGIN_NAME: &str = "mouse_output";

pub struct MouseOutputPlugin;

impl Plugin for MouseOutputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveMouseRelatively>();
        app.add_event::<MoveMouseToPosition>();
        app.add_event::<Scroll>();
        app.add_event::<MouseClick>();
        app.add_event::<DragAndDrop>();
        app.add_systems(Update, on_move_mouse_relatively_event);
        app.add_systems(Update, on_move_mouse_to_position_event);
        app.add_systems(Update, on_scroll_event);
        app.add_systems(Update, on_click_event);
        app.add_systems(Update, on_drag_and_drop_event);
    }

    fn name(&self) -> &str {
        MOUSE_OUTPUT_PLUGIN_NAME
    }
}

fn on_move_mouse_relatively_event(mut events: EventReader<MoveMouseRelatively>) {
    events.read()
        .for_each(move_mouse_relatively);
}

fn on_move_mouse_to_position_event(mut events: EventReader<MoveMouseToPosition>) {
    events.read()
        .for_each(move_mouse_to_position);
}

fn on_scroll_event(mut events: EventReader<Scroll>) {
    events.read()
        .for_each(scroll);
}

fn on_click_event(mut events: EventReader<MouseClick>) {
    events.read()
        .for_each(|event| click(&event.button))
}

fn on_drag_and_drop_event(mut events: EventReader<DragAndDrop>) {
    events.read()
        .for_each(drag_and_drop);
}

fn move_mouse_to_position(move_mouse: &MoveMouseToPosition){
    println!("mouse move absolutely");
    let (x,y) = (move_mouse.x, move_mouse.y);
    rdev::simulate(&EventType::MouseMove { x, y }).unwrap();
}

// todo use one crate for global and relative mouse movements
fn move_mouse_relatively(move_mouse: &MoveMouseRelatively) {
    let distance = move_mouse.distance;
    match move_mouse.direction {
        Direction::Up => Enigo.mouse_move_relative(0, -distance),
        Direction::Down => Enigo.mouse_move_relative(0, distance),
        Direction::Left => Enigo.mouse_move_relative(-distance, 0),
        Direction::Right => Enigo.mouse_move_relative(distance, 0),
    }
}

fn scroll(scroll: &Scroll) {
    let distance = scroll.distance;
    match scroll.direction {
        Direction::Up => rdev::simulate(&EventType::Wheel { delta_x: 0, delta_y: distance }).unwrap(),
        Direction::Down => rdev::simulate(&EventType::Wheel { delta_x: 0, delta_y: -distance }).unwrap(),
        Direction::Left => rdev::simulate(&EventType::Wheel { delta_x: -distance, delta_y: 0 }).unwrap(),
        Direction::Right => rdev::simulate(&EventType::Wheel { delta_x: distance, delta_y: 0 }).unwrap()
    }
}

fn drag_and_drop(drag_and_drop: &DragAndDrop){
    let button = &drag_and_drop.button;
    match drag_and_drop.action {
        DragAndDropAction::Start => press_button(button),
        DragAndDropAction::End => release_button(button)
    }
}

fn click(button: &Button) {
    press_button(button);
    release_button(button);
}

fn press_button(button: &Button){
    rdev::simulate(&EventType::ButtonPress(button.into())).unwrap();
}

fn release_button(button: &Button) {
    rdev::simulate(&EventType::ButtonRelease(button.into())).unwrap();
}
