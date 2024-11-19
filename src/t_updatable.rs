use std::fmt::Debug;

///刷帧更新接口 / updatable traits
pub trait Updatable:Debug {
    fn on_update(&self);
}