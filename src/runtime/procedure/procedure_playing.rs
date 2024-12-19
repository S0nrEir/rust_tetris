use std::any::Any;
use colored::Color;
use ggez::{Context, GameResult};
use ggez::input::keyboard::KeyCode;
use crate::constant;
use crate::t_state::TState;
use crate::define::enum_define::ProcedureEnum;
use crate::runtime::data::play_field::PlayField;
use crate::runtime::procedure::t_procedure_param::ProcedureParam;
use crate::t_updatable::{Drawable, Tickable};
use crate::runtime::data::playing_data::PlayingData;
use crate::tools::logger::*;

///游玩状态
/// playing state
#[derive(Debug)]
pub  struct ProcedurePlaying{
    /// 游玩区域数据 / play field data
    _play_field     : PlayField,
    /// 玩家数据 / player data
    _player_data    : PlayingData,
    /// 当前的按键输入 / current key input
    _curr_input     : Option<KeyCode>,
    /// 可处理输入的时间间隔 / time interval that can handle input
    _input_interval : f32,
    // tick轮询时间 / tick polling time
    _delta_tick : f32,
}

impl Drawable for ProcedurePlaying {
    fn on_draw(&mut self, ctx: &mut Context) -> GameResult {
        todo!()
    }
}

impl Tickable for ProcedurePlaying {
    fn on_tick(&mut self, ctx: &mut Context, delta_time: f32, interval: f32) {
        
    }
}

impl TState for ProcedurePlaying{
    fn on_enter(&mut self,param:Box<dyn ProcedureParam>){
        log_info_colored("ProcedurePlaying","enter",Color::Cyan);
        self._play_field.init_field_data();
        self._play_field.init_tetrimino();
        self._input_interval = 0.;
        self._delta_tick = 0.;
    }

    fn on_update(&mut self,ctx:&mut Context,key_code: Option<KeyCode>,delta_sec:f32) -> Option<ProcedureEnum>{
        self._curr_input = key_code;
        self._input_interval += delta_sec;
        
        //处理输入 / handle input
        if(self._input_interval >= constant::INPUT_HANDLE_INTERVAL && !key_code.is_none()){
            let key_code = key_code.unwrap();
            match key_code {
                //速落
                KeyCode::Down => {
                    
                },
                //左右移动
                KeyCode::Left | KeyCode::Right => {;
                    //移动成功，更新grid
                    if(self._play_field.try_horizontal_move_tetrimino(key_code == KeyCode::Right)){
                        
                    }
                },
                KeyCode::Up => {
                    //旋转成功，更新grid
                    if(self._play_field.try_rotate_tetrimino(true)){
                        
                    }
                }
                _ => {}
            }//end match key_code
            self._input_interval = 0.;
        }
        
        // main tick
        self._delta_tick += delta_sec;
        if(self._delta_tick >= constant::APP_MAIN_TICK_INTERVAL_1_SEC){
            self.on_tick(ctx,delta_sec,constant::APP_MAIN_TICK_INTERVAL_1_SEC);
            self._delta_tick = 0.;
        }
        self._curr_input = None;
        return Some(ProcedureEnum::Playing);
    }

    fn on_leave(&mut self,_param:Option<Box<dyn ProcedureParam>>) {
        self._play_field.clear();
    }

    fn get_state(&self) -> ProcedureEnum {
        return ProcedureEnum::Playing;
    }
}

impl ProcedurePlaying {
    //--------new--------
    pub fn new() -> Self{
        return ProcedurePlaying{
            _play_field: PlayField::new(),
            _player_data:PlayingData::new(),
            _curr_input:None,
            _input_interval : 0.,
            _delta_tick : 0.
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
    fn as_any_mut(&mut self) -> &mut dyn Any {
        return self;
    }
}