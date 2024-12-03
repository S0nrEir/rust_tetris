// use ggez::input::keyboard::KeyCode;
// use crate::define::enum_define::ProcedureEnum;
// use crate::runtime::app_components::AppComponents;
// use crate::runtime::event::EventComponent;
// use crate::t_updatable::Tickable;
// use crate::tools::logger::{log, LogLevelEnum};
// 
// /// 输入处理器 / Input handler
// type InputHandler = fn(&Controller,KeyCode) -> bool;
// 
// /// 游戏逻辑控制器 / Game logic controller
// #[derive(Debug)]
// pub struct Controller{
//     // _runtime_app_components: Option<Rc<RefCell<AppComponents>>>
//     _input_handler : InputHandler,
//     _curr_key_code : Option<KeyCode>
// }
// 
// impl Controller {
//     
//     /// 设置当前的输入模式 / set the current input mode
//     /// #Arguments
//     /// * `procedure_enum` - 流程枚举 / procedure enum
//     pub fn set_input_mode(&mut self,procedure_enum: ProcedureEnum){
//         match procedure_enum { 
//             ProcedureEnum::MainUI => {
//                 self._input_handler = Controller::main_ui_input_handler;
//             },
//             ProcedureEnum::Playing => {
//                 self._input_handler = Controller::not_impl_input_handler;
//             },
//             ProcedureEnum::Over => {
//                 self._input_handler = Controller::not_impl_input_handler;
//             },
//         }
//     }
//     
//     /// 清除输入处理 / clear input handler
//     pub fn clear_input_handler(&mut self){
//         self._input_handler = Controller::not_impl_input_handler;
//     }
//     
//     /// 清除输入键位 / clear input key code
//     pub fn clear_input_key_code(&mut self){
//         self._curr_key_code = None;
//     }
//     
//     pub fn controller_test(&self,components:&mut AppComponents){
//         components.event_mut().test();
//     }
//     
//     /// 主ui输入处理 / main ui input handler
//     /// #Arguments
//     /// * `key_code` - 按键码 / key code
//     /// #Return
//     /// * `bool` - 是否处理 / whether to process
//     fn main_ui_input_handler(&self,key_code:KeyCode) -> bool{
//         log(&self,&format!("main ui input handler"),LogLevelEnum::Info);
//         match key_code {
//             KeyCode::Return => {
//                 //e
//             },
//             _ => {
//                 log(&self,&format!("main ui input handler : other"),LogLevelEnum::Info);
//             },
//         }
//         return true;
//     }
//     
//     /// 未实现的输入处理 / not implemented input handler
//     /// #Arguments
//     /// * `key_code` - 按键码 / key code
//     /// #Return
//     /// * `bool` - 不处理 / do not process
//     fn not_impl_input_handler(&self,key_code:KeyCode) -> bool{
//         log(&self,&format!("not impl input handler"),LogLevelEnum::Error);
//         return false;
//     }
//     
//     //----------------------------new----------------------------
//     pub fn new() ->Self{
//         return Controller{
//             _input_handler:Controller::not_impl_input_handler,
//             _curr_key_code:None
//         };
//     }
// }
// 
// impl Tickable for Controller{
//      
//     fn on_tick(&mut self, delta_time: f32,interval:f32) {
//         
//     }
// }