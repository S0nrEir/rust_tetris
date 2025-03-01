﻿use std::any::Any;
use ggez::{Context, GameResult};
use ggez::input::keyboard::KeyCode;
use crate::t_state::TState;
use crate::define::enum_define::ProcedureEnum;
use crate::runtime::procedure::t_procedure_param::ProcedureParam;
use crate::t_updatable::{Drawable, Tickable};

///游戏结束，结算，重开
/// game over, settlement, restart
#[derive(Debug,Clone)]
pub  struct ProcedureOver{
}

impl Drawable for ProcedureOver {
    fn on_draw(&mut self, ctx: &mut Context) -> GameResult {
        todo!()
    }
}

impl Tickable for ProcedureOver {
    fn on_tick(&mut self, ctx: &mut Context, delta_time: f32, interval: f32) {
    }
}

impl TState for ProcedureOver{
    
    //--------impl--------
    fn on_enter(&mut self,_param:Box<dyn ProcedureParam>){
        println!("ProcedureOver enter");
    }

    fn on_update(&mut self,ctx:&mut Context,key_code: Option<KeyCode>,delta_sec:f32) -> Option<ProcedureEnum>{
        // println!("ProcedureOver update");
        return Some(ProcedureEnum::Over);
    }

    fn on_leave(&mut self,_param:Option<Box<dyn ProcedureParam>>) {
    }
    
    fn get_state(&self) -> ProcedureEnum {
        return ProcedureEnum::Over;
    }
}


impl ProcedureOver {
    //--------new--------
    pub fn new() -> Self{
        return ProcedureOver{};
    }   
}

#[derive(Debug,)]
pub struct ProcedureOverParam{
}

impl ProcedureParam for ProcedureOverParam{
    fn as_any_mut(&mut self) -> &mut dyn Any {
        return self;
    }
}