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
        //每次tick向下落一次
        let fall_succ_and_reached_top = self._play_field.drop_by_one();
        //顶部存在方块，直接结束游戏
        if(fall_succ_and_reached_top.1){
            self.settlement();
        }
        
        if(fall_succ_and_reached_top.0){
            //下落放置成功，重新生成方块，但如果生成失败要检查下是否已经到了顶部
            if(!self._play_field.generate_new_tetrimino()){
                if(self._play_field.is_top_occupied()){
                    self.settlement();
                }
                
            }
        }
        //下落没有被放置，继续下落
        // else{
        // }
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
        
        let mut is_reached_top = false;
        //是否放置了当前方块
        let mut is_placed = false;
        //处理输入 / handle input
        if(self._input_interval >= constant::INPUT_HANDLE_INTERVAL && !key_code.is_none()){
            if let Some(key_code) = key_code {
                match key_code {
                    //下落
                    KeyCode::Down => {
                        let fall_succ_and_reached_top = self._play_field.try_drop_to_bottom();
                        is_reached_top = fall_succ_and_reached_top.1;
                        if(is_reached_top){
                            self.settlement();
                        }    
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
            }
            self._input_interval = 0.;
            self._curr_input = None;
        }
        
        // main tick
        //处理方块的自然下落
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
    
    /// 结算` / stop game
    fn settlement(&mut self){
        
    }
    
    
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