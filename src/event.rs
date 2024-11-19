use std::collections::HashMap;
use crate::t_updatable::Updatable;
use crate::define::fn_define::Void_CallBack;

///事件组件 / Event Component
#[derive(Debug)]
pub struct EventComponent {
    ///回调列表
    _event_map:HashMap<i32,Vec<Void_CallBack>>
}

impl EventComponent {
    ///constructor
    pub fn new() -> Self {
        return EventComponent{
            _event_map:HashMap::new(),
        };
    }
    
    pub fn register_event(&self) {
        println!("EventComponent register");
    }
    
    pub fn unregister_event(&self) {
        println!("EventComponent unregister");
    }
}

impl Updatable for EventComponent {
    fn on_update(&self) {
        println!("EventComponent update");
    }
}