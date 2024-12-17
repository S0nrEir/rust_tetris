use std::fmt::Debug;
use ggez::{Context, GameResult};
use ggez::input::keyboard::KeyCode;
use crate::define::enum_define::ProcedureEnum;

///刷帧更新接口 / updatable traits
pub trait Updatable:Debug {
    ///响应刷帧调用 / response to frame call
    /// #Arguments
    /// * `ctx` - 上下文 / context
    /// * `key_code` - 按键码 / key code
    fn on_update(&mut self, ctx : &mut Context , key_code : Option<KeyCode>, delta_sec : f32) -> Option<ProcedureEnum>;
}

pub trait Drawable:Debug {
    ///响应绘制调用 / response to draw call
    /// #Arguments
    /// * `ctx` - 上下文 / context
    fn on_draw(&mut self,ctx:&mut Context) -> GameResult;
}

pub trait Tickable:Debug {
    ///响应触发调用 / response to trigger call
    /// #Arguments
    /// * `ctx` - 上下文 / context
    /// * `delta_time` - 间隔时间 / interval time
    /// * `interval` - 间隔时间 / interval time
    fn on_tick(&mut self,ctx:&mut Context,delta_time:f32,interval:f32);
}