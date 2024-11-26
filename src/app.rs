﻿use std::borrow::Borrow;
use std::{env, fmt, mem, path};
use std::cell::RefCell;
use std::fmt::{Display};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use ggez::{Context, event, GameResult, graphics::{self}};
use ggez::conf::WindowMode;
use ggez::input::keyboard::KeyCode::AbntC1;
use ggez::input::keyboard::KeyInput;
use rand::Rng;
use crate::constant;
use crate::runtime::input::InputComponent;
use crate::runtime::app_components::AppComponents;
use crate::runtime::procedure::{procedure_main_ui, procedure_over, procedure_playing};
use crate::t_state::TState;
use crate::tools::Logger::{log, LogLevelEnum};

/// 游戏的主入口 / Main entry of the game
pub struct App {
    ///帧率 / Frame rate
    _frames : usize,
    ///事件组件 / Event component
    // _event_comp : EventComponent,
    // ///输入组件 / Input component
    // _input_comp : InputComponent,
    // ///流程组件 / Procedure component
    // _procedure_comp : ProcedureComponent,
    // ///上一帧到当前帧的时间间隔 / Time interval from last frame to current frame
     _elapsed_sec_from_last_frame: f64,
    //组件集合 / Component collection
    _app_components : Option<Rc<RefCell<AppComponents>>>, 
}

//实现EventHandler trait以注册事件回调，以及子模块 / Implement the EventHandler trait to register event callbacks, as well as submodules
impl App {
    
    /// 启动 / Start
    pub fn start_up(){
        let mut context_builder = ggez::ContextBuilder::new(constant::APP_GAME_ID, constant::APP_AUTHOR_NAME)
            //resources
            .add_resource_path(Self::get_config_resource_dir())
            //window
            .window_mode(WindowMode::default().dimensions(constant::WINDOW_WIDTH, constant::WINDOW_HEIGHT));;
        
        //get context builder & build app
        if let Ok((mut context, event_loop)) = context_builder.build() {
            //initial procedures
            let procedure_list: Vec<Option<Rc<dyn TState>>> = vec![
                        Some(Rc::new(procedure_main_ui::ProcedureMainUI::new())),
                        Some(Rc::new(procedure_playing::ProcedurePlaying::new())),
                        Some(Rc::new(procedure_over::ProcedureOver::new()))];
            
            if let Ok(mut app) = App::new(&mut context, constant::RUNTIME_INITIAL_PROCEDURE_INDEX, procedure_list){
                event::run(context, event_loop,app);
            }
        }
    }
    
    //-------------instance method----------------
    /// 创建一个新的EventHandler实例 / Create a new EventHandler instance
    /// # Arguments
    /// * `ctx` - 上下文对象 / Context object
    /// * `initial_proc_index` - 初始流程索引 / Initial procedure index
    /// * `Vec<Option<Rc<dyn TState>>` - 流程列表 / Procedure list
    /// # Return
    /// * `GameResult<App>` - App实例 / App instance
    pub fn new(ctx: &mut Context,initial_proc_index:i32,procedure_list:Vec<Option<Rc<dyn TState>>>) -> GameResult<App> {

        Self::init_config_font(ctx);
        let app = App {
            _frames: 0,
            // _event_comp: EventComponent::new(),
            // _input_comp: InputComponent::new(),
            // _procedure_comp: ProcedureComponent::new(initial_proc_index, procedure_list),
             _elapsed_sec_from_last_frame: 0.0,
            _app_components : Some(AppComponents::new(initial_proc_index, procedure_list)),
        };
        
        return Ok(app);
    }
    
    /// 字体初始化配置 / Font initialization configuration
    /// # Arguments
    /// * `ctx` - 上下文对象 / Context object
    fn init_config_font(ctx : &mut Context) -> GameResult<()>{
        ctx.gfx.add_font(
            constant::FONT_NAME,
            graphics::FontData::from_path(ctx, constant::FONT_ASSET_PATH)?
        );
        return Ok(());
    }
    
    /// 获取初始化资源配置目录 / Get the initialized resource configuration directory
    /// # Return
    /// * `PathBuf` - 指定的资源目录 / Specified resource directory
    fn get_config_resource_dir() -> PathBuf{
        if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            let mut path = path::PathBuf::from(manifest_dir);
            path.push(constant::RESOURCE_DIR);
            return path;
        } 
        else {
            let base_path = Path::new("./");
            let resource_path = base_path.join(constant::RESOURCE_DIR);
            return path::PathBuf::from(resource_path);
        }
    }
    
    /// 主帧循环 / Main frame loop
    /// # Arguments
    /// * `delta_time` - 时间间隔 / Time interval
    fn main_update(&mut self, delta_time:f64){
        self._elapsed_sec_from_last_frame += delta_time;
        
        if(self._elapsed_sec_from_last_frame >= constant::APP_UPDATE_INTERVAL_1_SEC){
            let elapsed = mem::take(&mut self._elapsed_sec_from_last_frame);
            
            #[cfg(feature = "debug_log")]{
            }
        }
        
        #[cfg(feature = "debug_log")]{
        }
    }
}

////-------------impl EventHandler----------------
impl event::EventHandler<ggez::GameError> for App {
    
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        //主帧循环 / Main frame loop
        //将每秒钟的帧率限制在60，每过1/60秒更新一次
        while ctx.time.check_update_time(constant::APP_FPS) {
            self.main_update(ctx.time.delta().as_secs_f64());
        }
        
        return  Ok(());
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx, 
            graphics::Color::from([0.0, 0.0, 0.0, 1.0])
        );
        
        canvas.finish(ctx)?;
        // self.frames += 1;
        // if (self.frames % 300) == 0 {
        //     log(&self, &format!("FPS: {}",ctx.time.fps()), LogLevelEnum::Info);
        // }

        return Ok(());
    }
    
    /// 键盘按下输入处理 / Keyboard down input processing
    /// # Arguments
    /// * `ctx` - 上下文对象 / Context object
    /// * `input` - 键盘输入 / Keyboard input
    /// * `repeat` - 是否重复 / Whether repeat
    /// # Return
    /// * `GameResult` - 处理结果 / Processing result
    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, repeat: bool) -> GameResult {
        #[cfg(feature = "debug_log")]{
            log(self,&format!("Key pressed: keycode {:?},  repeat: {}", input.keycode, repeat),LogLevelEnum::Info);
        }
        Ok(())
    }
}

//-------------impl Debug----------------
impl fmt::Debug for App {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "App")
    }
}