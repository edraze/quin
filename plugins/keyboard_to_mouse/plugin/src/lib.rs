use bevy::app::{App, Plugin};
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use config_loader::Config;
use global_input_api::input::InputEvent;
use global_input_api::input_model::keyboard::{Key, KeyEvent};
use input_sequence_api::Sequence;

const KEYBOARD_TO_MOUSE_PLUGIN_NAME: &str = "keyboard_to_mouse";

pub struct KeyboardToMousePlugin;

impl Plugin for KeyboardToMousePlugin {
    fn build(&self, app: &mut App) {
        let config = config_loader::load_config::<MouseEmulatorConfig>();
        app.insert_resource(config);
    }

    fn name(&self) -> &str {
        KEYBOARD_TO_MOUSE_PLUGIN_NAME
    }
}

#[derive(Resource, Serialize, Deserialize, Default, Debug)]
pub struct MouseEmulatorConfig {
    cursor_speed: i32,
    scroll_speed: i32,
    key_bindings: MouseEmulatorKeyBindings,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MouseEmulatorKeyBindings {
    activate: Vec<Sequence>,
    deactivate: Vec<Sequence>,
    mouse_move_up: Vec<Sequence>,
    mouse_move_down: Vec<Sequence>,
    mouse_move_left: Vec<Sequence>,
    mouse_move_right: Vec<Sequence>,
    mouse_left_button_click: Vec<Sequence>,
    mouse_right_button_click: Vec<Sequence>,
    mouse_middle_button_click: Vec<Sequence>,
    mouse_scroll_up: Vec<Sequence>,
    mouse_scroll_down: Vec<Sequence>,
    mouse_drag_and_drop_activate: Vec<Sequence>,
    mouse_drag_and_drop_deactivate: Vec<Sequence>,
}

impl Default for MouseEmulatorKeyBindings {
    fn default() -> Self {
        Self {
            activate: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::AltGr))])],
            deactivate: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Released(Key::AltGr))])],
            mouse_move_up: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyK))])],
            mouse_move_down: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyJ))])],
            mouse_move_left: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyH))])],
            mouse_move_right: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyL))])],
            mouse_left_button_click: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyI)),
                InputEvent::Keyboard(KeyEvent::Released(Key::KeyI)),
            ])],
            mouse_right_button_click: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyA)),
                InputEvent::Keyboard(KeyEvent::Released(Key::KeyA)),
            ])],
            mouse_middle_button_click: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyM)),
                InputEvent::Keyboard(KeyEvent::Released(Key::KeyM)),
            ])],
            mouse_scroll_up: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyU))])],
            mouse_scroll_down: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyD))])],
            mouse_drag_and_drop_activate: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyG)),
                InputEvent::Keyboard(KeyEvent::Released(Key::KeyG)),
            ])],
            mouse_drag_and_drop_deactivate: vec![Sequence::new(vec![
                InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyP)),
                InputEvent::Keyboard(KeyEvent::Released(Key::KeyP)),
            ])],
        }
    }
}

impl Config for MouseEmulatorConfig {
    fn name() -> String {
        KEYBOARD_TO_MOUSE_PLUGIN_NAME.to_string()
    }
}

// fn bind_sequence_to_event<E: Event>(app: &mut App, sequence: Sequence, mapper: impl Fn(EventReader<Sequence>, EventWriter<E>, Res<Persistent<MouseEmulatorConfig>>)) {
//     app.add_systems(Startup, {
//         let sequence = sequence.clone();
//         move |mut event_writer: EventWriter<Subscribe>| {
//             let subscription = Subscription {
//                 subscriber: KEYBOARD_TO_MOUSE_PLUGIN_NAME.to_string(),
//                 sequence: sequence.clone(),
//             };
//             let event = Subscribe(subscription);
//             event_writer.send(event);
//         }
//     });
//     app.add_systems(Update, mapper::<E>);
// }
// 
// fn sequence_to_event<E>(target_sequence: Sequence, mapper: impl Fn(EventReader<Sequence>, EventWriter<E>, Res<Persistent<MouseEmulatorConfig>>, &Sequence)) -> impl Fn(EventReader<Sequence>, EventWriter<E>, Res<Persistent<MouseEmulatorConfig>>)
//     where E: Event {
//     move |sequence_events: EventReader<Sequence>, event_writer: EventWriter<E>, config: Res<Persistent<MouseEmulatorConfig>>| {
//         mapper(sequence_events, event_writer, config, &target_sequence);
//     }
// }
// 
// fn sequence_to_mouse_move_relatively_up(mut sequence_events: EventReader<Sequence>, mut event_writer: EventWriter<MoveMouseRelatively>,
//                                         config: Res<Persistent<MouseEmulatorConfig>>, target_sequence: &Sequence) {
//     let event = MoveMouseRelatively::new(Direction::Up, config.cursor_speed);
//     sequence_events
//         .read()
//         .for_each(|sequence| {
//             if target_sequence.eq(sequence) {
//                 event_writer.send(event.clone());
//             }
//         });
// }

