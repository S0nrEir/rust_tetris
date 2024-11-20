use std::fmt;
use ggez::{event, graphics::{self}, Context, GameResult};
use crate::tools::Logger::*;

/// 游戏的主入口 / Main entry of the game
// #[derive(Debug)]
pub struct App {
    frames: usize,
}

//实现EventHandler trait以注册事件回调，以及子模块 / Implement the EventHandler trait to register event callbacks, as well as submodules
impl App {
    pub fn new(ctx: &mut Context) -> GameResult<App> {
        ctx.gfx.add_font(
            "consola",
            graphics::FontData::from_path(ctx, "/font/consola.ttf")?,
        );

        let s = App { frames: 0 };
        return Ok(s);
    }
}

impl event::EventHandler<ggez::GameError> for App {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        return Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx, 
            graphics::Color::from([0.1, 0.2, 0.3, 1.0])
        );
        
        canvas.finish(ctx)?;
        self.frames += 1;
        if (self.frames % 100) == 0 {
            log(&self, &format!("FPS: {}",ctx.time.fps()), LogLevelEnum::Info);
        }

        return Ok(());
    }
}

impl fmt::Debug for App {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "App")
    }
    
}