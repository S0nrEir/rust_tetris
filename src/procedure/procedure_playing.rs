use crate::t_state::TState;
use crate::t_state::StateEnum::ProcedureEnum;

///游玩状态
/// playing state
#[derive(Debug)]
pub  struct ProcedurePlaying{
}

impl TState for ProcedurePlaying{
    fn on_enter(&self) {
        println!("ProcedurePlaying enter");
    }

    fn on_update(&self) {
        println!("ProcedurePlaying update");
    }

    fn on_leave(&self) {
        println!("ProcedurePlaying exit");
    }

    fn get_state(&self) -> ProcedureEnum {
        return ProcedureEnum::Playing;
    }
}

impl ProcedurePlaying {
    //--------new--------
    pub fn new() -> Self{
        return ProcedurePlaying{};
    }
}