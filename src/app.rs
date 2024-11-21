use std::{env, fmt, path};
use std::path::{Path, PathBuf};
use ggez::{event, graphics::{self}, Context, GameResult, timer};
use crate::tools::Logger::*;
use crate::constant;
use crate::constant::{APP_AUTHOR_NAME, APP_GAME_ID};
use crate::runtime::*;


/// 游戏的主入口 / Main entry of the game
pub struct App {
    ///帧率 / Frame rate
    frames : usize,
    
}

//实现EventHandler trait以注册事件回调，以及子模块 / Implement the EventHandler trait to register event callbacks, as well as submodules
impl App {
    
    /// 启动 / Start
    pub fn start_up(){
        let context_builder = ggez::ContextBuilder::new(APP_GAME_ID, APP_AUTHOR_NAME).add_resource_path(Self::get_config_resource_dir());
        //get context builder & build app
        if let Ok((mut context, event_loop)) = context_builder.build() {
            if let Ok(app) = App::new(&mut context) {
                event::run(context, event_loop,app);
            }
        }
    }
    
    //-------------instance method----------------
    /// 创建一个新的EventHandler实例 / Create a new EventHandler instance
    /// # Arguments
    /// * `ctx` - 上下文对象 / Context object
    /// # Return
    /// * `GameResult<App>` - App实例 / App instance
    pub fn new(ctx: &mut Context) -> GameResult<App> {
        Self::init_config_font(ctx);
        let app = Self::init_config_app();
        return Ok(app);
    }
    
    /// 游戏入口初始化配置 / Game entry initialization configuration
    fn init_config_app() -> Self{
        let app = App { frames: 0 };
        return  app;
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
}

////-------------impl EventHandler----------------
impl event::EventHandler<ggez::GameError> for App {
    
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        //主帧循环 / Main frame loop
        //将每秒钟的帧率限制在60，每过1/60秒更新一次
        while ctx.time.check_update_time(constant::APP_FPS) {
            
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
}

//-------------impl Debug----------------
impl fmt::Debug for App {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "App")
    }
}