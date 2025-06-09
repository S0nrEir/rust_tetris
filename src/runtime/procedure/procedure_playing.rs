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
use crate::runtime::procedure::procedure_over::ProcedureOverParam;
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
    /// tick轮询时间间隔 / tick polling time interval
    _tick_interval : f32,
    // tick轮询时间 / tick polling time
    _delta_tick : f32,
    //绘制相关 / draw related
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
    ///调试相关 / debug related
    _is_paused : bool,
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
        
        self.draw_score(ctx,&mut canvas);
        self.draw_border(ctx, &mut canvas);
        self.draw_play_field(ctx,&mut canvas);
        self.draw_playing_info(ctx, &mut canvas);
        #[cfg(feature = "debug")]{
            self.draw_playing_state(ctx, &mut canvas);
        }

        canvas.finish(ctx)?;

        return Ok(());
    }
}

impl Tickable for ProcedurePlaying {
    fn on_tick(&mut self, ctx: &mut Context, delta_time: f32, interval: f32) {
        let fall_succ_and_reached_top = self._play_field.fall_one();
        //顶部存在方块，直接结束游戏
        if fall_succ_and_reached_top.1 {
            self.settlement();
            return;
        }
        
        if fall_succ_and_reached_top.0 {
            //下落成功检查消除
            let cleared_line_cnt_and_coords  = self._play_field.try_clear_line();
            //有消除，重新生成
            if cleared_line_cnt_and_coords.0 != 0 {
                self.add_to_performing_coords(cleared_line_cnt_and_coords.1);
                self.switch_playing_state(PlayingStateEnum::Performing);
                self._player_data.add_score(constant::SCORE_PER_LINE * cleared_line_cnt_and_coords.0 as u32);
            }
            //没消除，生成新的
            else{
                // self._play_field.generate_new_tetrimino();
            }
        }
        //下落不成功，生成新的
        else {
            let gen_new_succ = self._play_field.generate_new_tetrimino();
            let is_top_occupied = self._play_field.is_top_occupied();
            if !gen_new_succ && is_top_occupied {
                //如果生成失败且顶部被占用，则结算
                self.settlement();
            }
        }
    }
}

