mod examples;
mod t_state;
mod constant;
mod tools;
mod test;
mod t_updatable;
mod define;
mod runtime;
mod main_state;

use std::{env, path};
use crate::tools::Logger::*;
use ggez::{
    event,
    glam::*,
    graphics::{self, Color},
    Context, GameResult,
};
use main_state::MainState;


fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } 
    else {
        path::PathBuf::from("./resources")
    };

    let context_builder = ggez::ContextBuilder::new("rust_tetris", "yhc")
        .add_resource_path(resource_dir);
    let build_result = context_builder.build();
    let (mut context, event_loop);
    match build_result {
        Ok(context_builder) => {
            context = context_builder.0;
            event_loop = context_builder.1;
        }
        Err(error) => {
            println!("{}",&error );
            return;
        }
    }
    
    let state = MainState::new(&mut context).unwrap();
    event::run(context, event_loop, state);

}