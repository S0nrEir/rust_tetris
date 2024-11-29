use std::fmt::Debug;
use ggez::Context;
use ggez::input::keyboard::KeyCode;

///刷帧更新接口 / updatable traits
pub trait Updatable:Debug {
    ///响应刷帧调用 / response to frame call
    /// #Arguments
    /// * `ctx` - 上下文 / context
    /// * `key_code` - 按键码 / key code
    fn on_update(&mut self, ctx : &mut Context , key_code : Option<KeyCode>);
}

pub trait Tickable:Debug {
    ///响应触发调用 / response to trigger call
    /// #Arguments
    /// * `ctx` - 上下文 / context
    /// * `delta_time` - 间隔时间 / interval time
    /// * `interval` - 间隔时间 / interval time
    fn on_tick(&mut self,ctx:&mut Context,delta_time:f32,interval:f32);
}