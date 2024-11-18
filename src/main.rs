mod examples;
mod procedure;
mod t_state;
mod constant;
mod entry;
mod tools;

use std::rc::Rc;
use crate::procedure::procedure_main_ui;
use crate::procedure::procedure_playing;
use crate::procedure::procedure_over;
use crate::procedure::ProcedureComponent;
use crate::t_state::StateEnum;

fn main() {
    
    let mut procedure_comp = ProcedureComponent::new(
        0,
        vec![
            Some(Rc::new(procedure_main_ui::ProcedureMainUI::new())),
            Some(Rc::new(procedure_playing::ProcedurePlaying::new())),
            Some(Rc::new(procedure_over::ProcedureOver::new()))]
    );
    
    procedure_comp.switch(StateEnum::ProcedureEnum::Playing);
    procedure_comp.switch(StateEnum::ProcedureEnum::Over);
    procedure_comp.switch(StateEnum::ProcedureEnum::MainUI);
    
    // tools::Logger::log(0u8,"test log info",LogLevelEnum::Info);
    // tools::Logger::log(1u8,"test log warning",LogLevelEnum::Warning);
    // tools::Logger::log(2u8,"test log error",LogLevelEnum::Error);
    // tools::Logger::log(3u8,"test log fatal",LogLevelEnum::Fatal);
    
    // examples::example_02_enter();
}
