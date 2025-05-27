use std::any::Any;
use std::collections::{HashSet};
use colored::Color;
use ggez::{Context, GameResult, graphics};
use ggez::glam::{IVec2, Vec2};
use ggez::graphics::{Canvas, DrawParam, Mesh};
use ggez::input::keyboard::KeyCode;
use crate::{constant, tools};
use crate::t_state::TState;
use crate::define::enum_define::ProcedureEnum;
use crate::runtime::data::play_field::PlayField;
use crate::runtime::procedure::t_procedure_param::ProcedureParam;
use crate::t_updatable::{Drawable, Tickable};
use crate::runtime::data::playing_data::PlayingData;
use crate::tools::logger::*;
use crate::runtime::procedure::playing_state_enum::PlayingStateEnum;
use crate::tools::logger::LogLevelEnum::Fatal;

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
    
    //绘制相关 / draw related
    /// 要绘制的mesh集合，每一个mesh表示一个方块 / mesh collection to be drawn, each mesh represents a block
    //_meshes: [[Mesh;constant::BLOCK_AREA_MAX_HORIZONTAL_BLOCK_CNT];constant::BLOCK_AREA_MAX_VERTICAL_BLOCK_CNT],
    
    //------------游玩表现相关------------
    /// 当前的游玩状态 / current playing state
    _curr_playing_state : PlayingStateEnum,
    /// 表现要删除的游玩区域方块坐标集合 / playing area block coordinates to be deleted
    _performing_coords : HashSet<(i32,i32)>,
    /// 表现效果持续时间 / duration of performance effect
    _performing_duration : f32,
    /// 演出中消除的闪烁时间 / flash time during performance
    _flash_time : f32,
    /// 闪烁颜色 / flash color
    _flash_color : graphics::Color,
    
    /// 边框屏幕坐标位置 / border screen positions
    _border_positions : [Vec2;5],
}

impl Drawable for ProcedurePlaying {
    fn on_draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = self.draw_background(ctx);
        
        match self._curr_playing_state { 
            PlayingStateEnum::Falling => {
                
            },
            PlayingStateEnum::Performing => {
                self.draw_performing(ctx,&mut canvas);
            },
            PlayingStateEnum::Settlement => {
                
            },
            _ => {}
        }
        
        self.draw_border(ctx, &mut canvas);
        self.draw_play_field(ctx,&mut canvas);
        return Ok(());
    }
}

impl Tickable for ProcedurePlaying {
    fn on_tick(&mut self, ctx: &mut Context, delta_time: f32, interval: f32) {
        //每次tick向下落一次
        let fall_succ_and_reached_top = self._play_field.fall_one();
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
                self.add_to_performing_coords(cleared_line_cnt_and_coords.1);
                self.switch_playing_state(PlayingStateEnum::Performing);
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
        self._performing_coords.clear();
        self._performing_duration = 0.;
        self._flash_time = 0.;
        let gen_tetri_succ = self._play_field.generate_new_tetrimino();
        if !gen_tetri_succ{
            tools::logger::log("app.rs","generate new tetrimino failed.",Fatal);
            panic!();
        }
        
        self._play_field.reset();
        self.switch_playing_state(PlayingStateEnum::Falling);
    }

