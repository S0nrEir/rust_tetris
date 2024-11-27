// use std::cell::RefCell;
// use std::rc::Rc;
// use crate::define::enum_define::ProcedureEnum;
// use crate::runtime::controller::Controller;
// use crate::runtime::event;
// use crate::runtime::input::InputComponent;
// use crate::runtime::procedure;
// use crate::runtime::procedure::ProcedureComponent;
// use crate::t_state::TState;
// 
// ///运行时组件集合 / Runtime components
// #[derive(Debug)]
// pub struct AppComponents{
//     /// 输入组件 / Input Component
//     _input_comp : InputComponent,
//     ///事件组件 / Event Component
//     _event_comp : event::EventComponent,
//     ///流程组件 / Procedure Component
//     _procedure_comp : procedure::ProcedureComponent,
//     ///逻辑控制器 / Logic Controller
//     _controller : Controller,
// }
// 
// impl AppComponents {
//     
//     //----------------------------get----------------------------
//     pub fn controller_mut(&mut self) -> &mut Controller{ 
//         return &mut self._controller; 
//     }
// 
//     pub fn controller(&self) -> &Controller{
//         return &self._controller;
//     }
//     
//     pub fn input_mut(&mut self) -> &mut InputComponent{ 
//         return &mut self._input_comp; 
//     }
//     
//     pub fn input(&self) -> &InputComponent{ 
//         return &self._input_comp; 
//     }
//     
//     pub fn event(&mut self) -> &mut event::EventComponent{
//         return &mut self._event_comp;
//     }
//     
//     pub fn procedure(&self) -> &ProcedureComponent{ 
//         return &self._procedure_comp; 
//     }
// 
//     pub fn procedure_mut(&mut self) -> &mut procedure::ProcedureComponent{
//         return &mut self._procedure_comp;
//     }
// 
//     //----------------------------new----------------------------
//     pub fn new(initial_proc_index:i32,procedure_list:Vec<Option<Rc<dyn TState>>>) -> Self{
//         let input_comp = InputComponent::new();
//         let event_comp = event::EventComponent::new();
//         let procedure_comp = procedure::ProcedureComponent::new(initial_proc_index,procedure_list);
//         let controller = Controller::new();
//         let mut app_components = AppComponents{
//             _input_comp:input_comp,
//             _event_comp:event_comp,
//             _procedure_comp:procedure_comp,
//             _controller:controller
//         };
//         
//         app_components._procedure_comp.switch(ProcedureEnum::MainUI,&mut app_components._controller);
//         return app_components;
//     }
// }