impl TState for ProcedurePlaying{
    fn on_enter(&mut self, mut param: Box<dyn ProcedureParam>){
        let param = param.as_any_mut().downcast_mut::<ProcedurePlayingParam>();
        if param.is_none() {
            #[cfg(feature = "debug")]{
                log("ProcedurePlaying" , "on_enter,using default tick interval" , LogLevelEnum::Info);
            }
            self._tick_interval = constant::APP_MAIN_TICK_INTERVAL;
        }
        else {
            let is_normal_mode = param.as_ref().unwrap()._is_normal_mode;
            #[cfg(feature = "debug")]{
                crate::tools::logger::log("ProcedurePlaying", &format!("on_enter,param is not none, is normal mode: {}", is_normal_mode), LogLevelEnum::Info);
            }
            self._tick_interval = if is_normal_mode { constant::APP_MAIN_TICK_INTERVAL } else { 0.5 };
        }
        log_info_colored("ProcedurePlaying","enter",Color::Cyan);
        self._play_field.reset();
        self._play_field.init_tetrimino();
        self._input_interval = 0.;
        self._delta_tick = 0.;
        self._performing_coords.clear();
        self._performing_duration = 0.;
        self._flash_time = 0.;
        //#todo这块的逻辑写的不好，前面已经初始化过tetrimino了
        let gen_tetri_succ = self._play_field.generate_new_tetrimino();
        if !gen_tetri_succ{
            tools::logger::log("app.rs","generate new tetrimino failed.",Fatal);
            panic!();
        }
        
        self.switch_playing_state(PlayingStateEnum::Falling);
    }

    fn on_update(&mut self,ctx:&mut Context,key_code: Option<KeyCode>,delta_sec:f32) -> (Option<ProcedureEnum>, Option<Box<dyn ProcedureParam>>){

        self._curr_input = key_code;
        self._input_interval += delta_sec;
        let mut procedure_to_return : (Option<ProcedureEnum>,Option<Box<dyn ProcedureParam>>) = (None,None);
        
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
                                    self._player_data.add_score(constant::SCORE_PER_LINE * cleared_line_cnt_and_coords.0 as u32);
                                }
                            }
                            //未到达顶部
                            else{
                                let cleared_line_cnt_and_coords = self._play_field.try_clear_line();
                                #[cfg(feature = "debug")]{
                                    log("ProcedurePlaying",&format!("fall success, cleared line count : {}",cleared_line_cnt_and_coords.0 ),LogLevelEnum::Info);
                                }
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
                                    self._player_data.add_score(constant::SCORE_PER_LINE * cleared_line_cnt_and_coords.0 as u32);
                                }
                            }
                        },//end match down
                        //左右移动
                        KeyCode::Left | KeyCode::Right | KeyCode::A | KeyCode::D => {
                            self._play_field.try_horizontal_move_tetrimino(if actual_key_code == KeyCode::Right || actual_key_code == KeyCode::D {1} else {-1});
                        },
                        //旋转
                        KeyCode::Up | KeyCode::W => {
                            self._play_field.try_rotate_tetrimino(true);
                        }
                        //退出
                        KeyCode::Escape => {
                            ctx.request_quit();
                        }
                        KeyCode::Pause => {
                            #[cfg(feature = "debug")]{
                                self._is_paused = !self._is_paused;
                            }
                        }

                        _ => {}
                    }
                    self._input_interval = 0.0;
                    self._curr_input = None;
                }

                procedure_to_return = (Some(ProcedureEnum::Playing),None);
            },//end match falling
            
            PlayingStateEnum::Performing => {

                #[cfg(feature = "debug")]{
                    log("Performing...", &format!("on_update, performing duration: {}, flash time: {}", self._performing_duration, self._flash_time), LogLevelEnum::Info);
                }

                self._performing_duration += delta_sec;
                if self._performing_duration >= constant::PLAYFIELD_PERFORMING_INTERVAL{
                    procedure_to_return = (Some(ProcedureEnum::Playing),None);
                    self._performing_duration = 0.;
                    self._flash_time = 0.;
                    self._flash_color = graphics::Color::WHITE;
                    self._performing_coords.clear();
                    self._play_field.fall_all_floating_blocks();
                    self._play_field.generate_new_tetrimino();
                    self._flash_color = graphics::Color::WHITE;
                    self.switch_playing_state(PlayingStateEnum::Falling);
                }
                else{
                    self._flash_time += delta_sec;
                    if self._flash_time >= constant::PLAYFIELD_FLASHING_INTERVAL{
                        self._flash_color.r = self._flash_color.r - 0.3;
                        self._flash_color.g = self._flash_color.g - 0.3;
                        self._flash_color.b = self._flash_color.b - 0.3;
                        self._flash_time = 0.;
                    }
                }
                procedure_to_return = (Some(ProcedureEnum::Playing),None);
            },//end match performing
            
            //结算
            PlayingStateEnum::Settlement =>{
                if key_code.is_none(){
                    procedure_to_return = (Some(ProcedureEnum::Playing),None);
                }
                else{
                    let param = Box::new(ProcedureOverParam{
                        _score : self._player_data.get_score()
                    });
                    procedure_to_return = (Some(ProcedureEnum::Over), Some(param));
                }
            }//end match settlement
            
            _ => {}
        }//end match
        
        // main tick
        self._delta_tick += delta_sec;

        if self._delta_tick >= self._tick_interval && 
            !self._is_paused && 
            self._curr_playing_state == PlayingStateEnum::Falling {

            #[cfg(feature = "debug")]{
                log("ProcedurePlaying", &format!("on_tick, delta_sec: {}", delta_sec), LogLevelEnum::Info);
            }

            self._delta_tick = 0.;
            self.on_tick(ctx,delta_sec,constant::APP_MAIN_TICK_INTERVAL);
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
    }
    
    /// 切换到指定的游玩状态 / switch to the specified playing state
    fn switch_playing_state(&mut self,state_to_switch:PlayingStateEnum){
        
        match state_to_switch { 
            PlayingStateEnum::Falling => {
            },
            PlayingStateEnum::Performing => {
                // self._performing_duration = 0.;
                // self._flash_time = 0.;
                // self._flash_color = graphics::Color::WHITE;
                // self._performing_coords.clear();
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

        let mut x_offset : f32;
        let mut y_offset : f32;

        for coords in self._performing_coords.iter(){
            x_offset = (constant::BLOCK_SIZE + constant::BLOCK_COORD_SPACING as f32) * coords.0 as f32;
            y_offset = (constant::BLOCK_SIZE + constant::BLOCK_COORD_SPACING as f32) * coords.1 as f32;
            let mesh = Mesh::new_rectangle
                ( 
                    ctx, 
                    graphics::DrawMode::fill(), 
                    graphics::Rect::new(
                        constant::BLOCK_SIZE + constant::BLOCK_INIT_START_COORD.0 + x_offset,
                        constant::BLOCK_SIZE + constant::BLOCK_INIT_START_COORD.1 + y_offset,
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
    

    /// 绘制游玩状态 / draw playing state
    fn draw_playing_state(&mut self,ctx:&mut Context,canvas:&mut Canvas){
        let mut text = graphics::Text::new(format!("Playing State: {}", self._curr_playing_state.as_str()));
        text.set_font(constant::FONT_NAME);
        text.set_scale(15.0);
        canvas.draw(
            &text,
            DrawParam::default().dest(Vec2::new(10.0, 800.0)).color(ggez::graphics::Color::WHITE)
        );
    }

    fn draw_score(&mut self,ctx:&mut Context,canvas:&mut Canvas){
        let mut score_text = graphics::Text::new(format!("Score : {}", self._player_data.get_score()));
        score_text.set_font(constant::FONT_NAME);
        score_text.set_scale(15.0);
        canvas.draw(
            &score_text,
            DrawParam::default().dest(constant::SCORE_TEXT_POSITION).color(ggez::graphics::Color::CYAN)
        );
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
            canvas.draw(&borders, DrawParam::default());
        }
    }
    
    /// 绘制游玩区域 / draw play field
    fn draw_play_field(&self,ctx:&mut Context,canvas:&mut Canvas){
        let block_area = self._play_field.get_block_area();
        let mut x_offset : f32;
        let mut y_offset : f32;

        for i in 0..block_area.len(){
            x_offset = (constant::BLOCK_SIZE + constant::BLOCK_COORD_SPACING as f32) * i as f32;
            for j in 0..block_area[i].len(){
                let coord = block_area[i][j].get_coord();
                let color = block_area[i][j].color();
                y_offset = (constant::BLOCK_SIZE + constant::BLOCK_COORD_SPACING as f32) * j as f32;
                if self._performing_coords.contains(&(coord.x, coord.y)) || !block_area[i][j].is_occupied() {
                    continue;
                }
                
                let mesh = Mesh::new_rectangle
                    ( 
                        ctx, 
                        graphics::DrawMode::fill(), 
                        graphics::Rect::new(
                            constant::BLOCK_SIZE + constant::BLOCK_INIT_START_COORD.0 + x_offset,
                            constant::BLOCK_SIZE + constant::BLOCK_INIT_START_COORD.1 + y_offset,
                            constant::BLOCK_SIZE,
                            constant::BLOCK_SIZE
                        ),
                        *color
                    );
                
                if let Ok(mesh) = mesh{
                    canvas.draw(&mesh, DrawParam::default());
                }
            }
            
        }
    }
    
    /// 绘制游玩信息 / draw playing info
    fn draw_playing_info(&self, ctx : &mut Context , canvas : &mut Canvas){
        
    }
    
    /// 结算 / stop game
    fn settlement(&mut self){
        self._curr_playing_state = PlayingStateEnum::Settlement;
    }
    
    pub fn new() -> Self{
        let min_position = constant::BORDER_MIN_POSITION;
        let max_position = constant::BORDER_MAX_POSITION;
        
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
            _flash_color : ggez::graphics::Color::WHITE,
            _border_positions : [min_position,
                                 Vec2::new(max_position.x, min_position.y),
                                 max_position,
                                 Vec2::new(min_position.x, max_position.y),
                                 min_position],
            _is_paused : false,
            _tick_interval : 0.0,
            
        };
    }
}

#[derive(Debug)]
pub struct ProcedurePlayingParam{
    pub _is_normal_mode : bool
}

impl ProcedurePlayingParam {
}

impl ProcedureParam for ProcedurePlayingParam{
    fn as_any_mut(&mut self) -> &mut dyn Any {
        return self;
    }
}