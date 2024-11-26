pub mod procedure;
pub mod event;
pub mod input;
pub mod controller;
pub(crate) mod app_components;

// mod procedure;
// mod event;
// mod input;
// mod controller;
// 
// ///运行时模块，提供运行时的逻辑和功能 / Runtime module, providing runtime logic and function
// pub mod Runtime{
//     use std::rc::Rc;
//     use crate::t_updatable::Updatable;
//     use crate::runtime::*;
//     use crate::runtime::procedure::{procedure_main_ui, procedure_over, procedure_playing};
// 
//     #[derive(Debug)]
//     pub struct Runtime{
//         _event_comp : event::EventComponent,
//         _input_comp : input::InputComponent,
//         _procedure_comp : procedure::ProcedureComponent,
//     }
// 
//     impl Updatable for Runtime{
//         fn on_update(&self) {
//             self._input_comp.on_update();
//             self._event_comp.on_update();
//             self._procedure_comp.on_update();
//         }
//     }
//     
//     impl Runtime{
//         
//         /// 初始化并创建一个运行时 / initialize and create a runtime
//         pub fn new() -> Self{
//             return Runtime{
//                 _event_comp : event::EventComponent::new(),
//                 _input_comp : input::InputComponent::new(),
//                 _procedure_comp : procedure::ProcedureComponent::new(
//                     0,
//                     vec![
//                         Some(Rc::new(procedure_main_ui::ProcedureMainUI::new())),
//                         Some(Rc::new(procedure_playing::ProcedurePlaying::new())),
//                         Some(Rc::new(procedure_over::ProcedureOver::new()))]),
//             };
//         }
//         
//         //-------------get component as mult----------------
//         /// 获取可变类型的事件组件 / get mutable event component
//         pub fn get_event_mut(&mut self) -> Option<&mut event::EventComponent>{
//             return Some(&mut self._event_comp);
//         }
//         
//         /// 获取可变类型的输入组件 / get mutable input component
//         pub fn get_input_mut(&mut self) -> Option<&mut input::InputComponent>{
//             return Some(&mut self._input_comp);
//         }
//         
//         /// 获取可变类型的流程组件 / get mutable procedure component
//         pub fn get_procedure_mut(&mut self) -> Option<&mut procedure::ProcedureComponent>{
//             return Some(&mut self._procedure_comp);
//         }
//         //-------------end get component as mult----------------
//         
//         
//     }
// }
