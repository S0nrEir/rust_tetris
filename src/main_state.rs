use ggez::{event,graphics::{self},Context, GameResult};
use crate::tools::Logger::*;

///游戏状态 / MainState
pub struct MainState {
    frames: usize,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        ctx.gfx.add_font(
            "consola",
            graphics::FontData::from_path(ctx, "/consola.ttf")?,
        );

        let s = MainState { frames: 0 };
        return Ok(s);
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
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
            log("MainState", &format!("FPS: {}",ctx.time.fps()), LogLevelEnum::Info);
        }

        return Ok(());
    }
}