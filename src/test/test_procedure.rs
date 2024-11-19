/// 测试流程相关内容 / test procedure related content
#[cfg(test)]
mod test_procedure{
    use std::rc::Rc;
    use crate::procedure::{procedure_main_ui, ProcedureComponent};
    use crate::procedure::procedure_playing;
    use crate::procedure::procedure_over;
    use crate::define::enum_define;
    use crate::tools::Logger::*;
    
    #[test]
    pub fn entry() {
        let mut procedure_comp = ProcedureComponent::new(
            0,
            vec![
                Some(Rc::new(procedure_main_ui::ProcedureMainUI::new())),
                Some(Rc::new(procedure_playing::ProcedurePlaying::new())),
                Some(Rc::new(procedure_over::ProcedureOver::new()))]
        );

        procedure_comp.switch(enum_define::ProcedureEnum::Playing);
        procedure_comp.switch(enum_define::ProcedureEnum::Over);
        procedure_comp.switch(enum_define::ProcedureEnum::MainUI);
        log("test_procedure","test procedure",LogLevelEnum::Info);
    }
}