    fn on_update(&mut self,ctx:&mut Context,key_code: Option<KeyCode>,delta_sec:f32) -> Option<ProcedureEnum>{
        self._curr_input = key_code;
        self._input_interval += delta_sec;
        let mut procedure_to_return : Option<ProcedureEnum> = None;
        
        match self._curr_playing_state {
            
            PlayingStateEnum::Falling => {
                
                if self._input_interval >= constant::INPUT_HANDLE_INTERVAL && !key_code.is_none(){
                    let actual_key_code = key_code.unwrap();
                    match actual_key_code{
                        //下落
                        KeyCode::Down | KeyCode::S => {
                            let fall_succ_and_reach_top = self._play_field.try_fall_to_bottom();
                            //到达顶部
                            if fall_succ_and_reach_top.1 {
                                let cleared_line_cnt_and_coords = self._play_field.try_clear_line();
                                //到达顶部且没有消除方块，则结算
                                if cleared_line_cnt_and_coords.0 == 0 {
                                    self.settlement();
                                }
                                //到达顶部有消除，进行表现效果
                                else{
                                    self.add_to_performing_coords(cleared_line_cnt_and_coords.1);
                                    self.switch_playing_state(PlayingStateEnum::Performing);
                                }
                            }
                            //未到达顶部
                            else{
                                let cleared_line_cnt_and_coords = self._play_field.try_clear_line();
                                //未到达顶部，没有消除，重新生成
                                if  cleared_line_cnt_and_coords.0 == 0 {
                                    //生成失败也结算
                                    if !self._play_field.generate_new_tetrimino() && self._play_field.is_top_occupied() {
                                        self.settlement();
                                    }
                                    else{
                                        // procedure_to_return = Some(ProcedureEnum::Playing);
                                    }
                                }
                                //未到达顶部，但有消除
                                else{
                                    self.add_to_performing_coords(cleared_line_cnt_and_coords.1);
                                    self.switch_playing_state(PlayingStateEnum::Performing);
                                }
                            }
                        },//end match down
                        //左右移动
                        KeyCode::Left | KeyCode::Right | KeyCode::A | KeyCode::D => {
                            let offset = if actual_key_code == KeyCode::Right || actual_key_code == KeyCode::D {1} else {-1};
                            self._play_field.try_horizontal_move_tetrimino(offset);
                        },
                        //旋转
                        KeyCode::Up | KeyCode::W => {
                            //旋转成功，更新grid
                            self._play_field.try_rotate_tetrimino(true);
                        }
                        //退出
                        KeyCode::Escape => {

                        }
                        _ => {}
                    }
                    
                    procedure_to_return = Some(ProcedureEnum::Playing);
                    self._input_interval = 0.0;
                    self._curr_input = None;
                }
            },//end match falling
            
            //处理表现
            PlayingStateEnum::Performing => {
                self._performing_duration += delta_sec;
                if self._performing_duration >= constant::PLAYFIELD_PERFORMING_INTERVAL{
                    procedure_to_return = Some(ProcedureEnum::Playing);
                    self._performing_duration = 0.;
                    self._performing_coords.clear();
                    self._play_field.generate_new_tetrimino();
                    self.switch_playing_state(PlayingStateEnum::Falling);
                }
                else{
                    self._flash_time += delta_sec;
                    if self._flash_time >= constant::PLAYFIELD_FLASHING_INTERVAL{
                        self._flash_color.r += self._flash_color.r * -1.;
                        self._flash_color.r += self._flash_color.g * -1.;
                        self._flash_color.r += self._flash_color.b * -1.;
                    }
                }
            },//end match performing
            
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
            }//end match settlement
            
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
    
    /// 添加区块坐标到要表现的坐标集合 / add block coordinates to the set of coordinates to be performed
    fn add_to_performing_coords(&mut self,performing_coords : Vec<IVec2>){
        
        for coords in performing_coords.iter(){
            self._performing_coords.insert((coords.x, coords.y));
        }
        
        // self._performing_coords = performing_coords.clone();
        // self._performing_coords.clear();
        // for coords in performing_coords.iter(){
        //     self._performing_coords.insert(format!("{}{}",coords.x,coords.y));
        // }
    }
    
    /// 切换到指定的游玩状态 / switch to the specified playing state
    fn switch_playing_state(&mut self,state_to_switch:PlayingStateEnum){
        
        match state_to_switch { 
            PlayingStateEnum::Falling => {
            },
            PlayingStateEnum::Performing => {
                self._performing_duration = 0.;
                self._flash_time = 0.;
                self._performing_coords.clear();
            },
            PlayingStateEnum::Settlement => {
            },
            _ => {}
        }//end match
        
        self._curr_playing_state = state_to_switch;
    }
    
    /// 绘制背景 / draw background
    fn draw_background(&mut self,ctx:&mut Context) -> Canvas{
        return  Canvas::from_frame(ctx, graphics::Color::from(constant::COLOR_R0G0B0A1));
    }
    
    
    fn draw_performing(&mut self,ctx:&mut Context,canvas:&mut Canvas){
        
        if self._performing_coords.len() == 0{
            return;
        }
        
        //绘制表现效果
        for coords in self._performing_coords.iter(){
            let mesh = Mesh::new_rectangle
                (
                    ctx, 
                    graphics::DrawMode::fill(), 
                    graphics::Rect::new(
                        (coords.0 as f32) + constant::BLOCK_INIT_START_COORD.0 + constant::BLOCK_COORD_SPACING as f32,
                        (coords.1 as f32) + constant::BLOCK_INIT_START_COORD.1 + constant::BLOCK_COORD_SPACING as f32,
                        constant::BLOCK_SIZE as f32,
                        constant::BLOCK_SIZE as f32
                    ),
                    self._flash_color
                );
            
            if let Ok(mesh) = mesh{
                canvas.draw(&mesh, DrawParam::default());
            }
        }//end for
    }
    
    /// 绘制边框 / draw border
    fn draw_border(&mut self,ctx:&mut Context,canvas:&mut Canvas){
        let borders = Mesh::new_line
            (
                ctx, 
                &self._border_positions,
                2.0, ggez::graphics::Color::WHITE
            );
        
        if let Ok(borders) = borders{
            canvas.draw(&borders, DrawParam::default().dest(Vec2::new(0.0, 0.0)));
        }
    }
    
    /// 绘制游玩区域 / draw play field
    fn draw_play_field(&self,ctx:&mut Context,canvas:&mut Canvas){
        let block_area = self._play_field.get_block_area();
        for i in 0..block_area.len(){
            for j in 0..block_area[i].len(){
                //绘制所有方块
                let block = block_area[i][j];
                let coord = block.get_coord();
                
                if self._performing_coords.contains(&(coord.x, coord.y)){
                    continue;
                }
                
                let mesh = Mesh::new_rectangle
                    (
                        ctx, 
                        graphics::DrawMode::fill(), 
                        graphics::Rect::new(
                            (coord.x as f32) * constant::BLOCK_SIZE + constant::BLOCK_INIT_START_COORD.0 + constant::BLOCK_COORD_SPACING as f32, 
                            (coord.y as f32) * constant::BLOCK_SIZE + constant::BLOCK_INIT_START_COORD.1 + constant::BLOCK_COORD_SPACING as f32,
                            constant::BLOCK_SIZE as f32,
                            constant::BLOCK_SIZE as f32
                        ),
                        block.color().clone()
                    );
                
                if let Ok(mesh) = mesh{
                    canvas.draw(&mesh, DrawParam::default());
                }
            }
        }
    }
    
    /// 绘制游玩信息 / draw playing info
    fn draw_playing_info(&self,ctx:&mut Context,canvas:& mut Canvas){
        
    }
    
    /// 结算 / stop game
    fn settlement(&mut self){
        self._curr_playing_state = PlayingStateEnum::Settlement;
    }
    
    pub fn new() -> Self{
        let min_position = constant::BORDER_MIN_POSITION;
        let max_position = constant::BORDER_MAX_POSITION;
        
        log("procedure_playing.rs","procedure_playing.rs ---> create ProcedurePlaying",LogLevelEnum::Info);
        
        return ProcedurePlaying{
            _play_field: PlayField::new(),
            _player_data:PlayingData::new(),
            _curr_input:None,
            _input_interval : 0.,
            _delta_tick : 0.,
            _curr_playing_state : PlayingStateEnum::Start,
            _performing_coords : HashSet::new(),
            _performing_duration : 0.,
            _flash_time : 0.,
            _flash_color : ggez::graphics::Color::from_rgb(1,1,1),
            _border_positions : [min_position,
                                 Vec2::new(max_position.x, min_position.y),
                                 max_position,
                                 Vec2::new(min_position.x, max_position.y),
                                 min_position]
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