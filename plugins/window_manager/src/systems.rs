use bevy::prelude::EventReader;
use komorebi_client::{OperationDirection, SocketMessage};

use crate::events::{CloseWindow, FocusWindowDown, FocusWindowLeft, FocusWindowRight, FocusWindowUp, MinimizeWindow, MoveWindowDown, MoveWindowLeft, MoveWindowRight, MoveWindowUp, StackWindowDown, StackWindowLeft, StackWindowRight, StackWindowUp, ToggleFloat, ToggleMaximize, ToggleMonocle, UnstackWindow};

pub fn on_focus_left_system(mut events: EventReader<FocusWindowLeft>) {
    for _ in events.read() {
        println!("Handle FocusWindowLeft event");
        komorebi_client::send_message(&SocketMessage::FocusWindow(OperationDirection::Left))
            .map_err(|error| println!("Failed to execute FocusWindowLeft event, error: {error}"))
            .unwrap()
    }
}

pub fn on_focus_right_system(mut events: EventReader<FocusWindowRight>) {
    for _ in events.read() {
        println!("Handle FocusWindowRight event");
        komorebi_client::send_message(&SocketMessage::FocusWindow(OperationDirection::Right))
            .map_err(|error| println!("Failed to execute FocusWindowRight event, error: {error}"))
            .unwrap()
    }
}

pub fn on_focus_up_system(mut events: EventReader<FocusWindowUp>) {
    for _ in events.read() {
        println!("Handle FocusWindowUp event");
        komorebi_client::send_message(&SocketMessage::FocusWindow(OperationDirection::Up))
            .map_err(|error| println!("Failed to execute FocusWindowUp event, error: {error}"))
            .unwrap()
    }
}

pub fn on_focus_down_system(mut events: EventReader<FocusWindowDown>) {
    for _ in events.read() {
        println!("Handle FocusWindowDown event");
        komorebi_client::send_message(&SocketMessage::FocusWindow(OperationDirection::Down))
            .map_err(|error| println!("Failed to execute FocusWindowDown event, error: {error}"))
            .unwrap()
    }
}

pub fn on_move_left_system(mut events: EventReader<MoveWindowLeft>) {
    for _ in events.read() {
        println!("Handle MoveWindowLeft event");
        komorebi_client::send_message(&SocketMessage::MoveWindow(OperationDirection::Left))
            .map_err(|error| println!("Failed to execute MoveWindowLeft event, error: {error}"))
            .unwrap()
    }
}

pub fn on_move_right_system(mut events: EventReader<MoveWindowRight>) {
    for _ in events.read() {
        println!("Handle MoveWindowRight event");
        komorebi_client::send_message(&SocketMessage::MoveWindow(OperationDirection::Right))
            .map_err(|error| println!("Failed to execute MoveWindowRight event, error: {error}"))
            .unwrap()
    }
}

pub fn on_move_up_system(mut events: EventReader<MoveWindowUp>) {
    for _ in events.read() {
        println!("Handle MoveWindowUp event");
        komorebi_client::send_message(&SocketMessage::MoveWindow(OperationDirection::Up))
            .map_err(|error| println!("Failed to execute MoveWindowUp event, error: {error}"))
            .unwrap()
    }
}

pub fn on_move_down_system(mut events: EventReader<MoveWindowDown>) {
    for _ in events.read() {
        println!("Handle MoveWindowDown event");
        komorebi_client::send_message(&SocketMessage::MoveWindow(OperationDirection::Down))
            .map_err(|error| println!("Failed to execute FocusWindow event, error: {error}"))
            .unwrap()
    }
}

pub fn on_stack_left_system(mut events: EventReader<StackWindowLeft>) {
    for _ in events.read() {
        println!("Handle StackWindowLeft event");
        komorebi_client::send_message(&SocketMessage::StackWindow(OperationDirection::Left))
            .map_err(|error| println!("Failed to execute StackWindowLeft event, error: {error}"))
            .unwrap()
    }
}

pub fn on_stack_right_system(mut events: EventReader<StackWindowRight>) {
    for _ in events.read() {
        println!("Handle StackWindowRight event");
        komorebi_client::send_message(&SocketMessage::StackWindow(OperationDirection::Right))
            .map_err(|error| println!("Failed to execute StackWindowRight event, error: {error}"))
            .unwrap()
    }
}

pub fn on_stack_up_system(mut events: EventReader<StackWindowUp>) {
    for _ in events.read() {
        println!("Handle StackWindowUp event");
        komorebi_client::send_message(&SocketMessage::StackWindow(OperationDirection::Up))
            .map_err(|error| println!("Failed to execute StackWindowUp event, error: {error}"))
            .unwrap()
    }
}

pub fn on_stack_down_system(mut events: EventReader<StackWindowDown>) {
    for _ in events.read() {
        println!("Handle StackWindowDown event");
        komorebi_client::send_message(&SocketMessage::StackWindow(OperationDirection::Down))
            .map_err(|error| println!("Failed to execute StackWindowDown event, error: {error}"))
            .unwrap()
    }
}

pub fn on_unstack_system(mut events: EventReader<UnstackWindow>) {
    for _ in events.read() {
        println!("Handle UnstackWindow event");
        komorebi_client::send_message(&SocketMessage::UnstackWindow)
            .map_err(|error| println!("Failed to execute UnstackWindow event, error: {error}"))
            .unwrap()
    }
}

pub fn on_toggle_maximize_system(mut events: EventReader<ToggleMaximize>) {
    for _ in events.read() {
        println!("Handle ToggleMaximize event");
        komorebi_client::send_message(&SocketMessage::ToggleMaximize)
            .map_err(|error| println!("Failed to execute ToggleMaximize event, error: {error}"))
            .unwrap()
    }
}

pub fn on_toggle_monocle_system(mut events: EventReader<ToggleMonocle>) {
    for _ in events.read() {
        println!("Handle ToggleMonocle event");
        komorebi_client::send_message(&SocketMessage::ToggleMonocle)
            .map_err(|error| println!("Failed to execute ToggleMonocle event, error: {error}"))
            .unwrap()
    }
}

pub fn on_toggle_float_system(mut events: EventReader<ToggleFloat>) {
    for _ in events.read() {
        println!("Handle ToggleFloat event");
        komorebi_client::send_message(&SocketMessage::ToggleFloat)
            .map_err(|error| println!("Failed to execute ToggleFloat event, error: {error}"))
            .unwrap()
    }
}

pub fn on_minimize_window_system(mut events: EventReader<MinimizeWindow>) {
    for _ in events.read() {
        println!("Handle MinimizeWindow event");
        komorebi_client::send_message(&SocketMessage::Minimize)
            .map_err(|error| println!("Failed to execute MinimizeWindow event, error: {error}"))
            .unwrap()
    }
}

pub fn on_close_window_system(mut events: EventReader<CloseWindow>) {
    for _ in events.read() {
        println!("Handle CloseWindow event");
        komorebi_client::send_message(&SocketMessage::Close)
            .map_err(|error| println!("Failed to execute CloseWindow event, error: {error}"))
            .unwrap()
    }
}
