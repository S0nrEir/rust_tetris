use ggez::{Context, GameResult, graphics};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, DrawMode, Mesh, StrokeOptions, Text, Color, DrawParam};
use ggez::input::keyboard::KeyCode;
use crate::constant;
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
    
    fn draw_text(&mut self,canvas: &mut Canvas){
        canvas.draw(
            Text::new("test_text").set_font(constant::FONT_NAME).set_scale(constant::PROC_MAIN_UI_ITEM_TEXT_SCALE),
            Vec2::new(0.0, 0.0)
        );
    }
    fn draw_block(&mut self,canvas: &mut Canvas,ctx:&mut Context) -> GameResult{
        let mesh_rect = Mesh::new_rectangle(
            ctx,
            // DrawMode::Stroke(StrokeOptions::default()),
            DrawMode::fill(),
            //指定位置和长宽
            graphics::Rect::new(300.0, 0.0, 100.0, 100.0),
            Color::WHITE)?;
        canvas.draw(&mesh_rect, DrawParam::default().dest(Vec2::new(300.0, 100.0)));
        
        return Ok(());
    }
}

impl TState for ProcedureTestDrawBlock{
    fn on_enter(&mut self, param: Box<dyn ProcedureParam>) {
        log_info_colored("ProcedureTestDrawBlock.on_enter()", &"calling".to_string(), colored::Color::Cyan);
    }

    fn on_update(&mut self, key_code: KeyCode) {
    }

    fn on_leave(&self, param: Option<Box<dyn ProcedureParam>>) {
        log_info_colored("ProcedureTestDrawBlock.on_leave()", &"calling".to_string(), colored::Color::Cyan);
    }

    fn get_state(&self) -> ProcedureEnum {
        return ProcedureEnum::TestDrawBlock;
    }
}

impl Drawable for ProcedureTestDrawBlock {
    fn on_draw(&mut self, ctx: &mut Context) -> GameResult {

        let mut canvas = Canvas::from_frame(ctx, graphics::Color::from(constant::COLOR_RGBA_BLACK_1));
        //canvas坐标起始从左上角开始
        self.draw_text(&mut canvas);
        self.draw_block(&mut canvas, ctx);
        canvas.finish(ctx)?;
        
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