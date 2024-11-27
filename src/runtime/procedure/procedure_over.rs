use crate::t_state::TState;
use crate::define::enum_define::ProcedureEnum;
use crate::runtime::controller::Controller;

///游戏结束，结算，重开
/// game over, settlement, restart
#[derive(Debug)]
pub  struct ProcedureOver{
}

impl TState for ProcedureOver{
    
    //--------impl--------
    fn on_enter(&self,controller:&mut Controller) {
        println!("ProcedureOver enter");
    }

    fn on_update(&self) {
        println!("ProcedureOver update");
    }

    fn on_leave(&self) {
    }
    
    fn get_state(&self) -> ProcedureEnum {
        return ProcedureEnum::Over;
    }
}

impl ProcedureOver {
    //--------new--------
    pub fn new() -> Self{
        return ProcedureOver{};
    }
    
}