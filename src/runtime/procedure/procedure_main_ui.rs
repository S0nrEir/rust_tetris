use std::any::{Any, TypeId};
use std::fmt::Debug;
use colored::Color;
use ggez::{Context, GameResult, graphics};
use ggez::glam::Vec2;
use crate::t_state::TState;
use crate::define::enum_define::ProcedureEnum;
use ggez::input::keyboard::KeyCode;
use crate::runtime::procedure::t_procedure_param::ProcedureParam;
use crate::t_updatable::{Drawable, Tickable};
use ggez::graphics::{Canvas, Text};
use crate::constant;

const MAX_ITEM_COUNT:i8 = 2;

///主UI流程 / 
/// main UI procedure
#[derive(Debug)]
pub struct ProcedureMainUI{
    _selected_item_index : i8,
    _param               : Option<ProcedureMainUIParam>,
    _title_text_offset   : Vec2,
    _start_game_flag     : bool
}

impl ProcedureMainUI {
    ///constructor
    pub fn new() -> Self {
        return ProcedureMainUI{
            _selected_item_index : 0,
            _param               : None,
            _title_text_offset   : Vec2::new(-100., 0.),
            _start_game_flag     : false
        };
    }
    
    /// 设置当前的选择索引 / set the current selection index
    /// #Arguments
    /// * `move_offset` - 索引偏移 / index offset
    fn select_item(&mut self, move_offset:i8) {
        let new_index = self._selected_item_index + move_offset;
        #[cfg(feature = "debug_log")]{
            crate::tools::logger::log_info_colored(&self, &format!("menu select new index:{}", new_index), Color::Cyan);
        }
        
        if new_index <= 0 {
            self._selected_item_index = 0;
        }
        else if new_index >= MAX_ITEM_COUNT {
            self._selected_item_index = 1;
        }
        else {
            self._selected_item_index = new_index;
        }
    }
    
    /// 开始游戏 / start game
    fn start_game(&mut self){
        self._start_game_flag = true;
    }
    
    /// 绘制标题文本 / draw title text
    /// #Arguments
    /// * `canvas` - 画布 / canvas
    fn draw_title(&self,canvas: &mut Canvas){
        canvas.draw(
            Text::new("Tetris").set_font(constant::FONT_NAME).set_scale(constant::PROC_MAIN_UI_ITEM_TEXT_SCALE), 
            Vec2::new(
                constant::WINDOW_WIDTH / 2.0 + self._title_text_offset.x, 
                constant::WINDOW_HEIGHT / 4.0 + self._title_text_offset.y)
        );
    }
}

impl Drawable for ProcedureMainUI {
    fn on_draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, graphics::Color::from(constant::COLOR_RGBA_BLACK_1));
        //draw title
        self.draw_title(&mut canvas);
        canvas.finish(ctx)?;
        
        return Ok(());
    }
}

impl Tickable for ProcedureMainUI {
    fn on_tick(&mut self, ctx: &mut Context, delta_time: f32, interval: f32) {
    }
}

/// 实现状态机接口 / implement state machine interface
impl TState for ProcedureMainUI{
    
    fn on_enter(&mut self, mut param:Box<dyn ProcedureParam>){
        self._start_game_flag = true;
        //let temp = param.as_any_mut().downcast_mut::<ProcedureMainUIParam>();
        
        #[cfg(feature = "debug_log")]{
            crate::tools::logger::log_info_colored(&self, &format!("proc main ui ---> on enter..."), Color::Cyan);
        }
    }
    
    fn on_update(&mut self,ctx:&mut Context,key_code: Option<KeyCode>,delta_sec:f32) -> Option<ProcedureEnum>{
        
        if(self._start_game_flag){
            return Some(ProcedureEnum::Playing);
        }
        
        if(key_code.is_none()){
            return Some(ProcedureEnum::MainUI);
        }
        
        match key_code.unwrap() {
            KeyCode::Return => {
                self.start_game();
            },
            KeyCode::Up => {
                self.select_item(1);
            },
            KeyCode::Down => {
                self.select_item(-1);
            },
            _ => {},
        }
        return Some(ProcedureEnum::MainUI);
    }

    fn on_leave(&mut self,_param:Option<Box<dyn ProcedureParam>>) {
    }
    
    fn get_state(&self) -> ProcedureEnum {
        return ProcedureEnum::MainUI;
    }
}

#[derive(Debug)]
pub struct ProcedureMainUIParam{
    pub _default_item_index : i32,
}

impl ProcedureMainUIParam {
    pub fn new() -> Self {
        return ProcedureMainUIParam{
            _default_item_index : 0
        };
    }
}

impl ProcedureParam for ProcedureMainUIParam {
    //之所以这么麻烦，在每个实现中都要实现一遍相同的代码
    //是因为处于安全考虑，rust需要知道每个struct的实际类型，这其实相当于将该struct的类型转换为了any
    fn as_any_mut(&mut self) -> &mut dyn Any {
        return self;
    }
}
