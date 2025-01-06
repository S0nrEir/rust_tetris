use ggez::glam::{IVec2, Vec2};
use ggez::graphics::Color;
use crate::tools::logger::{log, LogLevelEnum};

/// 表示游玩区域的单个区块 / Represents a single block in the play area
#[derive(Debug,Copy, Clone)]
pub struct TetriGridCell{
    /// 对应的世界坐标 / world coordinates
    _world_position : Vec2,
    /// 区块坐标 / block coordinates
    _coord : IVec2,
    /// 是否被占位的标记
    _occupied_flag : u8,
    /// 方块颜色 / block color
    _color : Color,
}

impl TetriGridCell{
    
    /// 设置方块颜色 / set block color
    pub fn set_color(&mut self,color_to_set:Color){
        self._color = color_to_set;
    }
    
    /// 方块颜色 / block color
    pub fn color(&self) -> &Color{
        return &self._color;
    }
    
    /// 返回String形式的坐标 / Return the coordinate in String format
    #[inline]
    pub fn coord_as_string(&self) -> String{
        return format!("({}, {})",self._coord.x,self._coord.y);
    }
    
    /// 是否被占位 / is occupied
    /// #Return
    /// * 返回是否被占位 / return whether it is occupied
    #[inline]
    pub fn is_occupied(&self) -> bool{
        return self._occupied_flag == 1;
    }
    
    /// 清理占位标记 / clear occupied flag
    #[inline]
    pub fn clear(&mut self){
        self._occupied_flag = 0;
    }
    
    /// 设置该区块的占位情况 / set the occupancy of the block
    #[inline]
    pub fn set_occupied(&mut self,occupied_flag:u8){
        if occupied_flag != 0 && occupied_flag != 1 {
            log("TetriGridCell.rs","set_occupied() ---> occupied_flag is not 0 or 1",LogLevelEnum::Error);
            return;
        }
        self._occupied_flag = occupied_flag;
    }
    
    /// 设置世界坐标 / set world coordinates
    #[inline]
    pub fn set_world_position(&mut self,world_position: Vec2){
        self._world_position = world_position;
    }
    
    /// 设置网格坐标 / set grid coordinates
    #[inline]
    pub fn set_coord(&mut self,x:i32,y:i32){
        self._coord = IVec2::new(x,y);
    }
    
    /// 设置区块的世界坐标 / set the world coordinates of the block
    #[inline]
    pub fn get_world_position(&self) -> &Vec2{
        return &self._world_position;
    }

    /// 区块的网格坐标 / block grid coordinates
    #[inline]
    pub fn get_coord(&self) -> &IVec2{
        return &self._coord;
    }
    
    #[inline]
    pub fn new(world_position: Vec2,coord:IVec2) -> Self{
        return TetriGridCell {
            _world_position: world_position,
            _coord: coord,
            _occupied_flag: 0,
            _color : Color::BLACK,
        };
    }
}

