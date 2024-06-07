use std::vec;

use crate::{ButtonInput, DeviceInput, Input, KeyboardInput, Modified, Modifier, MouseInput, Sequence};
use crate::input::AsModifier;

pub struct P<M>(pub M)
    where M: AsModifier;

impl<M> From<P<M>> for Input
    where M: AsModifier {
    fn from(value: P<M>) -> Self {
        match value.0.as_modifier() {
            Modifier::Key(key) => Input::Device(DeviceInput::Keyboard(KeyboardInput::Pressed(key))),
            Modifier::Button(button) => Input::Device(DeviceInput::Mouse(MouseInput::Button(ButtonInput::Pressed(button)))),
        }
    }
}

impl<M> From<P<M>> for Vec<Input>
    where M: AsModifier {
    fn from(value: P<M>) -> Self {
        let input = value.into();
        vec![input]
    }
}

impl<M> From<P<M>> for Sequence
    where M: AsModifier {
    fn from(value: P<M>) -> Self {
        Sequence::new(value.into())
    }
}

impl<M> From<P<M>> for Vec<Sequence>
    where M: AsModifier {
    fn from(value: P<M>) -> Self {
        vec![Sequence::new(value.into())]
    }
}

pub struct R<M>(pub M)
    where M: AsModifier;

impl<M> From<R<M>> for Input
    where M: AsModifier {
    fn from(value: R<M>) -> Self {
        match value.0.as_modifier() {
            Modifier::Key(key) => Input::Device(DeviceInput::Keyboard(KeyboardInput::Released(key))),
            Modifier::Button(button) => Input::Device(DeviceInput::Mouse(MouseInput::Button(ButtonInput::Released(button)))),
        }
    }
}

impl<M> From<R<M>> for Vec<Input>
    where M: AsModifier {
    fn from(value: R<M>) -> Self {
        let input = value.into();
        vec![input]
    }
}

impl<M> From<R<M>> for Sequence
    where M: AsModifier {
    fn from(value: R<M>) -> Self {
        Sequence::new(value.into())
    }
}

impl<M> From<R<M>> for Vec<Sequence>
    where M: AsModifier {
    fn from(value: R<M>) -> Self {
        vec![Sequence::new(value.into())]
    }
}

impl<M> From<(M, Input)> for Modified
    where M: AsModifier {
    fn from(value: (M, Input)) -> Self {
        let modifiers = vec![value.0.as_modifier()];
        Modified {
            modifiers,
            input: Box::new(value.1),
        }
    }
}

impl<M> From<(Vec<M>, Input)> for Modified
    where M: AsModifier {
    fn from(value: (Vec<M>, Input)) -> Self {
        let modifiers = value.0.iter()
            .map(|modifier| modifier.as_modifier())
            .collect();
        Modified {
            modifiers,
            input: Box::new(value.1),
        }
    }
}

impl<M> From<(M, Input)> for Input
    where M: AsModifier {
    fn from(value: (M, Input)) -> Self {
        Input::Modified(value.into())
    }
}

impl<M> From<(Vec<M>, Input)> for Input
    where M: AsModifier {
    fn from(value: (Vec<M>, Input)) -> Self {
        Input::Modified(value.into())
    }
}

impl<M> From<(M, Input)> for Sequence
    where M: AsModifier {
    fn from(value: (M, Input)) -> Self {
        Sequence::new(vec![value.into()])
    }
}

impl<M> From<(Vec<M>, Input)> for Sequence
    where M: AsModifier {
    fn from(value: (Vec<M>, Input)) -> Self {
        Sequence::new(vec![value.into()])
    }
}
