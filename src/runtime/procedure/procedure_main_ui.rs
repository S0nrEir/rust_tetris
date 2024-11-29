use crate::t_state::TState;
use crate::define::enum_define::ProcedureEnum;
use ggez::input::keyboard::KeyCode;

const MAX_ITEM_COUNT:i8 = 2;

///主UI流程 / 
/// main UI procedure
#[derive(Debug,Clone)]
pub struct ProcedureMainUI{
    selected_item_index: i8,
}

impl ProcedureMainUI {
    ///constructor
    pub fn new() -> Self {
        return ProcedureMainUI{
            selected_item_index: 0,
        };
    }
    
    ///选择项 / select item
    /// #Arguments
    /// * `move_offset` - 索引偏移 / index offset
    fn select_item(&mut self, move_offset:i8) {
        let new_index = self.selected_item_index + move_offset;
        match new_index{
            MAX_ITEM_COUNT  =>{
                self.selected_item_index = 0;
            },
            -1 => {
                self.selected_item_index = MAX_ITEM_COUNT;
            },
            _ =>{
                self.selected_item_index = new_index;
            }
        }
    }
    
    /// 开始游戏 / start game
    fn start_game(&self){
        
    }
}

/// 实现状态机接口 /
/// implement state machine interface
impl TState for ProcedureMainUI{

    fn on_enter(&self) {
        
    }

    fn on_update(&mut self,_key_code: KeyCode) {
        
        match _key_code {
            KeyCode::Return => {
                self.start_game();
            },
            KeyCode::Up => {
                self.select_item(1);
            },
            KeyCode::Down => {
                self.select_item(-1);
            },
            _ => {},
        }
    }

    fn on_leave(&self) {
    }
    
    fn get_state(&self) -> ProcedureEnum {
        return ProcedureEnum::MainUI;
    }
}