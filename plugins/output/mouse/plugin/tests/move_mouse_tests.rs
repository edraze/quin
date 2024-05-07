#[cfg(test)]
mod test {
    use bevy::app::App;
    use bevy::prelude::Events;
    use enigo::{Enigo, MouseControllable};
    use serial_test::serial;

    use mouse_output_api::{Direction, MoveMouseRelatively, MoveMouseToPosition};
    use mouse_output_plugin::MouseOutputPlugin;

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

        let new_mouse_location = Enigo.mouse_location();
        assert_ne!(new_mouse_location, mouse_location);
        assert_eq!(new_mouse_location, (100, 100))
    }
}
