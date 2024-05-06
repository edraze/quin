use bevy::prelude::Event;

#[derive(Event, Debug, Clone)]
pub struct ActivateKeyboardToMouse;

#[derive(Event, Debug, Clone)]
pub struct DeactivateKeyboardToMouse;

#[derive(Event, Debug, Clone)]
pub(crate) struct MoveMouseRelativelyUp;

#[derive(Event, Debug, Clone)]
pub(crate) struct MoveMouseRelativelyDown;

#[derive(Event, Debug, Clone)]
pub(crate) struct MoveMouseRelativelyLeft;

#[derive(Event, Debug, Clone)]
pub(crate) struct MoveMouseRelativelyRight;

#[derive(Event, Debug, Clone)]
pub(crate) struct ScrollUp;

#[derive(Event, Debug, Clone)]
pub(crate) struct ScrollDown;

#[derive(Event, Debug, Clone)]
pub(crate) struct ScrollLeft;

#[derive(Event, Debug, Clone)]
pub(crate) struct ScrollRight;

#[derive(Event, Debug, Clone)]
pub(crate) struct MouseLeftButtonClick;

#[derive(Event, Debug, Clone)]
pub(crate) struct MouseMiddleButtonClick;

#[derive(Event, Debug, Clone)]
pub(crate) struct MouseRightButtonClick;

#[derive(Event, Debug, Clone)]
pub(crate) struct DragAndDropStart;

#[derive(Event, Debug, Clone)]
pub(crate) struct DragAndDropEnd;
