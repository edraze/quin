use bevy::app::{App, Plugin};
use bevy::prelude::{Event, EventReader, EventWriter, Res, Resource};
use serde::{Deserialize, Serialize};

use config_loader::Config;
use global_input_api::filter::{FilterInput, InputFilterEvent};
use global_input_api::input::InputEvent;
use global_input_api::input_model::keyboard::{Key, KeyEvent};
use global_input_api::input_model::mouse::Button;
use input_sequence_api::Sequence;
use input_sequence_plugin::{listen_sequences, ToEvent};
use mouse_output_api::{Direction, DragAndDrop, DragAndDropAction, MouseClick, MoveMouseRelatively, Scroll};

const KEYBOARD_TO_MOUSE_PLUGIN_NAME: &str = "keyboard_to_mouse";

pub struct KeyboardToMousePlugin;

impl Plugin for KeyboardToMousePlugin {
    fn build(&self, app: &mut App) {
        let config = config_loader::load::<KeyboardToMouse>();
        app.insert_resource(config.clone());

        listen_sequences(app, config.key_bindings.activate.clone(), ToEvent::from_event(ActivateKeyboardToMouse), on_activate_keyboard_to_mouse);
        listen_sequences(app, config.key_bindings.deactivate.clone(), ToEvent::from_event(DeactivateKeyboardToMouse), on_deactivate_keyboard_to_mouse);
        
        listen_sequences(app, config.key_bindings.mouse_move_up.clone(), ToEvent::from_event(MoveMouseRelativelyUp), on_move_mouse_relatively_up);
        listen_sequences(app, config.key_bindings.mouse_move_down.clone(), ToEvent::from_event(MoveMouseRelativelyDown), on_move_mouse_relatively_down);
        listen_sequences(app, config.key_bindings.mouse_move_left.clone(), ToEvent::from_event(MoveMouseRelativelyLeft), on_move_mouse_relatively_left);
        listen_sequences(app, config.key_bindings.mouse_move_right.clone(), ToEvent::from_event(MoveMouseRelativelyRight), on_move_mouse_relatively_right);

        listen_sequences(app, config.key_bindings.mouse_scroll_up.clone(), ToEvent::from_event(ScrollUp), on_scroll_up);
        listen_sequences(app, config.key_bindings.mouse_scroll_down.clone(), ToEvent::from_event(ScrollDown), on_scroll_down);
        listen_sequences(app, config.key_bindings.mouse_scroll_left.clone(), ToEvent::from_event(ScrollLeft), on_scroll_left);
        listen_sequences(app, config.key_bindings.mouse_scroll_right.clone(), ToEvent::from_event(ScrollRight), on_scroll_right);

        listen_sequences(app, config.key_bindings.mouse_left_button_click.clone(), ToEvent::from_event(MouseLeftButtonClick), on_mouse_left_button_click);
        listen_sequences(app, config.key_bindings.mouse_middle_button_click.clone(), ToEvent::from_event(MouseMiddleButtonClick), on_mouse_middle_button_click);
        listen_sequences(app, config.key_bindings.mouse_right_button_click.clone(), ToEvent::from_event(MouseRightButtonClick), on_mouse_right_button_click);

        listen_sequences(app, config.key_bindings.mouse_drag_and_drop_activate.clone(), ToEvent::from_event(DragAndDropStart), on_drag_and_drop_start);
        listen_sequences(app, config.key_bindings.mouse_drag_and_drop_deactivate.clone(), ToEvent::from_event(DragAndDropEnd), on_drag_and_drop_end);
    }

    fn name(&self) -> &str {
        KEYBOARD_TO_MOUSE_PLUGIN_NAME
    }
}

#[derive(Resource, Serialize, Deserialize, Debug, Clone)]
pub struct KeyboardToMouse {
    mouse_speed: i32,
    scroll_speed: i64,
    key_bindings: KeyboardToMouseKeyBindings,
}

impl Default for KeyboardToMouse {
    fn default() -> Self {
        Self {
            mouse_speed: 10,
            scroll_speed: 1,
            key_bindings: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyboardToMouseKeyBindings {
    activate: Vec<Sequence>,
    deactivate: Vec<Sequence>,
    mouse_move_up: Vec<Sequence>,
    mouse_move_down: Vec<Sequence>,
    mouse_move_left: Vec<Sequence>,
    mouse_move_right: Vec<Sequence>,
    mouse_scroll_up: Vec<Sequence>,
    mouse_scroll_down: Vec<Sequence>,
    mouse_scroll_left: Vec<Sequence>,
    mouse_scroll_right: Vec<Sequence>,
    mouse_left_button_click: Vec<Sequence>,
    mouse_right_button_click: Vec<Sequence>,
    mouse_middle_button_click: Vec<Sequence>,
    mouse_drag_and_drop_activate: Vec<Sequence>,
    mouse_drag_and_drop_deactivate: Vec<Sequence>,
}

impl Default for KeyboardToMouseKeyBindings {
    fn default() -> Self {
        Self {
            activate: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::ControlRight))])],
            deactivate: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Released(Key::ControlRight))])],
            mouse_move_up: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyK))])],
            mouse_move_down: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyJ))])],
            mouse_move_left: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyH))])],
            mouse_move_right: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyL))])],
            mouse_scroll_up: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyU))])],
            mouse_scroll_down: vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyD))])],
            mouse_scroll_left: vec![],
            mouse_scroll_right: vec![],
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

impl Config for KeyboardToMouse {
    fn name() -> String {
        KEYBOARD_TO_MOUSE_PLUGIN_NAME.to_string()
    }
}

#[derive(Event, Debug, Clone)]
struct ActivateKeyboardToMouse;

