use crate::{Button, ButtonInput, DeviceInput, Input, Key, KeyboardInput, Modified, MouseInput};

pub struct P<KB>(pub KB);

impl From<P<Key>> for Input {
    fn from(value: P<Key>) -> Self {
        Input::Device(DeviceInput::Keyboard(KeyboardInput::Pressed(value.0)))
    }
}

impl From<P<Button>> for Input {
    fn from(value: P<Button>) -> Self {
        Input::Device(DeviceInput::Mouse(MouseInput::Button(ButtonInput::Pressed(value.0))))
    }
}

pub struct R<KB>(pub KB);

impl From<R<Key>> for Input {
    fn from(value: R<Key>) -> Self {
        Input::Device(DeviceInput::Keyboard(KeyboardInput::Released(value.0)))
    }
}

impl From<R<Button>> for Input {
    fn from(value: R<Button>) -> Self {
        Input::Device(DeviceInput::Mouse(MouseInput::Button(ButtonInput::Released(value.0))))
    }
}

impl From<(Key, Input)> for Modified {
    fn from(value: (Key, Input)) -> Self {
        Modified {
            modifiers: value.0.into(),
            input: Box::new(value.1),
        }
    }
}

impl From<(&[Key], Input)> for Modified {
    fn from(value: (&[Key], Input)) -> Self {
        let modifiers = value.0.iter()
            .map(|key| (*key).into())
            .collect();
        Modified {
            modifiers,
            input: Box::new(value.1),
        }
    }
}

impl From<(Button, Input)> for Modified {
    fn from(value: (Button, Input)) -> Self {
        Modified {
            modifiers: value.0.into(),
            input: Box::new(value.1),
        }
    }
}

impl From<(&[Button], Input)> for Modified {
    fn from(value: (&[Button], Input)) -> Self {
        let modifiers = value.0.iter()
            .map(|key| (*key).into())
            .collect();
        Modified {
            modifiers,
            input: Box::new(value.1),
        }
    }
}

impl From<(Key, Input)> for Input {
    fn from(value: (Key, Input)) -> Self {
        Input::Modified(Modified {
            modifiers: value.0.into(),
            input: Box::new(value.1),
        })
    }
}

impl From<(&[Key], Input)> for Input {
    fn from(value: (&[Key], Input)) -> Self {
        let modifiers = value.0.iter()
            .map(|key| (*key).into())
            .collect();
        Input::Modified(Modified {
            modifiers,
            input: Box::new(value.1),
        })
    }
}

impl From<(Button, Input)> for Input {
    fn from(value: (Button, Input)) -> Self {
        Input::Modified(Modified {
            modifiers: value.0.into(),
            input: Box::new(value.1),
        })
    }
}

impl From<(&[Button], Input)> for Input {
    fn from(value: (&[Button], Input)) -> Self {
        let modifiers = value.0.iter()
            .map(|key| (*key).into())
            .collect();
        Input::Modified(Modified {
            modifiers,
            input: Box::new(value.1),
        })
    }
}