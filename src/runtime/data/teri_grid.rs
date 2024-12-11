use ggez::glam::{IVec2, Vec2};
use ggez::glam::ivec2;

/// 游玩区域的单个小方块集合 / desc
#[derive(Debug,Copy, Clone)]
pub struct TetriGrid{
    /// 位置坐标 / coordinate
    _coord : Vec2,

    /// 区块位置 / block position
    _pos : IVec2,
}

impl TetriGrid{
    
    pub fn set_coord(&mut self,coord: Vec2){
        self._coord = coord;
    }
    
    pub fn set_pos(&mut self,x:i32,y:i32){
        self._pos = IVec2::new(x,y);
    }
    
    pub fn get_coord(&self) -> Vec2{
        return self._coord;
    }

    pub fn get_pos(&self) -> IVec2{
        return self._pos.clone();
    }

    pub fn new(coord: Vec2,pos:IVec2) -> Self{
        return TetriGrid {
            _coord: coord,
            _pos: pos,
        };
    }
}

