use ggez::glam::{IVec2, ivec2, Vec2};
use crate::constant::BLOCK_MAX_OCCUPIED;
use crate::define::enum_define::TetriminoTypeEnum;
use crate::tools::logger::{log, LogLevelEnum};

/// 表示一个俄罗斯方块 / A tetrimino
#[derive(Debug)]
pub struct Tetrimino{
    /// 方块类型 / type
    _tetri_type : TetriminoTypeEnum,
    ///被占位的grid坐标位置 / grid coordinate position occupied
    _occupied_coord : [[u8;BLOCK_MAX_OCCUPIED];BLOCK_MAX_OCCUPIED],
    /// 被占位的grid位置的索引 / index of the grid position occupied
    _occupied_index : Vec<IVec2>,
    /// 方块在grid坐标系统中的实际位置 / actual position in the grid coordinate system
    _occupied_actual_pos : Vec<IVec2>,
    /// grid中的实际位置，方块的中心位置 / actual position in the grid, the center position of the block
    _coord : IVec2,
    /// 是否需要更新位置 / whether to update the position
    _pos_change_flag : bool,
}

impl Tetrimino{
    
    /// 获取方块的实际位置 / get the actual position of the block
    /// #Return
    /// * 返回方块的实际位置 / return the actual position of the block
    pub fn block_actual_pos(&mut self) -> &Vec<IVec2>{
        if(self._pos_change_flag){
            self.update_occupied();
            self._pos_change_flag = false;
        }
        return &self._occupied_actual_pos;
    }
    
    /// 旋转方块 / rotate the block
    /// #Arguments
    /// * turn_right - 是否向右旋转，如果为false则向左旋转 / whether to rotate to the right, if false, rotate to the left
    pub fn rotate(&mut self,turn_right:bool){
        if(turn_right){
            self._occupied_coord.rotate_right(1);
        }
        else{
            self._occupied_coord.rotate_left(1);
        }
        self._pos_change_flag = true;
    }
    
    /// 更新方块在grid中的坐标位置 / update the coordinate position of the block in the grid
    /// #Arguments
    /// * offset - 偏移量 / offset
    pub fn update_coord(&mut self,offset:IVec2){
        self._coord = self._coord + offset;
        // self._pos_change_flag = true;
    }
    
    ///获取在pos在grid中的实际坐标位置 / get the actual position in the grid
    pub fn get_pos(&self) -> &IVec2{
        return &self._coord;
    }

    pub fn get_type(&self) -> &TetriminoTypeEnum{
        return &self._tetri_type;
    }
    
    /// 清理数据 / clear data
    pub fn clear(&mut self){
        self._occupied_coord = [[0;BLOCK_MAX_OCCUPIED];BLOCK_MAX_OCCUPIED];
        self._occupied_index.clear();
        self._occupied_actual_pos.clear();
        self._coord = IVec2::ZERO;
        self._pos_change_flag = false;
    }
    
    /// 更新占位方块的坐标，下标索引 / 
    fn update_occupied(&mut self){
        self._occupied_index.clear();
        self._occupied_actual_pos.clear();
        for (i, row) in self._occupied_coord.iter().enumerate() {
            for (j, &is_block) in row.iter().enumerate() {
                
                if (is_block != 1) {
                    continue;
                }
                
                let xy = IVec2::new(i as i32,j as i32);
                self._occupied_index.push(IVec2::new(xy.x,xy.y));
                self._occupied_actual_pos.push(IVec2::new( self._coord.x + xy.x,self._coord.y + xy.y));
            }
       }
    }
    
    pub fn new(tetri_type : isize) -> Option<Self> {
        let tetri_enum = TetriminoTypeEnum::try_from(tetri_type);
        if let Ok(tetri_type) = tetri_enum {
            let occupied_and_index = Self::gen_occupied(&tetri_enum.unwrap()); 
            return Some(Tetrimino{
                _tetri_type          : tetri_type,
                _occupied_coord      : occupied_and_index.0,
                _occupied_index      : occupied_and_index.1,
                _occupied_actual_pos : Vec::new(),
                _coord               : IVec2::ZERO,
                _pos_change_flag     : false,
            });
        }
        else{
            log("Tetrimino.rs","new() ---> tetri enum is none",LogLevelEnum::Fatal);
            panic!();
        }
    }
    
    fn gen_occupied(tetri_type : &TetriminoTypeEnum) 
        -> ([[u8; BLOCK_MAX_OCCUPIED]; BLOCK_MAX_OCCUPIED],Vec<IVec2>){
        let mut occupied = [[0;BLOCK_MAX_OCCUPIED];BLOCK_MAX_OCCUPIED];
        let mut occupied_index = Vec::new();
        let mut index_need_to_spotted = Vec::new();
        match tetri_type {
            TetriminoTypeEnum::Stick => {
                index_need_to_spotted.push(IVec2::new(0, 0));
                index_need_to_spotted.push(IVec2::new(1, 0));
                index_need_to_spotted.push(IVec2::new(2, 0));
                index_need_to_spotted.push(IVec2::new(3, 0));
            },
            TetriminoTypeEnum::LeftGun => {
                index_need_to_spotted.push(IVec2::new(0, 1));
                index_need_to_spotted.push(IVec2::new(1, 1));
                index_need_to_spotted.push(IVec2::new(2, 1));
                index_need_to_spotted.push(IVec2::new(2, 0));
            },
            TetriminoTypeEnum::RightGun => {
                index_need_to_spotted.push(IVec2::new(0, 0));
                index_need_to_spotted.push(IVec2::new(1, 0));
                index_need_to_spotted.push(IVec2::new(2, 0));
                index_need_to_spotted.push(IVec2::new(2, 1));
            },
            TetriminoTypeEnum::Square => {
                index_need_to_spotted.push(IVec2::new(0, 0));
                index_need_to_spotted.push(IVec2::new(0, 1));
                index_need_to_spotted.push(IVec2::new(1, 0));
                index_need_to_spotted.push(IVec2::new(1, 1));
            },
            TetriminoTypeEnum::RightSnake => {
                index_need_to_spotted.push(IVec2::new(0, 1));
                index_need_to_spotted.push(IVec2::new(0, 2));
                index_need_to_spotted.push(IVec2::new(1, 0));
                index_need_to_spotted.push(IVec2::new(1, 1));
                
            },
            TetriminoTypeEnum::LeftSnake => {
                index_need_to_spotted.push(IVec2::new(0, 0));
                index_need_to_spotted.push(IVec2::new(0, 1));
                index_need_to_spotted.push(IVec2::new(1, 1));
                index_need_to_spotted.push(IVec2::new(1, 2));
            },
            TetriminoTypeEnum::T => {
                index_need_to_spotted.push(IVec2::new(0, 0));
                index_need_to_spotted.push(IVec2::new(0, 1));
                index_need_to_spotted.push(IVec2::new(0, 2));
                index_need_to_spotted.push(IVec2::new(1, 1));
            },
            _ => {
                log("Tetrimino.rs","spot_occupied() ---> error",LogLevelEnum::Fatal);
                panic!();
            }
        }
        for index in index_need_to_spotted.iter(){
            occupied[index.x as usize][index.y as usize] = 1;
            occupied_index.push(index.clone());
        }
        return (occupied,occupied_index);
    }
}