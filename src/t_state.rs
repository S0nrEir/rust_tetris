///一个简易的状态 / 
/// A simple state
pub trait TState {
    fn on_enter(&self);
    fn on_update(&self);
    fn on_leave(&self);
}