use std::any::Any;
use std::fmt::Debug;
use ggez::input::keyboard::KeyCode;
use crate::define::enum_define::ProcedureEnum;
use crate::runtime::procedure::t_procedure_param::ProcedureParam;
use crate::t_updatable::Drawable;

///一个简易的状态 / 
/// A simple state
pub trait TState : Debug+Drawable {

    fn on_enter(&mut self,param:Box<dyn ProcedureParam>);
    // fn on_enter(&mut self,param:Option<Box<dyn ProcedureParam>>);
    fn on_update(&mut self,key_code: KeyCode);
    fn on_leave(&self,param:Option<Box<dyn ProcedureParam>>);
    fn get_state(&self) -> ProcedureEnum;
}