use std::fmt::Debug;

///刷帧更新接口 / updatable traits
pub trait Updatable:Debug {
    fn on_update(&self);
}

pub trait Tickable:Debug {
    ///响应触发调用 / response to trigger call
    /// #Arguments
    /// - delta_time: 距离上一帧时间 / time from last frame
    /// - interval: 主帧间隔时间 / main frame interval time
    /// - components: 运行时组件集合 / runtime component set
    fn on_tick(&mut self,delta_time:f32,interval:f32);
}