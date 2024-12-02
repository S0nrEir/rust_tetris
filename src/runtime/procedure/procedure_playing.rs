use ggez::input::keyboard::KeyCode;
use crate::t_state::TState;
use crate::define::enum_define::ProcedureEnum;
use crate::runtime::procedure::t_procedure_param::ProcedureParam;
// use crate::runtime::controller::Controller;

///游玩状态
/// playing state
#[derive(Debug,Clone)]
pub  struct ProcedurePlaying{
}

impl TState for ProcedurePlaying{
    fn on_enter(&self,_param:Option<Box<dyn ProcedureParam>>) {
        println!("ProcedurePlaying enter");
    }

    fn on_update(&mut self,key_code: KeyCode) {
        println!("ProcedurePlaying update");
    }

    fn on_leave(&self,_param:Option<Box<dyn ProcedureParam>>) {
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

#[derive(Debug)]
pub struct ProcedurePlayingParam{
}

impl ProcedurePlayingParam {
    pub fn new() -> Self{
        return ProcedurePlayingParam{};
    }
}

impl ProcedureParam for ProcedurePlayingParam{
    
}