use std::any::Any;
use colored::Color;
use ggez::{Context, GameResult, graphics};
use ggez::glam::{IVec2, Vec2};
use ggez::graphics::{Canvas, DrawParam, Mesh};
use ggez::input::keyboard::KeyCode;
use crate::constant;
use crate::t_state::TState;
use crate::define::enum_define::ProcedureEnum;
use crate::runtime::data::play_field::PlayField;
use crate::runtime::procedure::t_procedure_param::ProcedureParam;
use crate::t_updatable::{Drawable, Tickable};
use crate::runtime::data::playing_data::PlayingData;
use crate::tools::logger::*;
use crate::runtime::procedure::playing_state_enum::PlayingStateEnum;

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
    
    //------------游玩逻辑相关------------
    /// 当前的游玩状态
    _curr_playing_state : PlayingStateEnum,
    //表现要删除的游玩区域方块坐标集合
    _performing_coords : Vec<IVec2>,
}

impl Drawable for ProcedurePlaying {
    fn on_draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = self.draw_background(ctx);
        self.draw_border(ctx, &mut canvas);
        return Ok(());
    }
}

impl Tickable for ProcedurePlaying {
    fn on_tick(&mut self, ctx: &mut Context, delta_time: f32, interval: f32) {
        //每次tick向下落一次
        let fall_succ_and_reached_top = self._play_field.drop_once();
        //顶部存在方块，直接结束游戏
        if fall_succ_and_reached_top.1 {
            self.settlement();
            return;
        }
        
        if fall_succ_and_reached_top.0 {
            //下落成功检查消除
            let cleared_line_cnt_and_coords = self._play_field.try_clear_line();
            //没有消除，重新生成
            if cleared_line_cnt_and_coords.0 == 0 {
                //下落放置成功，重新生成方块，但如果生成失败要检查下是否已经到了顶部
                if !self._play_field.generate_new_tetrimino() && self._play_field.is_top_occupied() {
                    self.settlement();
                }
            }
            //有消除
            else{
                self._performing_coords = cleared_line_cnt_and_coords.1;
                self._curr_playing_state = PlayingStateEnum::Performing;
            }
        }
    }
}

impl TState for ProcedurePlaying{
    fn on_enter(&mut self,param:Box<dyn ProcedureParam>){
        log_info_colored("ProcedurePlaying","enter",Color::Cyan);
        self._play_field.init_field_data();
        self._play_field.init_tetrimino();
        self._input_interval = 0.;
        self._delta_tick = 0.;
        self._curr_playing_state = PlayingStateEnum::Falling;
        self._performing_coords.clear();
    }

    fn on_update(&mut self,ctx:&mut Context,key_code: Option<KeyCode>,delta_sec:f32) -> Option<ProcedureEnum>{
        self._curr_input = key_code;
        self._input_interval += delta_sec;
        let mut procedure_to_return : Option<ProcedureEnum> = None;
        
        match self._curr_playing_state { 
            PlayingStateEnum::Falling => {
                //处理输入 / handle input
                if self._input_interval < constant::INPUT_HANDLE_INTERVAL || key_code.is_none() {
                    return Some(ProcedureEnum::Playing);
                }

                let actual_key_code = key_code.unwrap();
                match actual_key_code{
                    //下落
                    KeyCode::Down => {
                        let fall_succ_and_reached_top = self._play_field.try_drop_to_bottom();
                        //到达顶部
                        if fall_succ_and_reached_top.1 {
                            let cleared_line_cnt_and_coords = self._play_field.try_clear_line();
                            //到达顶部且没有消除方块，则结算
                            if cleared_line_cnt_and_coords.0 == 0 {
                                self.settlement();
                            }
                            //到达顶部有消除，进行表现效果
                            else{
                                self._performing_coords = cleared_line_cnt_and_coords.1;
                                self._curr_playing_state = PlayingStateEnum::Performing;
                            }
                        }
                        //未到达顶部
                        else{
                            let cleared_line_cnt_and_coords = self._play_field.try_clear_line();
                            //未到达顶部，没有消除，重新生成
                            if  cleared_line_cnt_and_coords.0 == 0 {
                                if !self._play_field.generate_new_tetrimino() && self._play_field.is_top_occupied() {
                                    self.settlement();
                                }
                                else{
                                    procedure_to_return = Some(ProcedureEnum::Playing);
                                }
                            }
                            //未到达顶部，有消除
                            else{
                                self._performing_coords = cleared_line_cnt_and_coords.1;
                                self._curr_playing_state = PlayingStateEnum::Performing;
                            }
                        }

                    },//end match down
                    //左右移动
                    KeyCode::Left | KeyCode::Right => {
                        //移动成功，更新grid
                        self._play_field.try_horizontal_move_tetrimino(actual_key_code == KeyCode::Right);
                    },

                    KeyCode::Up => {
                        //旋转成功，更新grid
                        self._play_field.try_rotate_tetrimino(true);
                    }
                    _ => {}
                }
                self._input_interval = 0.0;
                self._curr_input = None;
            },//end match falling
            
            //处理表现
            PlayingStateEnum::Performing => {
                
                procedure_to_return = Some(ProcedureEnum::Playing);
                self._performing_coords.clear();
            },
            
            //结算
            PlayingStateEnum::Settlement =>{
                //没有输入就不做处理
                if key_code.is_none(){
                    procedure_to_return = Some(ProcedureEnum::Playing);
                }
                //有任何输入，就进入结束游戏流程
                else{
                    procedure_to_return = Some(ProcedureEnum::Over);
                }
            }
            _ => {}
        }//end match
        
        // main tick
        self._delta_tick += delta_sec;
        if self._delta_tick >= constant::APP_MAIN_TICK_INTERVAL_1_SEC {
            self.on_tick(ctx,delta_sec,constant::APP_MAIN_TICK_INTERVAL_1_SEC);
            self._delta_tick = 0.;
        }
        
        self._curr_input = None;
        return procedure_to_return;
    }

    fn on_leave(&mut self,_param:Option<Box<dyn ProcedureParam>>) {
        self._play_field.clear();
    }

    fn get_state(&self) -> ProcedureEnum {
        return ProcedureEnum::Playing;
    }
}

impl ProcedurePlaying {
    
    /// 绘制背景 / draw background
    fn draw_background(&mut self,ctx:&mut Context) -> Canvas{
        return  Canvas::from_frame(ctx, graphics::Color::from(constant::COLOR_RGBA_BLACK_1));
    }
    
    /// 绘制边框 / draw border
    fn draw_border(&mut self,ctx:&mut Context,canvas:&mut Canvas){
        let left_border = Mesh::new_line(ctx, &constant::BORDER_POSITIONS, 2.0, ggez::graphics::Color::WHITE);
        if let Ok(left_border) = left_border{
            canvas.draw(&left_border, DrawParam::default().dest(Vec2::new(0.0, 0.0)));
        }
    }
    
    /// 结算 / stop game
    fn settlement(&mut self){
        self._curr_playing_state = PlayingStateEnum::Settlement;
    }
    
    
    pub fn new() -> Self{
        return ProcedurePlaying{
            _play_field: PlayField::new(),
            _player_data:PlayingData::new(),
            _curr_input:None,
            _input_interval : 0.,
            _delta_tick : 0.,
            _curr_playing_state : PlayingStateEnum::Start,
            _performing_coords : Vec::new(),
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