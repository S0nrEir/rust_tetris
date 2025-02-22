use std::any::Any;
use ggez::{Context, GameResult, graphics};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, DrawMode, Mesh, StrokeOptions, Text, Color, DrawParam};
use ggez::input::keyboard::KeyCode;
use crate::constant;
use crate::define::enum_define::ProcedureEnum;
use crate::runtime::procedure::t_procedure_param::ProcedureParam;
use crate::t_state::TState;
use crate::t_updatable::{Drawable, Tickable};
use crate::tools::logger::*;



/// 测试绘制方块流程 / Test draw block process
#[derive(Debug,Clone)]
pub struct ProcedureTestDrawBlock{
    _tick_counter : u32,
}

impl ProcedureTestDrawBlock{
    pub fn new()-> Self{
        return  ProcedureTestDrawBlock{
            _tick_counter : 0,
        };
    }
    
    fn draw_text(&mut self,canvas: &mut Canvas){
        canvas.draw(
            Text::new("test_text").set_font(constant::FONT_NAME).set_scale(constant::PROC_MAIN_UI_ITEM_TEXT_SCALE),
            Vec2::new(0.0, 0.0)
        );
    }
    fn draw_block(&mut self,canvas: &mut Canvas,ctx:&mut Context) -> GameResult{
        let mesh_rect_1 = Mesh::new_rectangle(
            ctx,
            // DrawMode::Stroke(StrokeOptions::default()),
            DrawMode::fill(),
            //指定位置和长宽
            graphics::Rect::new(0.0, 0.0, 100.0, 100.0),
            Color::WHITE)?;

        let mesh_rect_2 = Mesh::new_rectangle(
            ctx,
            // DrawMode::Stroke(StrokeOptions::default()),
            DrawMode::fill(),
            //指定位置和长宽
            graphics::Rect::new(0.0, 0.0, 100.0, 100.0),
            Color::WHITE)?;
        canvas.draw(&mesh_rect_1, DrawParam::default().dest(Vec2::new(0.0, 0.0)));
        //方块间距离为3就差不多
        canvas.draw(&mesh_rect_1, DrawParam::default().dest(Vec2::new(103.0, 0.0)));
        return Ok(());
    }
    fn draw_another_block(&mut self,canvas: &mut Canvas,ctx:&mut Context) -> GameResult{
        let mesh_rect_1 = Mesh::new_rectangle(
            ctx,
            // DrawMode::Stroke(StrokeOptions::default()),
            DrawMode::fill(),
            //指定位置和长宽
            graphics::Rect::new(0.0, 0.0, 100.0, 100.0),
            Color::RED)?;
        canvas.draw(&mesh_rect_1, DrawParam::default().dest(Vec2::new(103.0, 0.0)));
        return Ok(());
    }
}

impl Tickable for ProcedureTestDrawBlock {
    fn on_tick(&mut self, ctx: &mut Context, delta_time: f32, interval: f32) {
        log_info_colored("ProcedureTestDrawBlock.on_tick()", &format!("tick counter:{}",self._tick_counter), colored::Color::White);
        self._tick_counter += 1;
    }
}

impl TState for ProcedureTestDrawBlock{
    fn on_enter(&mut self, param: Box<dyn ProcedureParam>) {
        log_info_colored("ProcedureTestDrawBlock.on_enter()", &"calling".to_string(), colored::Color::Cyan);
        let mut temp = Vec::new();
        temp.push(1);
        temp.push(2);
        temp.push(3);
    }

    fn on_update(&mut self,ctx:&mut Context, key_code: Option<KeyCode>,delta_sec:f32) -> Option<ProcedureEnum>{
        return Some(ProcedureEnum::TestDrawBlock);
        
    }

    fn on_leave(&mut self, param: Option<Box<dyn ProcedureParam>>) {
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
        // self.draw_text(&mut canvas);
        
        if(self._tick_counter < 10){
            self.draw_block(&mut canvas, ctx);
        }
        else{
            self.draw_another_block(&mut canvas, ctx);
        }
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        return self;
    }
}