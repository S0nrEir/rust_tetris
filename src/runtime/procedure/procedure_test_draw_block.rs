use colored::Color;
use ggez::{Context, GameResult};
use ggez::input::keyboard::KeyCode;
use crate::define::enum_define::ProcedureEnum;
use crate::runtime::procedure::t_procedure_param::ProcedureParam;
use crate::t_state::TState;
use crate::t_updatable::Drawable;
use crate::tools::logger::*;


/// 测试绘制方块流程 / Test draw block process
#[derive(Debug,Clone)]
pub struct ProcedureTestDrawBlock{
    
}

impl ProcedureTestDrawBlock{
    pub fn new()-> Self{
        return  ProcedureTestDrawBlock{
            
        };
    }
}

impl TState for ProcedureTestDrawBlock{
    fn on_enter(&mut self, param: Box<dyn ProcedureParam>) {
        log_info_colored("ProcedureTestDrawBlock.on_enter()",&format!("calling"),Color::Cyan);
    }

    fn on_update(&mut self, key_code: KeyCode) {
        todo!()
    }

    fn on_leave(&self, param: Option<Box<dyn ProcedureParam>>) {
        todo!()
    }

    fn get_state(&self) -> ProcedureEnum {
        return ProcedureEnum::TestDrawBlock;
    }
}

impl Drawable for ProcedureTestDrawBlock {
    fn on_draw(&mut self, ctx: &mut Context) -> GameResult {
        return Ok(());
    }
}

#[derive(Debug)]
pub struct ProcedureTestDrawBlockParam{
    
}

impl ProcedureTestDrawBlockParam {
    pub fn new() -> Self{
        return ProcedureTestDrawBlockParam{
            
        };
    }
}

impl ProcedureParam for ProcedureTestDrawBlockParam {
}