use bevy::prelude::Event;

#[derive(Event, Debug, Clone)]
pub struct FocusWindowLeft;

#[derive(Event, Debug, Clone)]
pub struct FocusWindowRight;

#[derive(Event, Debug, Clone)]
pub struct FocusWindowUp;

#[derive(Event, Debug, Clone)]
pub struct FocusWindowDown;

#[derive(Event, Debug, Clone)]
pub struct MoveWindowLeft;

#[derive(Event, Debug, Clone)]
pub struct MoveWindowRight;

#[derive(Event, Debug, Clone)]
pub struct MoveWindowUp;

#[derive(Event, Debug, Clone)]
pub struct MoveWindowDown;

#[derive(Event, Debug, Clone)]
pub struct StackWindowLeft;

#[derive(Event, Debug, Clone)]
pub struct StackWindowRight;

#[derive(Event, Debug, Clone)]
pub struct StackWindowUp;

#[derive(Event, Debug, Clone)]
pub struct StackWindowDown;

#[derive(Event, Debug, Clone)]
pub struct UnstackWindow;

#[derive(Event, Debug, Clone)]
pub struct ToggleMaximize;

#[derive(Event, Debug, Clone)]
pub struct ToggleMonocle;

#[derive(Event, Debug, Clone)]
pub struct ToggleFloat;

#[derive(Event, Debug, Clone)]
pub struct MinimizeWindow;

#[derive(Event, Debug, Clone)]
pub struct CloseWindow;