use std::cell::RefCell;
use std::rc::Rc;
use crate::runtime::app_components::AppComponents;
use crate::t_updatable::Updatable;

///输入组件 / Input Component
#[derive(Debug)]
pub struct InputComponent{
    ///关联的运行时组件集合 / Associated runtime component set
    _runtime_app_components: Option<Rc<RefCell<AppComponents>>>
}

impl Updatable for InputComponent{
    fn on_update(&self) {
        println!("InputComponent update");
    }
}

impl InputComponent{
    pub fn new() -> Self{
        return InputComponent{
            _runtime_app_components:None
        };
    }
}

impl InputComponent {
    ///关联运行时组件集合 / Associated runtime component set
    /// #Arguments
    /// - app_components: 运行时组件集合 / Runtime component set
    /// #Returns
    /// - 是否成功 / Is it successful
    pub fn set_components(&mut self,app_components:Rc<RefCell<AppComponents>>) -> bool{
        self._runtime_app_components = Some(app_components);
        return true;
    }
}