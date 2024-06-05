use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use config_loader::Config;
use global_input_api::input_model::definition::P;
use global_input_api::input_model::Key::{AltLeft, DownArrow, Escape, F, J, K, L, LeftArrow, M, O, RightArrow, SemiColon, ShiftLeft, T, UpArrow, X};
use input_sequence_api::Sequence;

use crate::TILING_WINDOW_MANAGER_PLUGIN_NAME;

#[derive(Resource, Serialize, Deserialize, Debug, Clone, Default)]
pub struct TilingWindowManagerConfig {
    pub key_bindings: TilingWindowManagerBindings,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TilingWindowManagerBindings {
    pub focus_left: Vec<Sequence>,
    pub focus_right: Vec<Sequence>,
    pub focus_up: Vec<Sequence>,
    pub focus_down: Vec<Sequence>,
    pub move_left: Vec<Sequence>,
    pub move_right: Vec<Sequence>,
    pub move_up: Vec<Sequence>,
    pub move_down: Vec<Sequence>,
    pub stack_left: Vec<Sequence>,
    pub stack_right: Vec<Sequence>,
    pub stack_up: Vec<Sequence>,
    pub stack_down: Vec<Sequence>,
    pub unstack: Vec<Sequence>,
    pub toggle_maximize: Vec<Sequence>,
    pub toggle_monocle: Vec<Sequence>,
    pub toggle_float: Vec<Sequence>,
    pub minimize: Vec<Sequence>,
    pub close: Vec<Sequence>,
}

impl Default for TilingWindowManagerBindings {
    fn default() -> Self {
        Self {
            focus_left: vec![Sequence::new(vec![
                P(AltLeft).into(),
                P(J).into()])],
            focus_right: vec![Sequence::new(vec![
                P(AltLeft).into(),
                P(SemiColon).into()])],
            focus_up: vec![Sequence::new(vec![
                P(AltLeft).into(),
                P(L).into()])],
            focus_down: vec![Sequence::new(vec![
                P(AltLeft).into(),
                P(K).into()])],
            move_left: vec![Sequence::new(vec![
                P(AltLeft).into(),
                P(ShiftLeft).into(),
                P(J).into()])],
            move_right: vec![Sequence::new(vec![
                P(AltLeft).into(),
                P(ShiftLeft).into(),
                P(SemiColon).into()])],
            move_up: vec![Sequence::new(vec![
                P(AltLeft).into(),
                P(ShiftLeft).into(),
                P(L).into()])],
            move_down: vec![Sequence::new(vec![
                P(AltLeft).into(),
                P(ShiftLeft).into(),
                P(K).into()])],
            stack_left: vec![Sequence::new(vec![
                P(AltLeft).into(),
                P(LeftArrow).into()])],
            stack_right: vec![Sequence::new(vec![
                P(AltLeft).into(),
                P(RightArrow).into()])],
            stack_up: vec![Sequence::new(vec![
                P(AltLeft).into(),
                P(UpArrow).into()])],
            stack_down: vec![Sequence::new(vec![
                P(AltLeft).into(),
                P(DownArrow).into()])],
            unstack: vec![Sequence::new(vec![
                P(AltLeft).into(),
                P(Escape).into()])],
            toggle_maximize: vec![Sequence::new(vec![
                P(AltLeft).into(),
                P(O).into()])],
            toggle_monocle: vec![Sequence::new(vec![
                P(AltLeft).into(),
                P(T).into()])],
            toggle_float: vec![Sequence::new(vec![
                P(AltLeft).into(),
                P(F).into()])],
            minimize: vec![Sequence::new(vec![
                P(AltLeft).into(),
                P(M).into()])],
            close: vec![Sequence::new(vec![
                P(AltLeft).into(),
                P(X).into()])],
        }
    }
}

impl Config for TilingWindowManagerConfig {
    fn name() -> String {
        TILING_WINDOW_MANAGER_PLUGIN_NAME.to_string()
    }
}
