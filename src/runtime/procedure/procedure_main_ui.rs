use ggez::{Context, GameError};
use ggez::event::EventHandler;
use crate::t_state::TState;
use crate::define::enum_define::ProcedureEnum;

///主UI流程 / 
/// main UI procedure
#[derive(Debug,Clone)]
pub struct ProcedureMainUI{
}

/// 实现状态机接口 /
/// implement state machine interface
impl TState for ProcedureMainUI{

    fn on_enter(&self) {
    }

    fn on_update(&mut self,_key_code: ggez::input::keyboard::KeyCode) {
        println!("ProcedureMainUI update");
    }

    fn on_leave(&self) {
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