mod procedure_main_ui;
mod procedure_playing;
mod procedure_over;
use crate::t_state::TState;

/// 流程组件，用于控制流程 /
/// procedure component, used to control procedure
pub struct ProcedureComponent{
    _current_procedure:Box<dyn TState>,
}

impl ProcedureComponent {
    
    ///constructor
    /// # Arguments
    /// * 'state' - 初始状态 / initial state
    /// * 'is_enter_after_init' - 是否在初始化后进入状态 / is enter state after init
    pub fn init(&mut self, state:Box<dyn TState>,is_enter_after_init:bool) {
        println!("init procedure");
        self._current_procedure = state;
        if(is_enter_after_init){
            self._current_procedure.on_enter();
        }
    }
}