fn on_activate_keyboard_to_mouse(mut events: EventReader<ActivateKeyboardToMouse>, mut writer: EventWriter<InputFilterEvent>) {
    if events.read().count() > 0 {
       writer.send(InputFilterEvent::Block(FilterInput::FullKeyboardPress));
    }
}

#[derive(Event, Debug, Clone)]
struct DeactivateKeyboardToMouse;

fn on_deactivate_keyboard_to_mouse(mut events: EventReader<DeactivateKeyboardToMouse>, mut writer: EventWriter<InputFilterEvent>) {
    if events.read().count() > 0 {
        writer.send(InputFilterEvent::Unblock(FilterInput::FullKeyboardPress));
    }
}

#[derive(Event, Debug, Clone)]
struct MoveMouseRelativelyUp;

fn on_move_mouse_relatively_up(mut events: EventReader<MoveMouseRelativelyUp>, config: Res<KeyboardToMouse>, mut writer: EventWriter<MoveMouseRelatively>) {
    let mouse_speed = config.mouse_speed;
    writer.send_batch(events.read()
        .map(|_| MoveMouseRelatively::new(Direction::Up, mouse_speed)));
}

#[derive(Event, Debug, Clone)]
struct MoveMouseRelativelyDown;

fn on_move_mouse_relatively_down(mut events: EventReader<MoveMouseRelativelyDown>, config: Res<KeyboardToMouse>, mut writer: EventWriter<MoveMouseRelatively>) {
    let mouse_speed = config.mouse_speed;
    writer.send_batch(events.read()
        .map(|_| MoveMouseRelatively::new(Direction::Down, mouse_speed)));
}

#[derive(Event, Debug, Clone)]
struct MoveMouseRelativelyLeft;

fn on_move_mouse_relatively_left(mut events: EventReader<MoveMouseRelativelyLeft>, config: Res<KeyboardToMouse>, mut writer: EventWriter<MoveMouseRelatively>) {
    let mouse_speed = config.mouse_speed;
    writer.send_batch(events.read()
        .map(|_| MoveMouseRelatively::new(Direction::Left, mouse_speed)));
}

#[derive(Event, Debug, Clone)]
struct MoveMouseRelativelyRight;

fn on_move_mouse_relatively_right(mut events: EventReader<MoveMouseRelativelyRight>, config: Res<KeyboardToMouse>, mut writer: EventWriter<MoveMouseRelatively>) {
    let mouse_speed = config.mouse_speed;
    writer.send_batch(events.read()
        .map(|_| MoveMouseRelatively::new(Direction::Right, mouse_speed)));
}

#[derive(Event, Debug, Clone)]
struct ScrollUp;

fn on_scroll_up(mut events: EventReader<ScrollUp>, config: Res<KeyboardToMouse>, mut writer: EventWriter<Scroll>) {
    let scroll_speed = config.scroll_speed;
    writer.send_batch(events.read()
        .map(|_| Scroll::new(Direction::Up, scroll_speed)));
}

#[derive(Event, Debug, Clone)]
struct ScrollDown;

fn on_scroll_down(mut events: EventReader<ScrollDown>, config: Res<KeyboardToMouse>, mut writer: EventWriter<Scroll>) {
    let scroll_speed = config.scroll_speed;
    writer.send_batch(events.read()
        .map(|_| Scroll::new(Direction::Down, scroll_speed)));
}

#[derive(Event, Debug, Clone)]
struct ScrollLeft;

fn on_scroll_left(mut events: EventReader<ScrollLeft>, config: Res<KeyboardToMouse>, mut writer: EventWriter<Scroll>) {
    let scroll_speed = config.scroll_speed;
    writer.send_batch(events.read()
        .map(|_| Scroll::new(Direction::Left, scroll_speed)));
}

#[derive(Event, Debug, Clone)]
struct ScrollRight;

fn on_scroll_right(mut events: EventReader<ScrollRight>, config: Res<KeyboardToMouse>, mut writer: EventWriter<Scroll>) {
    let scroll_speed = config.scroll_speed;
    writer.send_batch(events.read()
        .map(|_| Scroll::new(Direction::Right, scroll_speed)));
}

#[derive(Event, Debug, Clone)]
struct MouseLeftButtonClick;

fn on_mouse_left_button_click(mut events: EventReader<MouseLeftButtonClick>, mut writer: EventWriter<MouseClick>) {
    writer.send_batch(events.read()
        .map(|_| MouseClick::new(Button::Left)));
}

#[derive(Event, Debug, Clone)]
struct MouseMiddleButtonClick;

fn on_mouse_middle_button_click(mut events: EventReader<MouseMiddleButtonClick>, mut writer: EventWriter<MouseClick>) {
    writer.send_batch(events.read()
        .map(|_| MouseClick::new(Button::Middle)));
}

#[derive(Event, Debug, Clone)]
struct MouseRightButtonClick;

fn on_mouse_right_button_click(mut events: EventReader<MouseRightButtonClick>, mut writer: EventWriter<MouseClick>) {
    writer.send_batch(events.read()
        .map(|_| MouseClick::new(Button::Right)));
}


#[derive(Event, Debug, Clone)]
struct DragAndDropStart;

fn on_drag_and_drop_start(mut events: EventReader<DragAndDropStart>, mut writer: EventWriter<DragAndDrop>) {
    writer.send_batch(events.read()
        .map(|_| DragAndDrop::new(DragAndDropAction::Start, Button::Left)));
}

#[derive(Event, Debug, Clone)]
struct DragAndDropEnd;

fn on_drag_and_drop_end(mut events: EventReader<DragAndDropEnd>, mut writer: EventWriter<DragAndDrop>) {
    writer.send_batch(events.read()
        .map(|_| DragAndDrop::new(DragAndDropAction::End, Button::Left)));
}
