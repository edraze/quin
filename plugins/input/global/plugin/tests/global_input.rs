// Avoid user input during tests in this module because it can affect results
#[cfg(test)]
mod test {
    use bevy::app::App;
    use bevy::prelude::Events;
    use rdev::EventType;
    use serial_test::serial;

    use global_input_api::filter::{FilterInput, FilterKeyEvent, InputFilterEvent};
    use global_input_api::input::InputEvent;
    use global_input_api::input_model::keyboard::{Key, KeyEvent};
    use global_input_api::input_model::mouse::{Button, ButtonEvent};
    use global_input_plugin::GlobalInputPlugin;
    use test_utils::InputStream;

    #[test]
    #[serial]
    fn listen_keyboard_input_test() {
        let mut app = App::new();
        app.add_plugins(GlobalInputPlugin);

        app.update();

        rdev::simulate(&EventType::KeyPress(rdev::Key::KeyT)).unwrap();

        app.update();

        let input_events = app.world.get_resource_mut::<Events<InputEvent>>();
        assert!(input_events.is_some());
        let input_events = input_events.unwrap();
        let mut input_events_reader = input_events.get_reader();
        let input_event = input_events_reader.read(&input_events).next();
        assert!(input_event.is_some());
        assert_eq!(input_event.unwrap(), &InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyT)));
    }

    #[test]
    #[serial]
    fn listen_mouse_buttons_input_test() {
        let mut app = App::new();
        app.add_plugins(GlobalInputPlugin);

        app.update();

        rdev::simulate(&EventType::ButtonPress(rdev::Button::Left)).unwrap();

        app.update();

        let input_events = app.world.get_resource_mut::<Events<InputEvent>>();
        assert!(input_events.is_some());
        let input_events = input_events.unwrap();
        let mut input_events_reader = input_events.get_reader();
        let input_event = input_events_reader.read(&input_events).next();
        assert!(input_event.is_some());
        assert_eq!(input_event.unwrap(), &InputEvent::MouseButton(ButtonEvent::Pressed(Button::Left)));
    }

    #[test]
    #[serial]
    fn block_keyboard_key_input_test() {
        let mut app = App::new();
        app.add_plugins(GlobalInputPlugin);

        {
            let filter_input_events = app.world.get_resource_mut::<Events<InputFilterEvent>>();
            assert!(filter_input_events.is_some());
            filter_input_events.unwrap().send(InputFilterEvent::Block(FilterInput::Keyboard(FilterKeyEvent::Pressed(Key::KeyT))));
        }

        app.update();

        let mut input_stream = InputStream::default();
        rdev::simulate(&EventType::KeyPress(rdev::Key::KeyT)).unwrap();

        app.update();

        // check input inside app
        {
            let input_events = app.world.get_resource_mut::<Events<InputEvent>>();
            assert!(input_events.is_some());
            let input_events = input_events.unwrap();
            let mut input_events_reader = input_events.get_reader();
            let input_event = input_events_reader.read(&input_events).next();
            assert!(input_event.is_some());
            assert_eq!(input_event.unwrap(), &InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyT)));
        }

        // check input outside app
        let input_event = input_stream.next();
        assert!(input_event.is_none());
    }

    #[test]
    #[serial]
    fn unblock_keyboard_key_input_test() {
        let mut app = App::new();
        app.add_plugins(GlobalInputPlugin);

        {
            let filter_input_events = app.world.get_resource_mut::<Events<InputFilterEvent>>();
            assert!(filter_input_events.is_some());
            filter_input_events.unwrap().send(InputFilterEvent::Block(FilterInput::Keyboard(FilterKeyEvent::Pressed(Key::KeyT))));
        }

        app.update();

        {
            let filter_input_events = app.world.get_resource_mut::<Events<InputFilterEvent>>();
            assert!(filter_input_events.is_some());
            filter_input_events.unwrap().send(InputFilterEvent::Unblock(FilterInput::Keyboard(FilterKeyEvent::Pressed(Key::KeyT))));
        }

        app.update();

        let mut input_stream = InputStream::default();
        rdev::simulate(&EventType::KeyPress(rdev::Key::KeyT)).unwrap();

        app.update();

        let input_event = input_stream.next();
        assert!(input_event.is_some());
        assert_eq!(input_event.unwrap(), EventType::KeyPress(rdev::Key::KeyT))
    }
}
