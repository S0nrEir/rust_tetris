use crate::t_state::TState;

///主UI流程 / 
/// main UI procedure
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
}

impl ProcedureMainUI {
    
    ///constructor
    pub fn new() -> Self {
        return ProcedureMainUI{};
    }
}