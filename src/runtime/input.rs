use crate::t_updatable::Updatable;

///输入组件 / Input Component
#[derive(Debug)]
pub struct InputComponent{
    
}

impl Updatable for InputComponent{
    fn on_update(&self) {
        println!("InputComponent update");
    }
}

impl InputComponent{
    pub fn new() -> Self{
        return InputComponent{};
    }
    
}