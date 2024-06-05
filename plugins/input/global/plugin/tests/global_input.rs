// Avoid user input during tests in this module because it can affect results
#[cfg(test)]
mod test {
    use bevy::app::App;
    use bevy::prelude::Events;
    use rdev::EventType;

    use global_input_api::filter::InputFilterEvent;
    use global_input_api::input::InputEvent;
    use global_input_api::input_model::Button::Left;
    use global_input_api::input_model::definition::{P, R};
    use global_input_api::input_model::filter::InputFilter;
    use global_input_api::input_model::Key::{ControlLeft, T};
    use global_input_plugin::GlobalInputPlugin;
    use test_utils::InputStream;

    // #[ignore = "unsupported on ci"]
    #[test]
    fn listen_keyboard_input_test() {
        let mut app = App::new();
        app.add_plugins(GlobalInputPlugin);

        app.update();

        rdev::simulate(&EventType::KeyPress(rdev::Key::KeyT)).unwrap();
        rdev::simulate(&EventType::KeyRelease(rdev::Key::KeyT)).unwrap();

        app.update();

        let input_events = app.world.get_resource_mut::<Events<InputEvent>>();
        assert!(input_events.is_some());
        let input_events = input_events.unwrap();
        let mut input_events_reader = input_events.get_reader();
        let input_event = input_events_reader.read(&input_events).next();
        assert!(input_event.is_some());
        assert_eq!(input_event.unwrap(), &P(T).into());
    }

    // #[ignore = "unsupported on ci"]
    #[test]
    fn listen_mouse_buttons_input_test() {
        let mut app = App::new();
        app.add_plugins(GlobalInputPlugin);

        app.update();

        rdev::simulate(&EventType::ButtonPress(rdev::Button::Left)).unwrap();
        rdev::simulate(&EventType::ButtonRelease(rdev::Button::Left)).unwrap();

        app.update();

        let input_events = app.world.get_resource_mut::<Events<InputEvent>>();
        assert!(input_events.is_some());
        let input_events = input_events.unwrap();
        let mut input_events_reader = input_events.get_reader();
        let input_event = input_events_reader.read(&input_events).next();
        assert!(input_event.is_some());
        assert_eq!(input_event.unwrap(), &P(Left).into());
    }

    // #[ignore = "unsupported on ci"]
    #[test]
    fn listen_modified_input_test() {
        let mut app = App::new();
        app.add_plugins(GlobalInputPlugin);

        app.update();

        rdev::simulate(&EventType::KeyPress(rdev::Key::ControlLeft)).unwrap();
        rdev::simulate(&EventType::KeyPress(rdev::Key::KeyT)).unwrap();
        rdev::simulate(&EventType::KeyRelease(rdev::Key::KeyT)).unwrap();
        rdev::simulate(&EventType::KeyRelease(rdev::Key::ControlLeft)).unwrap();

        app.update();
        app.update();

        let input_events = app.world.get_resource_mut::<Events<InputEvent>>();
        assert!(input_events.is_some());
        let input_events = input_events.unwrap();
        let mut input_events_reader = input_events.get_reader();
        let mut input_events_iter = input_events_reader.read(&input_events);
        let input_event = input_events_iter.next();
        assert!(input_event.is_some());
        assert_eq!(input_event.unwrap(), &P(ControlLeft).into());
        let input_event = input_events_iter.next();
        assert!(input_event.is_some());
        assert_eq!(input_event.unwrap(), &(ControlLeft, P(T).into()).into());
    }

    // #[ignore = "unsupported on ci"]
    #[test]
    fn block_keyboard_key_input_test() {
        let mut app = App::new();
        app.add_plugins(GlobalInputPlugin);

        {
            let filter_input_events = app.world.get_resource_mut::<Events<InputFilterEvent>>();
            assert!(filter_input_events.is_some());
            filter_input_events.unwrap().send(InputFilterEvent(InputFilter::Block(vec![P(T).into(), R(T).into()])));
        }

        app.update();

        let mut input_stream = InputStream::default();
        rdev::simulate(&EventType::KeyPress(rdev::Key::KeyT)).unwrap();
        rdev::simulate(&EventType::KeyRelease(rdev::Key::KeyT)).unwrap();

        app.update();
        app.update();

        // check input inside app
        {
            let input_events = app.world.get_resource_mut::<Events<InputEvent>>();
            assert!(input_events.is_some());
            let input_events = input_events.unwrap();
            let mut input_events_reader = input_events.get_reader();
            let input_event = input_events_reader.read(&input_events).next();
            assert!(input_event.is_some());
            assert_eq!(input_event.unwrap(), &P(T).into());
        }

        // check input outside app
        let input_event = input_stream.next();
        assert!(input_event.is_some());
        assert_eq!(input_event.unwrap(), EventType::KeyRelease(rdev::Key::KeyT));
        let input_event = input_stream.next();
        assert!(input_event.is_none());
    }

    // #[ignore = "unsupported on ci"]
    #[test]
    fn unblock_keyboard_key_input_test() {
        let mut app = App::new();
        app.add_plugins(GlobalInputPlugin);

        {
            let filter_input_events = app.world.get_resource_mut::<Events<InputFilterEvent>>();
            assert!(filter_input_events.is_some());
            filter_input_events.unwrap().send(InputFilterEvent(InputFilter::Block(vec![P(Left).into()])));
        }

        app.update();

        {
            let filter_input_events = app.world.get_resource_mut::<Events<InputFilterEvent>>();
            assert!(filter_input_events.is_some());
            filter_input_events.unwrap().send(InputFilterEvent(InputFilter::Unblock(vec![P(T).into()])));
        }

        app.update();

        let mut input_stream = InputStream::default();
        rdev::simulate(&EventType::KeyPress(rdev::Key::KeyT)).unwrap();
        rdev::simulate(&EventType::KeyRelease(rdev::Key::KeyT)).unwrap();

        app.update();

        let input_event = input_stream.next();
        assert!(input_event.is_some());
        assert_eq!(input_event.unwrap(), EventType::KeyPress(rdev::Key::KeyT))
    }
}
