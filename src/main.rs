mod examples;
mod t_state;
mod constant;
mod tools;
mod test;
mod t_updatable;
mod define;
mod runtime;
mod app;

use std::{env, path};
use crate::tools::Logger::*;
use ggez::{
    event,
    glam::*,
    graphics::{self, Color},
    Context, GameResult,
};
use app::App;


fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resource");
        path
    } 
    else {
        path::PathBuf::from("./resource")
    };

    //context对象包含与硬件交互的的所有状态
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
    
    let state = App::new(&mut context).unwrap();
    event::run(context, event_loop, state);

}