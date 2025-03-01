﻿use std::{env, fmt, path};
use std::path::{Path, PathBuf};
use ggez::{Context, event, GameResult, graphics::{self}};
use ggez::conf::WindowMode;
use ggez::input::keyboard::{KeyCode, KeyInput};
use crate::constant;
use crate::define::enum_define::ProcedureEnum;
use crate::runtime::input::InputComponent;
use crate::runtime::procedure::{procedure_main_ui, procedure_over, procedure_playing, ProcedureComponent};
use crate::runtime::procedure::procedure_main_ui::ProcedureMainUIParam;
use crate::runtime::procedure::procedure_test_draw_block::{ProcedureTestDrawBlock, ProcedureTestDrawBlockParam};
use crate::t_state::TState;
use crate::t_updatable::{Updatable};
use crate::tools::logger::{log, LogLevelEnum};
use crate::tools::logger::LogLevelEnum::Fatal;

/// 游戏的主入口 / Main entry of the game
pub struct App {
    ///帧率 / Frame rate
    _frames : usize,
    _delta_sec: f32,
    _procedure_component: ProcedureComponent,
    _input_component: InputComponent,
}

//实现EventHandler trait以注册事件回调，以及子模块 / Implement the EventHandler trait to register event callbacks, as well as submodules
impl App {
    
    /// 启动 / Start
    pub fn start_up(){
        let context_builder = ggez::ContextBuilder::new(constant::APP_GAME_ID, constant::APP_AUTHOR_NAME)
            //resources
            .add_resource_path(Self::get_config_resource_dir())
            //window
            .window_mode(WindowMode::default().dimensions(constant::WINDOW_WIDTH, constant::WINDOW_HEIGHT));
        
        //get context builder & build app
        if let Ok((mut context, event_loop)) = context_builder.build() {
            
            #[cfg(feature = "debug_draw_block")]{
                let procedure_list: Vec<Option<Box<dyn TState>>> = vec![
                    Some(Box::new(ProcedureTestDrawBlock::new()))];

                if let Ok(mut app) = App::new(&mut context,procedure_list){
                    app._procedure_component.switch(ProcedureEnum::TestDrawBlock,Box::new(ProcedureTestDrawBlockParam::new()),None);
                    event::run(context, event_loop,app);
                }
                return;
            }
            
            //initial procedures
            let procedure_list: Vec<Option<Box<dyn TState>>> = vec![
                Some(Box::new(procedure_main_ui::ProcedureMainUI::new())),
                Some(Box::new(procedure_playing::ProcedurePlaying::new())),
                Some(Box::new(procedure_over::ProcedureOver::new()))];

            if let Ok(mut app) = App::new(&mut context,procedure_list){
                app._procedure_component.switch(ProcedureEnum::MainUI,Box::new(ProcedureMainUIParam::new()),None);
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
    pub fn new(ctx: &mut Context,procedure_list:Vec<Option<Box<dyn TState>>>) -> GameResult<App> {
        let _ = Self::init_config_font(ctx);
        let app = App {
            _frames: 0,
            _delta_sec: 0.0,
            _procedure_component:ProcedureComponent::new(procedure_list),
            _input_component:InputComponent::new(),
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
        return if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            let mut path = path::PathBuf::from(manifest_dir);
            path.push(constant::RESOURCE_DIR);
            path
        } else {
            let base_path = Path::new("./");
            let resource_path = base_path.join(constant::RESOURCE_DIR);
            path::PathBuf::from(resource_path)
        }
    }
    
    /// 主帧循环 / Main frame loop
    /// # Arguments
    /// * `delta_time` - 时间间隔 / Time interval4
    fn main_update(&mut self, ctx: &mut Context, key_code : Option<KeyCode>, delta_time:f64){
        let old_procedure = self._procedure_component.curr_procedure();
        let new_procedure = self._procedure_component.on_update(ctx,key_code,delta_time as f32);
        if !old_procedure.is_none() && !new_procedure.is_none() {
            let old_procedure = old_procedure.unwrap();
            let new_procedure = new_procedure.unwrap();
            if old_procedure != new_procedure {
                self._procedure_component.switch(new_procedure,Box::new(ProcedureMainUIParam::new()),None);
            }
        }
        else{
            log("app.rs","main_update() ---> return a none procedure enum!",Fatal);
            panic!();
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
            
            self.main_update(
                ctx,
                self._input_component.get_curr_input_key(),
                ctx.time.delta().as_secs_f64()
            );
            
        }
        return  Ok(());
    }
    
    /// 绘制接口回调 / Draw interface callback
    /// # Arguments
    /// * `ctx` - 上下文对象 / Context object
    /// # Return
    /// * `GameResult` - 处理结果 / Processing result
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let _ = self._procedure_component.on_draw(ctx);
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
        self._input_component.set_curr_input_key(input.keycode);
        
        #[cfg(feature = "debug_log")]{
            log(&self,&format!("Key pressed: keycode {:?},  repeat: {}", input.keycode, repeat),LogLevelEnum::Info);
        }
        
        return Ok(());
    }
}

//-------------impl Debug----------------
impl fmt::Debug for App {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "App")
    }
}