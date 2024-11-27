use ggez::{Context, GameError};
use ggez::event::EventHandler;
use crate::t_state::TState;
use crate::define::enum_define::ProcedureEnum;
use crate::runtime::controller::Controller;

///主UI流程 / 
/// main UI procedure
#[derive(Debug)]
pub struct ProcedureMainUI{
}

/// 实现状态机接口 /
/// implement state machine interface
impl TState for ProcedureMainUI{

    fn on_enter(&self,controller:&mut Controller) {
        controller.set_input_mode(ProcedureEnum::MainUI);
    }

    fn on_update(&self) {
        println!("ProcedureMainUI update");
    }

    fn on_leave(&self,controller:&mut Controller) {
        println!("ProcedureMainUI exit");
        controller.clear_input();
    }
    
    fn get_state(&self) -> ProcedureEnum {
        return ProcedureEnum::MainUI;
    }
}

impl ProcedureMainUI {
    ///constructor
    pub fn new() -> Self {
        return ProcedureMainUI{};
    }
}