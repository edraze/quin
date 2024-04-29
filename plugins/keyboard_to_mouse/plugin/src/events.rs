use bevy::prelude::Event;

#[derive(Event, Debug, Clone)]
pub struct ActivateKeyboardToMouse;

#[derive(Event, Debug, Clone)]
pub struct DeactivateKeyboardToMouse;

#[derive(Event, Debug, Clone)]
pub struct MoveMouseRelativelyUp;

#[derive(Event, Debug, Clone)]
pub struct MoveMouseRelativelyDown;

#[derive(Event, Debug, Clone)]
pub struct MoveMouseRelativelyLeft;

#[derive(Event, Debug, Clone)]
pub struct MoveMouseRelativelyRight;

#[derive(Event, Debug, Clone)]
pub struct ScrollUp;

#[derive(Event, Debug, Clone)]
pub struct ScrollDown;

#[derive(Event, Debug, Clone)]
pub struct ScrollLeft;

#[derive(Event, Debug, Clone)]
pub struct ScrollRight;

#[derive(Event, Debug, Clone)]
pub struct MouseLeftButtonClick;

#[derive(Event, Debug, Clone)]
pub struct MouseMiddleButtonClick;

#[derive(Event, Debug, Clone)]
pub struct MouseRightButtonClick;

#[derive(Event, Debug, Clone)]
pub struct DragAndDropStart;

#[derive(Event, Debug, Clone)]
pub struct DragAndDropEnd;
