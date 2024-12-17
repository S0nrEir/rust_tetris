use ggez::glam::{IVec2, Vec2};
use ggez::glam::ivec2;

/// 表示游玩区域的单个区块 / Represents a single block in the play area
#[derive(Debug,Copy, Clone)]
pub struct TetriGridCell{
    /// 对应的世界坐标 / world coordinates
    _world_position : Vec2,
    /// 区块坐标 / block coordinates
    _coord : IVec2,
    /// 是否被占位的标记
    _occupied_flag : u8
}

impl TetriGridCell{
    
    /// 清理占位标记 / clear occupied flag
    pub fn clear(&mut self){
        self._occupied_flag = 0;
    }
    
    /// 设置该区块的占位情况 / set the occupancy of the block
    pub fn set_occupied(&mut self,occupied_flag:u8){
        self._occupied_flag = occupied_flag;
    }
    
    /// 设置世界坐标 / set world coordinates
    pub fn set_world_position(&mut self,world_position: Vec2){
        self._world_position = world_position;
    }
    
    /// 设置网格坐标 / set grid coordinates
    pub fn set_coord(&mut self,x:i32,y:i32){
        self._coord = IVec2::new(x,y);
    }
    
    /// 设置区块的世界坐标 / set the world coordinates of the block
    pub fn get_world_position(&self) -> &Vec2{
        return &self._world_position;
    }

    /// 区块的网格坐标 / block grid coordinates
    pub fn get_coord(&self) -> &IVec2{
        return &self._coord;
    } 

    pub fn new(world_postion: Vec2,coord:IVec2) -> Self{
        return TetriGridCell {
            _world_position: world_postion,
            _coord: coord,
            _occupied_flag: 0
        };
    }
}

