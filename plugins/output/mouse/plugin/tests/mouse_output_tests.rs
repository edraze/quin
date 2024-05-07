// Avoid user input during tests in this module because it can affect results
#[cfg(test)]
mod test {
    use bevy::app::App;
    use bevy::prelude::Events;
    use enigo::{Enigo, MouseControllable};
    use rdev::EventType;
    use serial_test::serial;

    use mouse_output_api::{Direction, DragAndDrop, DragAndDropAction, MouseClick, MoveMouseRelatively, MoveMouseToPosition, Scroll};
    use mouse_output_api::mouse::Button;
    use mouse_output_plugin::MouseOutputPlugin;
    use test_utils::InputStream;

    #[test]
    #[serial]
    fn move_mouse_relatively_up_test() {
        let mut app = App::new();
        app.add_plugins(MouseOutputPlugin);

        let mouse_location = Enigo.mouse_location();

        let move_mouse_events = app.world.get_resource_mut::<Events<MoveMouseRelatively>>();
        assert!(move_mouse_events.is_some());
        let mut move_mouse_events = move_mouse_events.unwrap();
        move_mouse_events.send(MoveMouseRelatively::new(Direction::Up, 50));

        app.update();
        app.update();

        let new_mouse_location = Enigo.mouse_location();
        assert_eq!(new_mouse_location, (mouse_location.0, mouse_location.1 - 50))
    }

    #[test]
    #[serial]
    fn move_mouse_relatively_down_test() {
        let mut app = App::new();
        app.add_plugins(MouseOutputPlugin);

        let mouse_location = Enigo.mouse_location();

        let move_mouse_events = app.world.get_resource_mut::<Events<MoveMouseRelatively>>();
        assert!(move_mouse_events.is_some());
        let mut move_mouse_events = move_mouse_events.unwrap();
        move_mouse_events.send(MoveMouseRelatively::new(Direction::Down, 50));

        app.update();
        app.update();

        let new_mouse_location = Enigo.mouse_location();
        assert_eq!(new_mouse_location, (mouse_location.0, mouse_location.1 + 50))
    }

    #[test]
    #[serial]
    fn move_mouse_relatively_left_test() {
        let mut app = App::new();
        app.add_plugins(MouseOutputPlugin);

        let mouse_location = Enigo.mouse_location();

        let move_mouse_events = app.world.get_resource_mut::<Events<MoveMouseRelatively>>();
        assert!(move_mouse_events.is_some());
        let mut move_mouse_events = move_mouse_events.unwrap();
        move_mouse_events.send(MoveMouseRelatively::new(Direction::Left, 50));

        app.update();
        app.update();

        let new_mouse_location = Enigo.mouse_location();
        assert_eq!(new_mouse_location, (mouse_location.0 - 50, mouse_location.1))
    }

    #[test]
    #[serial]
    fn move_mouse_relatively_right_test() {
        let mut app = App::new();
        app.add_plugins(MouseOutputPlugin);

        let mouse_location = Enigo.mouse_location();

        let move_mouse_events = app.world.get_resource_mut::<Events<MoveMouseRelatively>>();
        assert!(move_mouse_events.is_some());
        let mut move_mouse_events = move_mouse_events.unwrap();
        move_mouse_events.send(MoveMouseRelatively::new(Direction::Right, 50));

        app.update();
        app.update();

        let new_mouse_location = Enigo.mouse_location();
        assert_eq!(new_mouse_location, (mouse_location.0 + 50, mouse_location.1))
    }

    #[test]
    #[serial]
    fn move_mouse_to_position_test() {
        let mut app = App::new();
        app.add_plugins(MouseOutputPlugin);

        let mouse_location = Enigo.mouse_location();

        let move_mouse_events = app.world.get_resource_mut::<Events<MoveMouseToPosition>>();
        assert!(move_mouse_events.is_some());
        let mut move_mouse_events = move_mouse_events.unwrap();
        move_mouse_events.send(MoveMouseToPosition::new(100., 100.));

        app.update();
        app.update();

        let new_mouse_location = Enigo.mouse_location();
        assert_ne!(new_mouse_location, mouse_location);
        assert_eq!(new_mouse_location, (100, 100))
    }

    #[test]
    #[serial]
    fn scroll_mouse_up_test() {
        let mut app = App::new();
        app.add_plugins(MouseOutputPlugin);

        let mut input_stream = InputStream::default();

        let scroll_events = app.world.get_resource_mut::<Events<Scroll>>();
        assert!(scroll_events.is_some());
        let mut move_mouse_events = scroll_events.unwrap();
        move_mouse_events.send(Scroll::new(Direction::Up, 1));

        app.update();
        app.update();

        let scroll_event = input_stream.next();
        assert!(scroll_event.is_some());
        assert_eq!(scroll_event.unwrap(), EventType::Wheel { delta_x: 0, delta_y: 1 });
    }

    #[test]
    #[serial]
    fn scroll_mouse_down_test() {
        let mut app = App::new();
        app.add_plugins(MouseOutputPlugin);

        let mut input_stream = InputStream::default();

        let scroll_events = app.world.get_resource_mut::<Events<Scroll>>();
        assert!(scroll_events.is_some());
        let mut move_mouse_events = scroll_events.unwrap();
        move_mouse_events.send(Scroll::new(Direction::Down, 1));

        app.update();
        app.update();

        let scroll_event = input_stream.next();
        assert!(scroll_event.is_some());
        assert_eq!(scroll_event.unwrap(), EventType::Wheel { delta_x: 0, delta_y: -1 });
    }

