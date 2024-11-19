use std::collections::HashMap;
use crate::t_updatable::Updatable;
use crate::tools::Logger::*;

///事件组件 / Event Component
#[derive(Debug)]
pub struct EventComponent {
    ///回调列表
    _event_map:HashMap<i32,Vec<fn()>>
}

impl EventComponent {
    ///constructor
    pub fn new() -> Self {
        return EventComponent{
            _event_map:HashMap::new(),
        };
    }
    
    /// 触发事件 / Fire Event
    /// #Arguments
    /// - event_id: 事件ID
    pub fn fire(&mut self,event_id:i32){
        if(!self._event_map.contains_key(&event_id)){
            log(self, &format!("event map doesnt contains key : {}",event_id), LogLevelEnum::Error);
            return;
        }
        
        let vec = self._event_map.get(&event_id).unwrap();
        for i in 0..vec.len(){
            vec[i]();
        }
    }
    
    ///注册一个事件  / Register an event
    /// #Arguments
    /// - event_id: 事件ID
    /// - call_back: 回调函数
    pub fn register_event(&mut self,event_id:i32,call_back:fn()) -> bool{

        if(!self._event_map.contains_key(&event_id)){
            let mut call_back_vec : Vec<fn()> = Vec::new();
            call_back_vec.push(call_back);
            self._event_map.insert(event_id,call_back_vec);
            return true;
        }
        
        let call_back_vec  = self._event_map.get_mut(&event_id).unwrap();
        if(!call_back_vec.contains(&call_back)){
            log(self, &format!("event map contains call back , event id: {} ",event_id), LogLevelEnum::Error);
            return false;
        }
        call_back_vec.push(call_back);
        return true;
    }

    ///注销一个事件 / Unregister an event
    /// #Arguments
    /// - event_id: 事件ID
    /// - call_back: 回调函数
    /// #Returns
    /// - 是否成功 / Is it successful
    pub fn unregister_event(&mut self,event_id: i32,call_back:fn()) -> bool {
        if(!self._event_map.contains_key(&event_id)){
            log(self, &format!("event map doesnt contains key : {}",event_id), LogLevelEnum::Error);
            return false;
        }
        
        let call_back_vec = self._event_map.get_mut(&event_id).unwrap();
        if(!call_back_vec.contains(&call_back)){
            log(self, &format!("event map doesnt contains call back , event id: {}",event_id), LogLevelEnum::Error);
            return false;
        }
        let index = call_back_vec.iter().position(|x| *x == call_back).unwrap();
        call_back_vec.clear();
        self._event_map.remove(&event_id);
        return true;
    }
}

impl Updatable for EventComponent {
    fn on_update(&self) {
        println!("EventComponent update");
    }
}