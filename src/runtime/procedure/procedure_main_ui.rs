use std::fmt::Debug;
use ggez::{Context, GameResult, graphics};
use crate::t_state::TState;
use crate::define::enum_define::ProcedureEnum;
use ggez::input::keyboard::KeyCode;
use crate::runtime::procedure::t_procedure_param::ProcedureParam;
use crate::t_updatable::Drawable;
use ggez::graphics::{Canvas, Text};
use crate::constant;

const MAX_ITEM_COUNT:i8 = 2;

///主UI流程 / 
/// main UI procedure
#[derive(Debug)]
pub struct ProcedureMainUI{
    _selected_item_index : i8,
    _param               : Option<ProcedureMainUIParam>,
    _title_text          : Text
}

impl ProcedureMainUI {
    ///constructor
    pub fn new() -> Self {
        return ProcedureMainUI{
            _selected_item_index : 0,
            _param               : None,
            _title_text          : Text::new("Tetris").
                set_font(constant::FONT_NAME).
                set_scale(48.).clone()
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
        
        match new_index{
            MAX_ITEM_COUNT  =>{
                self._selected_item_index = 0;
            },
            -1 => {
                self._selected_item_index = MAX_ITEM_COUNT;
            },
            _ =>{
                self._selected_item_index = new_index;
            }
        }
    }
    
    /// 开始游戏 / start game
    fn start_game(&self){
        
    }
    
    /// 绘制标题文本 / draw title text
    /// #Arguments
    /// * `canvas` - 画布 / canvas
    fn draw_title(&self,canvas: &mut Canvas){
        canvas.draw(
            Text::new("Tetris").set_font(constant::FONT_NAME).set_scale(48.0), 
            ggez::glam::Vec2::new(constant::WINDOW_WIDTH / 2.0, constant::WINDOW_HEIGHT / 2.0)
        );
    }
}

impl Drawable for ProcedureMainUI {
    fn on_draw(&mut self, ctx: &mut Context) -> GameResult {
        //draw title
        
        #[cfg(feature = "debug_log")]{
            crate::tools::logger::log_info_colored("procedure_main_ui.on_draw()", &format!("calling..."), Color::Blue);
        }
        
        let mut canvas = Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        // canvas.draw(
        //     Text::new("Tetris").set_font(constant::FONT_NAME).set_scale(48.),
        //     ggez::glam::Vec2::new(1000., 100.),
        // );
        self.draw_title(&mut canvas);
        canvas.finish(ctx)?;
        
        return Ok(());
    }
}

/// 实现状态机接口 / implement state machine interface
impl TState for ProcedureMainUI{
    
    fn on_enter(&mut self,param:Box<dyn ProcedureParam>){
        #[cfg(feature = "debug_log")]{
            crate::tools::logger::log_info_colored(&self, &format!("proc main ui ---> on enter..."), Color::Cyan);
        }
    }
    
    fn on_update(&mut self,_key_code: KeyCode) {
        
        match _key_code {
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
    }

    fn on_leave(&self,_param:Option<Box<dyn ProcedureParam>>) {
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
}