    #[test]
    #[serial]
    fn scroll_mouse_left_test() {
        let mut app = App::new();
        app.add_plugins(MouseOutputPlugin);

        let mut input_stream = InputStream::default();

        let scroll_events = app.world.get_resource_mut::<Events<Scroll>>();
        assert!(scroll_events.is_some());
        let mut move_mouse_events = scroll_events.unwrap();
        move_mouse_events.send(Scroll::new(Direction::Left, 1));

        app.update();
        app.update();

        let scroll_event = input_stream.next();
        assert!(scroll_event.is_some());
        assert_eq!(scroll_event.unwrap(), EventType::Wheel { delta_x: -1, delta_y: 0 });
    }

    #[test]
    #[serial]
    fn scroll_mouse_right_test() {
        let mut app = App::new();
        app.add_plugins(MouseOutputPlugin);

        let mut input_stream = InputStream::default();

        let scroll_events = app.world.get_resource_mut::<Events<Scroll>>();
        assert!(scroll_events.is_some());
        let mut move_mouse_events = scroll_events.unwrap();
        move_mouse_events.send(Scroll::new(Direction::Right, 1));

        app.update();
        app.update();

        let scroll_event = input_stream.next();
        assert!(scroll_event.is_some());
        assert_eq!(scroll_event.unwrap(), EventType::Wheel { delta_x: 1, delta_y: 0 });
    }

    #[ignore = "bug in rdev crate"]
    #[test]
    #[serial]
    fn left_click_test() {
        let mut app = App::new();
        app.add_plugins(MouseOutputPlugin);

        let mut input_stream = InputStream::default();

        let click_events = app.world.get_resource_mut::<Events<MouseClick>>();
        assert!(click_events.is_some());
        let mut click_events = click_events.unwrap();
        click_events.send(MouseClick::new(Button::Left));

        app.update();
        app.update();

        let press_event = input_stream.next();
        assert!(press_event.is_some());
        assert_eq!(press_event.unwrap(), EventType::ButtonPress(rdev::Button::Left));
        let release_event = input_stream.next();
        assert!(release_event.is_some());
        assert_eq!(release_event.unwrap(), EventType::ButtonRelease(rdev::Button::Left));
        let other_event = input_stream.next();
        assert!(other_event.is_none());
    }

    #[ignore = "bug in rdev crate"]
    #[test]
    #[serial]
    fn middle_click_test() {
        let mut app = App::new();
        app.add_plugins(MouseOutputPlugin);

        let mut input_stream = InputStream::default();

        let click_events = app.world.get_resource_mut::<Events<MouseClick>>();
        assert!(click_events.is_some());
        let mut click_events = click_events.unwrap();
        click_events.send(MouseClick::new(Button::Middle));

        app.update();
        app.update();

        let press_event = input_stream.next();
        assert!(press_event.is_some());
        assert_eq!(press_event.unwrap(), EventType::ButtonPress(rdev::Button::Middle));
        let release_event = input_stream.next();
        assert!(release_event.is_some());
        assert_eq!(release_event.unwrap(), EventType::ButtonRelease(rdev::Button::Middle));
        let other_event = input_stream.next();
        assert!(other_event.is_none());
    }

    #[ignore = "bug in rdev crate"]
    #[test]
    #[serial]
    fn right_click_test() {
        let mut app = App::new();
        app.add_plugins(MouseOutputPlugin);

        let mut input_stream = InputStream::default();

        let click_events = app.world.get_resource_mut::<Events<MouseClick>>();
        assert!(click_events.is_some());
        let mut click_events = click_events.unwrap();
        click_events.send(MouseClick::new(Button::Right));

        app.update();
        app.update();

        let press_event = input_stream.next();
        assert!(press_event.is_some());
        assert_eq!(press_event.unwrap(), EventType::ButtonPress(rdev::Button::Right));
        let release_event = input_stream.next();
        assert!(release_event.is_some());
        assert_eq!(release_event.unwrap(), EventType::ButtonRelease(rdev::Button::Right));
        let other_event = input_stream.next();
        assert!(other_event.is_none());
    }

    #[ignore = "bug in rdev crate"]
    #[test]
    #[serial]
    fn drag_and_drop_start_test() {
        let mut app = App::new();
        app.add_plugins(MouseOutputPlugin);

        let mut input_stream = InputStream::default();

        let drag_and_drop_events = app.world.get_resource_mut::<Events<DragAndDrop>>();
        assert!(drag_and_drop_events.is_some());
        let mut click_events = drag_and_drop_events.unwrap();
        click_events.send(DragAndDrop::new(DragAndDropAction::Start, Button::Left));

        app.update();
        app.update();

        let press_event = input_stream.next();
        assert!(press_event.is_some());
        assert_eq!(press_event.unwrap(), EventType::ButtonPress(rdev::Button::Left));
        let other_event = input_stream.next();
        assert!(other_event.is_none());
    }

    #[ignore = "bug in rdev crate"]
    #[test]
    #[serial]
    fn drag_and_drop_end_test() {
        let mut app = App::new();
        app.add_plugins(MouseOutputPlugin);

        let mut input_stream = InputStream::default();

        let drag_and_drop_events = app.world.get_resource_mut::<Events<DragAndDrop>>();
        assert!(drag_and_drop_events.is_some());
        let mut click_events = drag_and_drop_events.unwrap();
        click_events.send(DragAndDrop::new(DragAndDropAction::End, Button::Left));

        app.update();
        app.update();

        let press_event = input_stream.next();
        assert!(press_event.is_some());
        assert_eq!(press_event.unwrap(), EventType::ButtonRelease(rdev::Button::Left));
        let other_event = input_stream.next();
        assert!(other_event.is_none());
    }
}
