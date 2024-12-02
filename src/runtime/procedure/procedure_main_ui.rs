use crate::t_state::TState;
use crate::define::enum_define::ProcedureEnum;
use ggez::input::keyboard::KeyCode;
use crate::runtime::procedure::t_procedure_param::ProcedureParam;

const MAX_ITEM_COUNT:i8 = 2;

///主UI流程 / 
/// main UI procedure
#[derive(Debug)]
pub struct ProcedureMainUI{
    selected_item_index: i8,
    _param : Option<Box<dyn ProcedureParam>>,
}

impl ProcedureMainUI {
    ///constructor
    pub fn new() -> Self {
        return ProcedureMainUI{
            selected_item_index: 0,
            _param : None,
        };
    }
    
    /// 设置当前的选择索引 / set the current selection index
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

    fn on_enter(&self,_param:Option<Box<dyn ProcedureParam>>) {
        
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

    fn on_leave(&self,_param:Option<Box<dyn ProcedureParam>>) {
    }
    
    fn get_state(&self) -> ProcedureEnum {
        return ProcedureEnum::MainUI;
    }
}

#[derive(Debug)]
pub struct ProcedureMainUIParam{
    
}

impl ProcedureMainUIParam {
    pub fn new() -> Self {
        return ProcedureMainUIParam{};
    }
}

impl ProcedureParam for ProcedureMainUIParam {
}