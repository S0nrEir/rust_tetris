use ggez::glam::{IVec2, ivec2, Vec2};
use rand::Rng;
use crate::constant;
use crate::constant::BLOCK_MAX_OCCUPIED;
use crate::define::enum_define::TetriminoTypeEnum;
use crate::runtime::data::play_field::PlayField;
use crate::runtime::data::teri_grid::TetriGridCell;
use crate::tools::logger::{log, LogLevelEnum};

/// 表示一个俄罗斯方块 / A tetrimino
#[derive(Debug,Clone)]
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

    /// 更改自身的属性，成为一个新的方块 / change its own attributes to become a new block
    /// #Arguments
    /// * blocl_area - 游戏区域的方块数据 / block data of the game area
    /// #Return
    /// * 返回是否生成成功 / return whether it is generated successfully
    pub fn gen_as_new(&mut self,blocl_area:&[[TetriGridCell;constant::BLOCK_AREA_MAX_HORIZONTAL_BLOCK_CNT];constant::BLOCK_AREA_MAX_VERTICAL_BLOCK_CNT]) -> bool{
        let mut rand = rand::thread_rng();
        let rand_type = rand.gen_range(TetriminoTypeEnum::get_min_max_range());
        let new_tetri_type = TetriminoTypeEnum::try_from(rand_type);
        
        if let Ok(new_tetri_type) = new_tetri_type {
            self._tetri_type = new_tetri_type;
            let index_need_to_spotted = Self::get_spotted_idx(&new_tetri_type);
            Self::set_occupied(&mut self._occupied_coord, &mut self._occupied_index, index_need_to_spotted);
            //更新位置
            // let new_pos = rand.gen_range(0..constant::BLOCK_AREA_MAX_HORIZONTAL_BLOCK_CNT);
            //暂时放到中间
            let new_pos = constant::BLOCK_AREA_MAX_HORIZONTAL_BLOCK_CNT / 2;
            self._coord = IVec2::new(new_pos as i32,0);
            self._pos_change_flag = true;
            let actual_coords = self.block_actual_coord();
            let detected_collision = !PlayField::detect_tetrimino_collision(&blocl_area,&actual_coords);
            return !detected_collision;
        }
        else{
            log("Tetrimino.rs","gen_as_new() ---> new tetri type is none",LogLevelEnum::Fatal);
            panic!();
        }
    }
    
    /// 获取占位方块在grid坐标中的坐标位置 / get the coordinate position of the block in the grid coordinate
    /// #Return
    /// * 返回占位方块在grid坐标中的坐标位置 / return the coordinate position of the block in the grid coordinate
    pub fn block_actual_coord(&mut self) -> &Vec<IVec2>{
        if self._pos_change_flag {
            self.update_occupied();
            self._pos_change_flag = false;
        }
        return &self._occupied_actual_pos;
    }
    
    /// 获取向左/向右旋转后的方块坐标组
    /// #Arguments
    /// * turn_right - 是否向右旋转 / whether to rotate to the right
    /// #Return
    /// * 返回旋转后的方块坐标组 / return the rotated block coordinate group
    // pub fn get_rotated_block(&self,turn_right:bool) -> [[u8;BLOCK_MAX_OCCUPIED];BLOCK_MAX_OCCUPIED]{
    //     let mut rotated = self._occupied_coord.clone();
    //     let len = rotated.len(); 
    //     if len == 0 || rotated[0].len() == 0 {
    //         log("Tetrimino.rs","get_rotated_block() ---> rotated is empty",LogLevelEnum::Fatal);
    //         return [[0;BLOCK_MAX_OCCUPIED];BLOCK_MAX_OCCUPIED];
    //     }
    //     
    //     for i in 0..len{
    //         for j in 0..len{
    //             if turn_right {
    //                 rotated[j][len-1-i] = self._occupied_coord[i][j];
    //             }
    //             else{
    //                 rotated[len-1-j][i] = self._occupied_coord[i][j];
    //             }
    //         }
    //     }
    //     
    //     if turn_right {
    //         return rotated;
    //     }
    //     else{
    //         let mut rotated_left = [[0;BLOCK_MAX_OCCUPIED];BLOCK_MAX_OCCUPIED];
    //         for i in 0..BLOCK_MAX_OCCUPIED{
    //             for j in 0..BLOCK_MAX_OCCUPIED{
    //                 rotated_left[i][j] = rotated[j][BLOCK_MAX_OCCUPIED - 1 - i];
    //             }
    //         }
    //         return rotated_left;
    //     }
    // }
    
    /// 旋转方块 / rotate the block
    /// #Arguments
    /// * turn_right - 是否向右旋转，如果为false则向左旋转 / whether to rotate to the right, if false, rotate to the left
    pub fn rotate(&mut self,turn_right:bool){
        if turn_right {
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
        self._pos_change_flag = true;
        self._coord = self._coord + offset;
    }
    
    ///获取在pos在grid中的坐标位置 / get the actual position in the grid
    pub fn get_coord(&self) -> &IVec2{
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
    
    /// 更新占位方块的坐标在grid坐标系统中的下标索引 / update the index of the occupied block coordinate in the grid coordinate system
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
            let index_need_to_spotted = Self::get_spotted_idx(&tetri_enum.unwrap());
            let mut tetrimino = Tetrimino{
                _tetri_type          : tetri_type,
                _occupied_coord      : [[0;BLOCK_MAX_OCCUPIED];BLOCK_MAX_OCCUPIED],
                _occupied_index      : Vec::new(),
                _occupied_actual_pos : Vec::new(),
                _coord               : IVec2::ZERO,
                _pos_change_flag     : false,
            };
            
            Self::set_occupied(&mut tetrimino._occupied_coord, &mut tetrimino._occupied_index, index_need_to_spotted);
            return Some(tetrimino);
        }
        else{
            log("Tetrimino.rs","new() ---> tetri enum is none",LogLevelEnum::Fatal);
            panic!();
        }
    }
    
    ///设置占位方块 / set the block
    pub fn set_occupied(
        occupied : &mut [[u8;BLOCK_MAX_OCCUPIED];BLOCK_MAX_OCCUPIED],
        occupied_index:&mut Vec<IVec2>,
        idx_need_to_spotted:Vec<IVec2>){
        
        //reset
        for i in 0..BLOCK_MAX_OCCUPIED{
            for j in 0..BLOCK_MAX_OCCUPIED{
                occupied[i][j] = 0;
            }
        }
        occupied_index.clear();
        for index in idx_need_to_spotted.iter(){
            occupied[index.x as usize][index.y as usize] = 1;
            occupied_index.push(index.clone());
        }
    }
    
    /// 获取要标记的方块坐标下标集合 / get the set of block coordinate subscripts to be marked
    fn get_spotted_idx(tetri_type : &TetriminoTypeEnum) -> Vec<IVec2>{
        // let mut occupied = [[0;BLOCK_MAX_OCCUPIED];BLOCK_MAX_OCCUPIED];
        // let mut occupied_index = Vec::new();
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
        return index_need_to_spotted;
    }
}