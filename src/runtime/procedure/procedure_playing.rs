use std::any::Any;
use ggez::{Context, GameResult};
use ggez::input::keyboard::KeyCode;
use crate::t_state::TState;
use crate::define::enum_define::ProcedureEnum;
use crate::runtime::data::block_area::BlockArea;
use crate::runtime::procedure::t_procedure_param::ProcedureParam;
use crate::t_updatable::Drawable;
// use crate::runtime::controller::Controller;

///游玩状态
/// playing state
#[derive(Debug)]
pub  struct ProcedurePlaying{
    _block_area: BlockArea
}

impl Drawable for ProcedurePlaying {
    fn on_draw(&mut self, ctx: &mut Context) -> GameResult {
        todo!()
    }
}

impl TState for ProcedurePlaying{
    fn on_enter(&mut self,_param:Box<dyn ProcedureParam>){
        println!("ProcedurePlaying enter");
    }

    fn on_update(&mut self,key_code: KeyCode) {
        println!("ProcedurePlaying update");
    }

    fn on_leave(&self,_param:Option<Box<dyn ProcedureParam>>) {
        println!("ProcedurePlaying exit");
    }

    fn get_state(&self) -> ProcedureEnum {
        return ProcedureEnum::Playing;
    }
}

impl ProcedurePlaying {
    //--------new--------
    pub fn new() -> Self{
        return ProcedurePlaying{
            _block_area:BlockArea::new()
        };
    }
}

#[derive(Debug)]
pub struct ProcedurePlayingParam{
}

impl ProcedurePlayingParam {
    pub fn new() -> Self{
        return ProcedurePlayingParam{};
    }
}

impl ProcedureParam for ProcedurePlayingParam{
    
}