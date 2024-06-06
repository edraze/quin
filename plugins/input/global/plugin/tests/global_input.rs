// Avoid user input during tests in this module because it can affect results
#[cfg(test)]
mod test {
    use bevy::app::App;
    use bevy::prelude::Events;
    use rdev::EventType;

    use global_input_api::filter::InputFilterEvent;
    use global_input_api::input::InputEvent;
    use global_input_api::input_model::Button::Left;
    use global_input_api::input_model::filter::InputFilter;
    use global_input_api::input_model::Key::{ControlLeft, ShiftLeft, T};
    use global_input_api::input_model::views::definition::{P, R};
    use global_input_plugin::GlobalInputPlugin;
    use test_utils::InputStream;

    #[ignore = "unsupported on ci"]
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
        assert_eq!(input_event.unwrap(), &InputEvent(P(T).into()));
    }

    #[ignore = "unsupported on ci"]
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
        assert_eq!(input_event.unwrap(), &InputEvent(P(Left).into()));
    }

    #[ignore = "unsupported on ci & don't work"]
    #[test]
    fn listen_modified_input_test() {
        let mut app = App::new();
        app.add_plugins(GlobalInputPlugin);

        app.update();

        rdev::simulate(&EventType::KeyPress(rdev::Key::ControlLeft)).unwrap();
        app.update();
        rdev::simulate(&EventType::KeyPress(rdev::Key::KeyT)).unwrap();
        app.update();
        rdev::simulate(&EventType::KeyRelease(rdev::Key::KeyT)).unwrap();
        app.update();
        rdev::simulate(&EventType::KeyRelease(rdev::Key::ControlLeft)).unwrap();
        app.update();

        let input_events = app.world.get_resource_mut::<Events<InputEvent>>();
        assert!(input_events.is_some());
        let input_events = input_events.unwrap();
        let mut input_events_reader = input_events.get_reader();
        let mut input_events_iter = input_events_reader.read(&input_events);
        let input_event = input_events_iter.next();
        assert!(input_event.is_some());
        assert_eq!(input_event.unwrap(), &InputEvent(P(ControlLeft).into()));
        let input_event = input_events_iter.next();
        assert!(input_event.is_some());
        assert_eq!(input_event.unwrap(), &InputEvent((ControlLeft, P(T).into()).into()));
        let input_event = input_events_iter.next();
        assert!(input_event.is_some());
        assert_eq!(input_event.unwrap(), &InputEvent((ControlLeft, R(T).into()).into()));
        let input_event = input_events_iter.next();
        assert!(input_event.is_some());
        assert_eq!(input_event.unwrap(), &InputEvent(R(ControlLeft).into()));
        let input_event = input_events_iter.next();
        assert!(input_event.is_none());
    }

    #[ignore = "unsupported on ci & don't work"]
    #[test]
    fn listen_multi_modified_input_test() {
        let mut app = App::new();
        app.add_plugins(GlobalInputPlugin);

        app.update();

        rdev::simulate(&EventType::KeyPress(rdev::Key::ControlLeft)).unwrap();
        rdev::simulate(&EventType::KeyPress(rdev::Key::ShiftLeft)).unwrap();
        rdev::simulate(&EventType::KeyPress(rdev::Key::KeyT)).unwrap();
        rdev::simulate(&EventType::KeyRelease(rdev::Key::KeyT)).unwrap();
        rdev::simulate(&EventType::KeyRelease(rdev::Key::ShiftLeft)).unwrap();
        rdev::simulate(&EventType::KeyRelease(rdev::Key::ControlLeft)).unwrap();

        app.update();

        let input_events = app.world.get_resource_mut::<Events<InputEvent>>();
        assert!(input_events.is_some());
        let input_events = input_events.unwrap();
        let mut input_events_reader = input_events.get_reader();
        let mut input_events_iter = input_events_reader.read(&input_events);
        let input_event = input_events_iter.next();
        assert!(input_event.is_some());
        assert_eq!(input_event.unwrap(), &InputEvent(P(ControlLeft).into()));
        let input_event = input_events_iter.next();
        assert!(input_event.is_some());
        assert_eq!(input_event.unwrap(), &InputEvent((ControlLeft, P(ShiftLeft).into()).into()));
        let input_event = input_events_iter.next();
        assert!(input_event.is_some());
        assert_eq!(input_event.unwrap(), &InputEvent((vec![ShiftLeft, ControlLeft], P(T).into()).into()));
        let input_event = input_events_iter.next();
        assert!(input_event.is_some());
        assert_eq!(input_event.unwrap(), &InputEvent((vec![ShiftLeft, ControlLeft], R(T).into()).into()));
        let input_event = input_events_iter.next();
        assert!(input_event.is_some());
        assert_eq!(input_event.unwrap(), &InputEvent((ControlLeft, R(ShiftLeft).into()).into()));
        let input_event = input_events_iter.next();
        assert!(input_event.is_some());
        assert_eq!(input_event.unwrap(), &InputEvent(R(ControlLeft).into()));
        let input_event = input_events_iter.next();
        assert!(input_event.is_none());
    }

    #[ignore = "unsupported on ci"]
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
            assert_eq!(input_event.unwrap(), &InputEvent(P(T).into()));
        }

        // check input outside app
        let input_event = input_stream.next();
        assert!(input_event.is_some());
        assert_eq!(input_event.unwrap(), EventType::KeyRelease(rdev::Key::KeyT));
        let input_event = input_stream.next();
        assert!(input_event.is_none());
    }

    #[ignore = "unsupported on ci"]
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
