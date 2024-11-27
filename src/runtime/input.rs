use std::cell::RefCell;
use std::rc::Rc;
use ggez::input::keyboard::KeyCode;
// use crate::runtime::app_components::AppComponents;
use crate::t_updatable::{Tickable, Updatable};
use crate::tools::Logger::{log, LogLevelEnum};

///输入组件 / Input Component
#[derive(Debug)]
pub struct InputComponent{
    ///当前输入的key / Current input key
    _curr_input_key : Option<KeyCode>
}

impl Updatable for InputComponent{
    fn on_update(&self) {
        println!("InputComponent update");
    }
}

impl Tickable for InputComponent{
    fn on_tick(&mut self, delta_time: f32,interval:f32){
        
        #[cfg(feature = "debug_log")]{
            log(&self, &format!("InputComponent tick, delta time : {}",delta_time), LogLevelEnum::Info);
        }
    }
}

impl InputComponent{
    pub fn new() -> Self{
        return InputComponent{
            _curr_input_key:None
        };
    }
}

impl InputComponent {
    
    /// 清除输入标记 / Clear input flag
    pub fn clear_input(&mut self){
        self._curr_input_key = None;
    }
    
    ///设置当前输入的键 / Set the current input key
    /// #Arguments
    /// - key: 当前输入的键 / Current input key
    pub fn set_curr_input_key(&mut self,key:Option<KeyCode>){
        self._curr_input_key = key;
    }
    
    
    
    /// 获取当前输入的键 / Get the current input key
    /// #Returns
    /// - 当前输入的键 / Current input key
    pub fn get_curr_input_key(&self) -> Option<KeyCode>{
        return self._curr_input_key;
    }
}