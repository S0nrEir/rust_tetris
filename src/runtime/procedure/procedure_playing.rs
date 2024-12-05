use ggez::{Context, GameResult};
use ggez::input::keyboard::KeyCode;
use crate::t_state::TState;
use crate::define::enum_define::ProcedureEnum;
use crate::runtime::data::play_field::PlayField;
use crate::runtime::procedure::t_procedure_param::ProcedureParam;
use crate::t_updatable::{Drawable, Tickable};
use crate::runtime::data::playing_data::PlayingData;

///游玩状态
/// playing state
#[derive(Debug)]
pub  struct ProcedurePlaying{
    _block_area: PlayField,
    _player_data : PlayingData,
}

impl Drawable for ProcedurePlaying {
    fn on_draw(&mut self, ctx: &mut Context) -> GameResult {
        todo!()
    }
}

impl Tickable for ProcedurePlaying {
    fn on_tick(&mut self, ctx: &mut Context, delta_time: f32, interval: f32) {
    }
}

impl TState for ProcedurePlaying{
    fn on_enter(&mut self,_param:Box<dyn ProcedureParam>){
        println!("ProcedurePlaying enter");
    }

    fn on_update(&mut self,key_code: KeyCode) {
        println!("ProcedurePlaying update");
    }

    fn on_leave(&mut self,_param:Option<Box<dyn ProcedureParam>>) {
        self._block_area.clear();
    }

    fn get_state(&self) -> ProcedureEnum {
        return ProcedureEnum::Playing;
    }
}

impl ProcedurePlaying {
    //--------new--------
    pub fn new() -> Self{
        return ProcedurePlaying{
            _block_area: PlayField::new(),
            _player_data:PlayingData::new(),
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
    
}