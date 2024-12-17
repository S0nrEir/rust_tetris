use std::any::Any;
use std::fmt::Debug;

/// 流程参数类 Trait / ProcedureParam Trait
pub trait ProcedureParam : Debug+Any{
    fn as_any_mut(&mut self) -> &mut dyn Any;
}