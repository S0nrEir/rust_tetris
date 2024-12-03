use std::fmt::Debug;
use ggez::input::keyboard::KeyCode;
use crate::define::enum_define::ProcedureEnum;
use crate::tools::logger::{log, LogLevelEnum};
use crate::define::fn_define::Bool_KeyCode_Fn;
// use crate::tools::logger::*;

///输入组件 / Input Component
#[derive(Debug)]
pub struct InputComponent{
    ///当前输入的key / Current input key
    _curr_input_key : Option<KeyCode>,
    _input_filter : Bool_KeyCode_Fn,
}

// impl Updatable for InputComponent{
//     fn on_update(&self) {
//         println!("InputComponent update");
//     }
// }

// impl Tickable for InputComponent{
//     fn on_tick(&mut self, ctx:&mut Context, key_code: KeyCode, delta_time: f32, interval: f32) {
//         todo!()
//     }
// }


impl InputComponent {
    pub fn new() -> Self{
        return InputComponent{
            _curr_input_key:None,
            _input_filter:InputComponent::not_impl_input_filter,
        };
    }
    
    /// 清除输入标记 / Clear input flag
    pub fn clear_input(&mut self){
        self._curr_input_key = None;
    }
    
    /// 默认的未实现的输入过滤器 / Unimplemented input filter
    /// #Arguments
    /// * `key_code` - 输入的键 / input key
    /// #Returns
    /// - 总是返回失败 / Always return false
    fn not_impl_input_filter(key_code: KeyCode) -> bool{
        log("input.rs", &format!("not impl input filter"), LogLevelEnum::Error);
        return false;
    }

    /// 主ui状态输入过滤器 / Input filter
    /// #Arguments
    /// * `key_code` - 输入的键 / input key
    /// #Returns
    /// - 是否接受输入 / Whether to accept input
    fn main_ui_input_filter(key_code: KeyCode) -> bool{
        return match key_code {
            KeyCode::Return |
            KeyCode::Up |
            KeyCode::Down |
            KeyCode::Left |
            KeyCode::Right |
            KeyCode::Space => {
                true
            },
            _ => {
                false
            },
        }
    }

    fn playing_input_filter(key_code: KeyCode) -> bool{
        return true;
    }

    fn over_input_filter(key_code: KeyCode) -> bool{
        return true;
    }
    
    /// 设置当前输入的键 / Set the current input key
    /// #Arguments
    /// * `key` - 要设置的键 / key to set
    pub fn set_curr_input_key(&mut self,key:Option<KeyCode>){
        if let Some(key_code) = key{
            //在流程内检查输入，这里不需要了 / Check input in the process, no longer needed here
            // if !(self._input_filter)(key.unwrap()){
            //     log(&self, &format!("input not impled,keycode"), LogLevelEnum::Warning);
            //     return;
            // }
            self._curr_input_key = key;
        }
    }
    
    pub fn set_input_filter_mode(&mut self,procedure:ProcedureEnum){
        match procedure {
            ProcedureEnum::MainUI => {
                self._input_filter = InputComponent::main_ui_input_filter;
            },
            ProcedureEnum::Playing => {
                self._input_filter = InputComponent::playing_input_filter;
            },
            ProcedureEnum::Over => {
                self._input_filter = InputComponent::over_input_filter;
            },
            ProcedureEnum::TestDrawBlock => {
                self._input_filter = InputComponent::not_impl_input_filter;
            },
        }
    }
    
    /// 获取当前输入的键 / Get the current input key
    /// #Returns
    /// - 当前输入的键 / Current input key
    pub fn get_curr_input_key(&self) -> Option<KeyCode>{
        return self._curr_input_key;
    }
}