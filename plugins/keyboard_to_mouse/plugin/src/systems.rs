use bevy::prelude::{EventReader, EventWriter, Res, ResMut};

use global_input_api::filter::{FilterInput, InputFilterEvent};
use mouse_output_api::{Direction, DragAndDrop, DragAndDropAction, MouseClick, MoveMouseRelatively, Scroll};
use mouse_output_api::mouse::Button;
use toggle::Active;

use crate::config::KeyboardToMouseConfig;
use crate::events::{ActivateKeyboardToMouse, DeactivateKeyboardToMouse, DragAndDropEnd, DragAndDropStart, MouseLeftButtonClick, MouseMiddleButtonClick, MouseRightButtonClick, MoveMouseRelativelyDown, MoveMouseRelativelyLeft, MoveMouseRelativelyRight, MoveMouseRelativelyUp, ScrollDown, ScrollLeft, ScrollRight, ScrollUp};
use crate::state::KeyboardToMouseState;

pub fn on_activate_keyboard_to_mouse_system(mut events: EventReader<ActivateKeyboardToMouse>, mut writer: EventWriter<InputFilterEvent>) {
    if events.read().count() > 0 {
        writer.send(InputFilterEvent::Block(FilterInput::FullKeyboardPress));
    }
}

pub fn on_deactivate_keyboard_to_mouse_system(mut events: EventReader<DeactivateKeyboardToMouse>,
                                              state: ResMut<KeyboardToMouseState>,
                                              mut drag_and_drop_end_writer: EventWriter<DragAndDropEnd>, 
                                              mut input_filter_writer: EventWriter<InputFilterEvent>) {
    if events.read().count() > 0 {
        if state.drag_and_drop_active {
           drag_and_drop_end_writer.send(DragAndDropEnd); 
        }
        input_filter_writer.send(InputFilterEvent::Unblock(FilterInput::FullKeyboardPress));
    }
}

pub fn on_move_mouse_relatively_up_system(mut events: EventReader<Active<MoveMouseRelativelyUp>>, config: Res<KeyboardToMouseConfig>, mut writer: EventWriter<MoveMouseRelatively>) {
    let mouse_speed = config.mouse_speed;
    writer.send_batch(events.read()
        .map(|_| MoveMouseRelatively::new(Direction::Up, mouse_speed)));
}

pub fn on_move_mouse_relatively_down_system(mut events: EventReader<Active<MoveMouseRelativelyDown>>, config: Res<KeyboardToMouseConfig>, mut writer: EventWriter<MoveMouseRelatively>) {
    let mouse_speed = config.mouse_speed;
    writer.send_batch(events.read()
        .map(|_| MoveMouseRelatively::new(Direction::Down, mouse_speed)));
}

pub fn on_move_mouse_relatively_left_system(mut events: EventReader<Active<MoveMouseRelativelyLeft>>, config: Res<KeyboardToMouseConfig>, mut writer: EventWriter<MoveMouseRelatively>) {
    let mouse_speed = config.mouse_speed;
    writer.send_batch(events.read()
        .map(|_| MoveMouseRelatively::new(Direction::Left, mouse_speed)));
}

pub fn on_move_mouse_relatively_right_system(mut events: EventReader<Active<MoveMouseRelativelyRight>>, config: Res<KeyboardToMouseConfig>, mut writer: EventWriter<MoveMouseRelatively>) {
    let mouse_speed = config.mouse_speed;
    writer.send_batch(events.read()
        .map(|_| MoveMouseRelatively::new(Direction::Right, mouse_speed)));
}

pub fn on_scroll_up_system(mut events: EventReader<Active<ScrollUp>>, config: Res<KeyboardToMouseConfig>, mut writer: EventWriter<Scroll>) {
    let scroll_speed = config.scroll_speed;
    writer.send_batch(events.read()
        .map(|_| Scroll::new(Direction::Up, scroll_speed)));
}

pub fn on_scroll_down_system(mut events: EventReader<Active<ScrollDown>>, config: Res<KeyboardToMouseConfig>, mut writer: EventWriter<Scroll>) {
    let scroll_speed = config.scroll_speed;
    writer.send_batch(events.read()
        .map(|_| Scroll::new(Direction::Down, scroll_speed)));
}

pub fn on_scroll_left_system(mut events: EventReader<Active<ScrollLeft>>, config: Res<KeyboardToMouseConfig>, mut writer: EventWriter<Scroll>) {
    let scroll_speed = config.scroll_speed;
    writer.send_batch(events.read()
        .map(|_| Scroll::new(Direction::Left, scroll_speed)));
}

pub fn on_scroll_right_system(mut events: EventReader<Active<ScrollRight>>, config: Res<KeyboardToMouseConfig>, mut writer: EventWriter<Scroll>) {
    let scroll_speed = config.scroll_speed;
    writer.send_batch(events.read()
        .map(|_| Scroll::new(Direction::Right, scroll_speed)));
}

pub fn on_mouse_left_button_click_system(mut events: EventReader<Active<MouseLeftButtonClick>>, mut writer: EventWriter<MouseClick>) {
    writer.send_batch(events.read()
        .map(|_| MouseClick::new(Button::Left)));
}

pub fn on_mouse_middle_button_click_system(mut events: EventReader<Active<MouseMiddleButtonClick>>, mut writer: EventWriter<MouseClick>) {
    writer.send_batch(events.read()
        .map(|_| MouseClick::new(Button::Middle)));
}

pub fn on_mouse_right_button_click_system(mut events: EventReader<Active<MouseRightButtonClick>>, mut writer: EventWriter<MouseClick>) {
    writer.send_batch(events.read()
        .map(|_| MouseClick::new(Button::Right)));
}

pub fn on_drag_and_drop_start_system(mut events: EventReader<Active<DragAndDropStart>>, 
                                     mut state: ResMut<KeyboardToMouseState>,
                                     mut writer: EventWriter<DragAndDrop>) {
    writer.send_batch(events.read()
        .map(|_| {
            state.drag_and_drop_active = true;
            DragAndDrop::new(DragAndDropAction::Start, Button::Left)
        }));
}

pub fn on_drag_and_drop_end_system(mut events: EventReader<Active<DragAndDropEnd>>,
                                   mut state: ResMut<KeyboardToMouseState>,
                                   mut writer: EventWriter<DragAndDrop>) {
    writer.send_batch(events.read()
        .map(|_| {
            state.drag_and_drop_active = false;
            DragAndDrop::new(DragAndDropAction::End, Button::Left)
        }));
}
