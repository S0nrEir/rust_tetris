pub mod StateEnum;

use std::fmt::Debug;
use StateEnum::ProcedureEnum;

///一个简易的状态 / 
/// A simple state
pub trait TState : Debug {
        
    fn on_enter(&self);
    fn on_update(&self);
    fn on_leave(&self);
    fn get_state(&self) -> ProcedureEnum;
}