use crate::core::{Handler, Label, Labels, State};

const GM_ACTIVATE: &str = "gm_activate";

#[derive(Default)]
pub struct GridModeHandler {}

impl Label for GridModeHandler {
    fn get_labels(&self) -> Vec<Labels> {
        todo!()
    }
}

impl Handler for GridModeHandler {
    fn execute(&mut self, label: &Labels, state: &mut State) {
        if let Labels::Keys(target_label) = label {
            if target_label.eq(GM_ACTIVATE) {
                println!("GM GM_ACTIVATE")
            }
        }
    }
}
