use crate::t_state::TState;
use crate::t_state::StateEnum::ProcedureEnum;

///主UI流程 / 
/// main UI procedure
#[derive(Debug)]
pub struct ProcedureMainUI{
}

/// 实现状态机接口 /
/// implement state machine interface
impl TState for ProcedureMainUI{

    fn on_enter(&self) {
        println!("ProcedureMainUI enter");
    }

    fn on_update(&self) {
        println!("ProcedureMainUI update");
    }

    fn on_leave(&self) {
        println!("ProcedureMainUI exit");
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