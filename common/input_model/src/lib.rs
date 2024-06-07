pub use input::DeviceInput;
pub use input::Input;
pub use input::Modified;
pub use input::Modifier;
pub use input::Sequence;
pub use keyboard::Key;
pub use keyboard::KeyboardInput;
pub use mouse::Button;
pub use mouse::ButtonInput;
pub use mouse::MouseInput;
pub use mouse::Position;
pub use mouse::Rotation;

mod keyboard;
mod mouse;
mod input;

pub mod filter;
pub mod views;


#[cfg(test)]
mod test {
    use crate::Button::Left;
    use crate::Key::{ControlLeft, ShiftLeft, T};
    use crate::Sequence;
    use crate::views::definition::{P, R};

    // todo introduce marco for sequence definition
    #[test]
    fn definitions_test() {
        let definition: [Vec<Sequence>; 8] = [
            P(T).into(),
            R(T).into(),
            // todo think about short definition for key press and release
            vec![
                P(T).into(),
                R(T).into(),
            ],
            vec![
                P(T).into(),
                P(Left).into(),
            ],
            vec![
                R(T).into(),
                R(Left).into(),
            ],
            vec![
                P(T).into(),
                R(Left).into(),
            ],
            vec![
                (ControlLeft, P(T).into()).into()
            ],
            vec![
                (vec![ControlLeft, ShiftLeft], P(T).into()).into()
            ]
        ];
        assert!(!definition.is_empty())
    }
}
