use crate::t_state::TState;
use crate::define::enum_define::ProcedureEnum;
use crate::runtime::controller::Controller;

///游玩状态
/// playing state
#[derive(Debug)]
pub  struct ProcedurePlaying{
}

impl TState for ProcedurePlaying{
    fn on_enter(&self,controller:&mut Controller) {
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