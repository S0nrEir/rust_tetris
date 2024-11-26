use std::cell::RefCell;
use std::rc::Rc;
use crate::runtime::event;
use crate::runtime::input::InputComponent;
use crate::runtime::procedure;
use crate::t_state::TState;

///运行时组件集合 / Runtime components
#[derive(Debug)]
pub struct AppComponents{
    _input_comp : InputComponent,
    _event_comp : event::EventComponent,
    _procedure_comp : procedure::ProcedureComponent,
}

impl AppComponents {
    
    //----------------------------get----------------------------
    pub fn input(&self) -> &InputComponent{
        return &self._input_comp;
    }
    
    pub fn event(&self) -> &event::EventComponent{
        return &self._event_comp;
    }
    
    pub fn procedure(&self) -> &procedure::ProcedureComponent{
        return &self._procedure_comp;
    }

    //----------------------------new----------------------------
    pub fn new(initial_proc_index:i32,procedure_list:Vec<Option<Rc<dyn TState>>>) -> Rc<RefCell<Self>>{
        let input_comp = InputComponent::new();
        let mut event_comp = event::EventComponent::new();
        let procedure_comp = procedure::ProcedureComponent::new(initial_proc_index,procedure_list);
        
        let app_components = AppComponents{
            _input_comp:input_comp,
            _event_comp:event_comp,
            _procedure_comp:procedure_comp,
        };
        //app_components moved here
        let app_comp_ref = Rc::new(RefCell::new(app_components));
        //set ref
        app_comp_ref.borrow_mut()._event_comp.set_components(app_comp_ref.clone());
        app_comp_ref.borrow_mut()._input_comp.set_components(app_comp_ref.clone());
        app_comp_ref.borrow_mut()._procedure_comp.set_components(app_comp_ref.clone());
        return app_comp_ref;
    }
}