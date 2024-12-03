mod examples;
mod t_state;
mod constant;
mod tools;
mod test;
mod t_updatable;
mod define;
mod runtime;
mod app;

use app::App;

fn main() {
    App::start_up();
}