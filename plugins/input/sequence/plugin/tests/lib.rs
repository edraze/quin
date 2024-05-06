#[cfg(test)]
mod test {
    use bevy::prelude::{App, Event, Events};

    use global_input_api::input::InputEvent;
    use global_input_api::input_model::keyboard::{Key, KeyEvent};
    use input_sequence_api::{ResetSequenceBuffer, Sequence, SequencesToEvent};
    use input_sequence_plugin::{InputSequencePlugin, listen_sequences};

    #[derive(Event, Clone)]
    struct TestEvent;

    #[test]
    fn subscribe_to_single_key_sequence() {
        let test_sequence = vec![Sequence::new(vec![InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyT))])];
        let test_binding: SequencesToEvent<_> = (test_sequence, TestEvent).into();

        let mut app = App::new();
        app.add_plugins(InputSequencePlugin);
        listen_sequences(&mut app, test_binding);

        {
            let sequence_event = app.world.get_resource_mut::<Events<TestEvent>>();
            assert!(sequence_event.is_some());
            let sequence_event = sequence_event.unwrap();
            let is_sequence_events_exists = sequence_event.get_reader().read(&sequence_event).next().is_some();
            assert!(!is_sequence_events_exists);
        }

        {
            let input_events = app.world.get_resource_mut::<Events<InputEvent>>();
            assert!(input_events.is_some());
            input_events.unwrap().send(InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyF)));
        }

        app.update();
        app.update();

        {
            let sequence_event = app.world.get_resource_mut::<Events<TestEvent>>().unwrap();
            let is_sequence_events_exists = sequence_event.get_reader().read(&sequence_event).next().is_some();
            assert!(!is_sequence_events_exists);
        }

        {
            let input_events = app.world.get_resource_mut::<Events<InputEvent>>();
            assert!(input_events.is_some());
            input_events.unwrap().send(InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyT)));
        }

        app.update();
        app.update(); // todo why it needs two updates here ?

        {
            let sequence_event = app.world.get_resource_mut::<Events<TestEvent>>().unwrap();
            let is_sequence_events_exists = sequence_event.get_reader().read(&sequence_event).next().is_some();
            assert!(is_sequence_events_exists);
        }
    }

    #[test]
    fn subscribe_to_word_sequence() {
        let test_sequence = vec![Sequence::new(vec![
            InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyT)),
            InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyE)),
            InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyS)),
            InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyT))]),
        ];
        let test_binding: SequencesToEvent<_> = (test_sequence, TestEvent).into();

        let mut app = App::new();
        app.add_plugins(InputSequencePlugin);
        listen_sequences(&mut app, test_binding);

        {
            let sequence_event = app.world.get_resource_mut::<Events<TestEvent>>();
            assert!(sequence_event.is_some());
            let sequence_event = sequence_event.unwrap();
            let is_sequence_events_exists = sequence_event.get_reader().read(&sequence_event).next().is_some();
            assert!(!is_sequence_events_exists);
        }

        {
            let input_events = app.world.get_resource_mut::<Events<InputEvent>>();
            assert!(input_events.is_some());
            let mut input_events = input_events.unwrap();
            input_events.send(InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyT)));
            input_events.send(InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyE)));
            input_events.send(InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyS)));
            input_events.send(InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyS)));
        }

        app.update();
        app.update();

        {
            let sequence_event = app.world.get_resource_mut::<Events<TestEvent>>().unwrap();
            let is_sequence_events_exists = sequence_event.get_reader().read(&sequence_event).next().is_some();
            assert!(!is_sequence_events_exists);
        }

        {
            let input_events = app.world.get_resource_mut::<Events<InputEvent>>();
            assert!(input_events.is_some());
            let mut input_events = input_events.unwrap();
            input_events.send(InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyT)));
            input_events.send(InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyE)));
            input_events.send(InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyS)));
            input_events.send(InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyT)));
        }

        app.update();
        app.update(); // todo why it needs two updates here ?

        {
            let sequence_event = app.world.get_resource_mut::<Events<TestEvent>>().unwrap();
            let is_sequence_events_exists = sequence_event.get_reader().read(&sequence_event).next().is_some();
            assert!(is_sequence_events_exists);
        }
    }

    #[test]
    fn reset_sequence_buffer() {
        let test_sequence = vec![Sequence::new(vec![
            InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyT)),
            InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyE)),
            InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyS)),
            InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyT))]),
        ];
        let test_binding: SequencesToEvent<_> = (test_sequence, TestEvent).into();

        let mut app = App::new();
        app.add_plugins(InputSequencePlugin);
        listen_sequences(&mut app, test_binding);

        {
            let sequence_event = app.world.get_resource_mut::<Events<TestEvent>>();
            assert!(sequence_event.is_some());
            let sequence_event = sequence_event.unwrap();
            let is_sequence_events_exists = sequence_event.get_reader().read(&sequence_event).next().is_some();
            assert!(!is_sequence_events_exists);
        }

        {
            let input_events = app.world.get_resource_mut::<Events<InputEvent>>();
            assert!(input_events.is_some());
            let mut input_events = input_events.unwrap();
            input_events.send(InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyT)));
            input_events.send(InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyE)));
        }

        {
            let reset_buffer_events = app.world.get_resource_mut::<Events<ResetSequenceBuffer>>();
            assert!(reset_buffer_events.is_some());
            let mut reset_buffer_events = reset_buffer_events.unwrap();
            reset_buffer_events.send(ResetSequenceBuffer);
        }

        {
            let input_events = app.world.get_resource_mut::<Events<InputEvent>>();
            assert!(input_events.is_some());
            let mut input_events = input_events.unwrap();
            input_events.send(InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyS)));
            input_events.send(InputEvent::Keyboard(KeyEvent::Pressed(Key::KeyT)));
        }

        app.update();
        app.update(); // todo why it needs two updates here ?

        {
            let sequence_event = app.world.get_resource_mut::<Events<TestEvent>>().unwrap();
            let is_sequence_events_exists = sequence_event.get_reader().read(&sequence_event).next().is_some();
            assert!(!is_sequence_events_exists);
        }
    }
}
