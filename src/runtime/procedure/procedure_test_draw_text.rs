use std::any::Any;
use std::fmt::{Debug, Formatter};
use ggez::{Context, GameResult};
use crate::runtime::procedure::t_procedure_param::ProcedureParam;
use crate::t_state::TState;
use crate::t_updatable::{Drawable, Tickable};

#[derive(Debug)]
pub struct ProcedureTestDrawText{
    
}

impl ProcedureTestDrawText{
    pub fn new() -> Self{
        return ProcedureTestDrawText{
            
        };
    }
}

impl Drawable for ProcedureTestDrawText {
    fn on_draw(&mut self, ctx: &mut Context) -> GameResult {
        return Ok(());
    }
}

impl TState for ProcedureTestDrawText{
    fn on_enter(&mut self,param:Box<dyn ProcedureParam>){
        
    }
    fn on_update(&mut self,ctx:&mut ggez::Context,key_code: Option<ggez::input::keyboard::KeyCode>,delta_sec:f32) -> Option<crate::define::enum_define::ProcedureEnum>{
        return None;
    }
    fn on_leave(&mut self,param:Option<Box<dyn ProcedureParam>>){
        
    }
    fn get_state(&self) -> crate::define::enum_define::ProcedureEnum{
        return crate::define::enum_define::ProcedureEnum::TestDrawText;
    }
}

impl Tickable for ProcedureTestDrawText{
    fn on_tick(&mut self,ctx:&mut ggez::Context,delta_time:f32,interval:f32){
        // let text = ggez::graphics::Text::new("Hello, world!");
        // let dest_point = ggez::mint::Point2{x: 100.0, y: 100.0};
        // ggez::graphics::draw(ctx, &text, (dest_point,)).unwrap();
    }
}

#[derive(Debug)]
pub struct ProcedureTestDrawTextParam{
    
}

impl ProcedureTestDrawTextParam{
    pub fn new() -> Self{
        return ProcedureTestDrawTextParam{
            
        };
    }
}

impl ProcedureParam for ProcedureTestDrawTextParam{
    fn as_any_mut(&mut self) -> &mut dyn Any {
        return self;
    }
}