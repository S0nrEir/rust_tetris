///一个简易的状态 / 
/// A simple state
pub trait TState {
    ///进入状态
    // fn on_enter(param:&dyn std::any::Any);
    // 
    // ///更新状态，每帧调用
    // fn on_update(param:&dyn std::any::Any);
    // 
    // ///离开状态
    // fn on_leave(param:&dyn std::any::Any);

    fn on_enter(&self);
    fn on_update(&self);
    fn on_leave(&self);
}