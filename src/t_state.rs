use std::fmt::Debug;
use ggez::input::keyboard::KeyCode;
use crate::define::enum_define::ProcedureEnum;
use crate::runtime::procedure::t_procedure_param::ProcedureParam;

///一个简易的状态 / 
/// A simple state
pub trait TState : Debug {
        
    fn on_enter(&self,param:Option<Box<dyn ProcedureParam>>);
    fn on_update(&mut self,key_code: KeyCode);
    fn on_leave(&self,param:Option<Box<dyn ProcedureParam>>);
    fn get_state(&self) -> ProcedureEnum;
}