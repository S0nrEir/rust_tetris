use std::fmt::Debug;
use ggez::input::keyboard::KeyCode;
use crate::define::enum_define::ProcedureEnum;
// use crate::runtime::controller::Controller;

///一个简易的状态 / 
/// A simple state
pub trait TState : Debug {
        
    fn on_enter(&self);
    fn on_update(&mut self,key_code: KeyCode);
    fn on_leave(&self);
    fn get_state(&self) -> ProcedureEnum;
}