use core::panic;

use ggez::glam::{IVec2, ivec2, Vec2};
use rand::Rng;
use crate::constant;
use crate::constant::BLOCK_MAX_OCCUPIED;
use crate::define::enum_define::{TetriminoColorEnum, TetriminoTypeEnum};
use crate::runtime::data::teri_grid::TetriGridCell;
use crate::tools::logger::{log, LogLevelEnum};

use super::play_field::PlayField;

/// 表示一个俄罗斯方块 / A tetrimino
#[derive(Debug,Clone)]
pub struct Tetrimino{
    /// 方块类型 / type
    _tetri_type : TetriminoTypeEnum,
    /// 方块当前的角度
    _curr_angle : u16,
    /// 每个方块的最小单位，表示在游玩区域中的坐标位置 / the minimum unit of each block, indicating the coordinate position in the play area
    _minos : Vec<IVec2>,
    _color_index : usize,
}

impl Tetrimino{

    /// 更改自身的属性，成为一个新的方块 / change its own attributes to become a new block
    /// #Arguments
    /// * blocl_area - 游戏区域的方块数据 / block data of the game area
    /// #Return
    /// * 返回一个元组，第一个值表示生成是否成功，第二个值表示失败原因 / return a tuple, the first value indicates whether the generation was successful, and the second value indicates the reason for failure
    pub fn gen_as_new(&mut self,blocl_area:&[[TetriGridCell;constant::PLAY_FIELD_RAWS];constant::PLAY_FIELD_COLS]) -> (bool,String) {

        let mut gen_succ = true;
        let mut failed_msg: String = String::new();
        let mut rand = rand::thread_rng();
        let rand_type = rand.gen_range(TetriminoTypeEnum::get_min_max_range());
        let new_tetri_type = TetriminoTypeEnum::try_from(rand_type);

        if let Ok(tetri_type) = new_tetri_type{
            self.clear();
            self._tetri_type = tetri_type;
            Self::set_spotted_minos( tetri_type, &mut self._minos );
            let detected_collision = PlayField::detect_tetrimino_collision(&blocl_area,&self._minos);
            self._color_index = tetri_type as usize;
            gen_succ |= detected_collision;
            if detected_collision {
                failed_msg = format!("gen_as_new() ---> tetri type : {:?} , collision detected",self._tetri_type);
            }

            return (gen_succ , failed_msg);
        }
        else {
            log("Tetrimino.rs","gen_as_new() ---> new tetri type is none",LogLevelEnum::Fatal);
            panic!("gen_as_new() ---> tetri enum is none");
        }

    }
    
    /// 获取占位方块在grid坐标中的坐标位置 / get the coordinate position of the block in the grid coordinate
    /// #Return
    /// * 返回占位方块在grid坐标中的坐标位置 / return the coordinate position of the block in the grid coordinate
    pub fn block_actual_coord(&mut self) -> &Vec<IVec2>{
        return &self._minos;
    }

    /// 旋转当前方块 / rotate the current block
    /// #Arguments
    /// * clock_wise - 是否顺时针旋转 / whether to rotate clockwise
    /// #Return
    /// * 返回是否旋转成功 / return whether the rotation was successful
    pub fn rotate(&mut self, clock_wise : bool) -> bool
    {
        let old_angle = self._curr_angle;
        if clock_wise{
            self._curr_angle = if self._curr_angle == 270{ 0 } else { self._curr_angle + 90 };
        }
        else {
            self._curr_angle = if self._curr_angle == 0 { 270 } else { self._curr_angle - 90 };
        }
        
        let is_negate = self.negate(&self._tetri_type,clock_wise, self._curr_angle, old_angle);
        let tetri_offsets: &[IVec2];
        match self._tetri_type {
            TetriminoTypeEnum::RightSnake =>{
                tetri_offsets = &constant::TETRI_OFFSET_RIGHT_SNAKE;
            }//right sanke

            TetriminoTypeEnum::LeftSnake => {
                tetri_offsets = &constant::TETRI_OFFSET_LEFT_SNAKE;
            }//left snake

            TetriminoTypeEnum::RightGun => {
                match old_angle {
                    0 => {
                        tetri_offsets = if clock_wise {&constant::TETRI_OFFSET_RIGHT_GUN_0_90} else {&constant::TETRI_OFFSET_RIGHT_GUN_270_0};
                    }
                    90 =>{
                        tetri_offsets = if clock_wise {&constant::TETRI_OFFSET_RIGHT_GUN_90_180} else {&constant::TETRI_OFFSET_RIGHT_GUN_0_90};
                    }
                    180 => {
                        tetri_offsets = if clock_wise {&constant::TETRI_OFFSET_RIGHT_GUN_180_270} else {&constant::TETRI_OFFSET_RIGHT_GUN_90_180};
                    }
                    270 => {
                        tetri_offsets = if clock_wise {&constant::TETRI_OFFSET_RIGHT_GUN_270_0} else {&constant::TETRI_OFFSET_RIGHT_GUN_180_270};
                    }
                    _ =>{
                        log("Tetrimino.rs",&format!("invalid old_angle, old_angle: {} , tetri type : {:?}", old_angle,self._tetri_type),LogLevelEnum::Fatal);
                        panic!();
                    }

                }//match curr_angle
            }//left gun

            TetriminoTypeEnum::LeftGun => {
                match old_angle {
                     0 =>{
                        tetri_offsets = if clock_wise {&constant::TETRI_OFFSET_LEFT_GUN_0_90} else {&constant::TETRI_OFFSET_LEFT_GUN_270_0};
                     }
                     90 => {
                        tetri_offsets = if clock_wise {&constant::TETRI_OFFSET_LEFT_GUN_90_180} else {&constant::TETRI_OFFSET_LEFT_GUN_0_90};
                     }
                     180 => {
                        tetri_offsets = if clock_wise {&constant::TETRI_OFFSET_LEFT_GUN_180_270} else {&constant::TETRI_OFFSET_LEFT_GUN_90_180};
                     }
                     270 => {
                        tetri_offsets = if clock_wise {&constant::TETRI_OFFSET_LEFT_GUN_270_0} else {&constant::TETRI_OFFSET_LEFT_GUN_180_270};
                     }
                    _=> {
                        log("Tetrimino.rs",&format!("invalid old_angle, old_angle: {} , tetri type : {:?}", old_angle,self._tetri_type),LogLevelEnum::Fatal);
                        panic!();
                    }
                }//match curr_angle
            }//right gun

            TetriminoTypeEnum::T => {
                match  old_angle {
                    0 => {
                        tetri_offsets = if clock_wise {&constant::TETRI_OFFSET_T_0_90} else {&constant::TETRI_OFFSET_T_270_0};
                    }
                    90 => {
                        tetri_offsets = if clock_wise {&constant::TETRI_OFFSET_T_90_180} else {&constant::TETRI_OFFSET_T_0_90};
                    }
                    180 => {
                        tetri_offsets = if clock_wise {&constant::TETRI_OFFSET_T_180_270} else {&constant::TETRI_OFFSET_T_90_180};
                    }
                    270 => {
                        tetri_offsets = if clock_wise {&constant::TETRI_OFFSET_T_270_0} else {&constant::TETRI_OFFSET_T_180_270};
                    }
                    _ => {
                        log("Tetrimino.rs",&format!("invalid old_angle, old_angle: {} , tetri type : {:?}", old_angle,self._tetri_type),LogLevelEnum::Fatal);
                        panic!();
                    }
                }
            }//T

            TetriminoTypeEnum::Stick => {
                tetri_offsets = &constant::TETRI_OFFSET_STICK;
            }//stick

            //square啥也不用干 / square doesn't need to do anything
            TetriminoTypeEnum::Square =>{
                return true;
            }//square

            _=>{
                log("Tetrimino.rs","invalid tetri type",LogLevelEnum::Fatal);
                panic!();
            }
        }
        
        let mut i : usize = 0;
        for offset in tetri_offsets {
            self._minos[i].x = offset.x * is_negate + self._minos[i].x;
            self._minos[i].y = offset.y * is_negate + self._minos[i].y;
            i += 1;
        }
        return true;
    }
    
    /// 获取计算旋转时坐标的偏移量 / get the offset for calculating the coordinates when rotating
    /// #Arguments
    /// * tetri_type - 方块类型 / block type
    /// * clock_wise - 是否顺时针旋转 / whether to rotate clockwise
    /// * curr_angle - 当前角度 / current angle
    /// * old_angle - 旧角度 / old angle
    /// #Return
    /// * 返回偏移量 / return the offset
    fn negate(&self,tetri_type : &TetriminoTypeEnum , clock_wise : bool , curr_angle : u16 , old_angle : u16) -> i32{
        match tetri_type {
            TetriminoTypeEnum::RightSnake | TetriminoTypeEnum::LeftSnake =>{
                return if curr_angle == 90 || curr_angle == 270 {1} else {-1};
            }
            TetriminoTypeEnum::Stick => {
                return if curr_angle == 90 || curr_angle == 270 {1} else {-1};
            }
            _ => {
                return if clock_wise {1} else {-1};
            }
        }
    }
    
    /// 更新方块在grid中的坐标位置 / update the coordinate position of the block in the grid
    /// #Arguments
    /// * offset - 偏移量 / offset
    /// #Return
    /// * 返回是否更新成功 / return whether the update was successful
    #[inline]
    pub fn update_coord(&mut self,offset:IVec2) -> bool{
        let mut new_minos : Vec<IVec2> = Vec::new();
        for i in 0..self._minos.len(){
            let new_x = self._minos[i].x + offset.x;
            let new_y = self._minos[i].y + offset.y;
            if new_x < 0 || new_x >= constant::PLAY_FIELD_COLS as i32 || new_y < 0 || new_y >= constant::PLAY_FIELD_RAWS as i32 {
                return false;
            }
            new_minos.push(ivec2(new_x, new_y));
        }

        self._minos = new_minos;
        return true;
    }
    
    pub fn update_coord_by_vec2(&mut self,new_coords:Vec<IVec2>) -> bool{
        if new_coords.len() != self._minos.len() {
            log("Tetrimino.rs","update_coord_by_vec2() ---> new coords length is not equal to minos length",LogLevelEnum::Error);
            return false;
        }
        
        self._minos = new_coords;
        return true;
    }
    
    /// 获取方块颜色 / get the block color
    #[inline]
    pub fn color(&self) -> TetriminoColorEnum{
        if self._color_index >= constant::BLOCK_COLOR_GEN_SEQUENCE.len(){
            log("Tetrimino.rs","color() ---> index out of range",LogLevelEnum::Fatal);
            panic!();
        }
        return constant::BLOCK_COLOR_GEN_SEQUENCE[self._color_index];
    }
    
    /// 获取方块类型 / get the block type
    #[inline]
    pub fn get_type(&self) -> &TetriminoTypeEnum{
        return &self._tetri_type;
    }
    
    /// 清理数据 / clear data
    #[inline]
    pub fn clear(&mut self){
        self._curr_angle = 0;
        self._minos.clear();
        self._tetri_type = TetriminoTypeEnum::None;
    }
    
    /// 更新占位方块的坐标在grid坐标系统中的下标索引 / update the index of the occupied block coordinate in the grid coordinate system
    fn update_occupied(&mut self){
    }
    
    pub fn new(tetri_type : isize) -> Option<Self> {
        let tetri_enum = TetriminoTypeEnum::try_from(tetri_type);
        if let Ok(tetri_type) = tetri_enum{
            let mut new_tetrimino = Tetrimino{
                _tetri_type : tetri_type,
                _curr_angle : 0,
                _minos : Self::get_spotted_minos(tetri_type),
                _color_index : tetri_type as usize,
            };
            
            return Some(new_tetrimino);
        }
        else {
            log("Tetrimino.rs","new() ---> tetri enum is none",LogLevelEnum::Fatal);
            panic!();
        }
    }
    
    //#todo set和get冲突了，没有必要的逻辑
    /// 重新设置方块到顶端 / reset the block to the top
    fn set_spotted_minos(tetri_type : TetriminoTypeEnum,minos:&mut Vec<IVec2>){
        minos.clear();
        match tetri_type {
            TetriminoTypeEnum::Stick => {
                minos.push(ivec2(4,0));
                minos.push(ivec2(4,1));
                minos.push(ivec2(4,2));
                minos.push(ivec2(4,3));
            }
            TetriminoTypeEnum::LeftGun => {
                minos.push(ivec2(3,0));
                minos.push(ivec2(4,0));
                minos.push(ivec2(5,0));
                minos.push(ivec2(5,1));
            }
            TetriminoTypeEnum::RightGun => {
                minos.push(ivec2(3,0));
                minos.push(ivec2(4,0));
                minos.push(ivec2(5,0));
                minos.push(ivec2(3,1));
            }
            TetriminoTypeEnum::Square => {
                minos.push(ivec2(0,3));
                minos.push(ivec2(0,4));
                minos.push(ivec2(1,3));
                minos.push(ivec2(1,4));
            }
            TetriminoTypeEnum::RightSnake => {
                minos.push(ivec2(3,1));
                minos.push(ivec2(4,1));
                minos.push(ivec2(4,0));
                minos.push(ivec2(5,0));
            }
            TetriminoTypeEnum::LeftSnake => {
                minos.push(ivec2(3,0));
                minos.push(ivec2(4,0));
                minos.push(ivec2(4,1));
                minos.push(ivec2(5,1));
            }
            TetriminoTypeEnum::T => {
                minos.push(ivec2(3,0));
                minos.push(ivec2(4,0));
                minos.push(ivec2(5,0));
                minos.push(ivec2(4,1));
            }
            _ => {
                    log("Tetrimino.rs","set_spotted_minos() ---> tetri enum is none",LogLevelEnum::Fatal);
                    panic!();
            }
        }
    }

    fn get_spotted_minos(tetri_type : TetriminoTypeEnum) -> Vec<IVec2>{
        let mut minos : Vec<IVec2> = Vec::new();
        match tetri_type {
                    TetriminoTypeEnum::Stick => {
                        minos.push(ivec2(4,0));
                        minos.push(ivec2(4,1));
                        minos.push(ivec2(4,2));
                        minos.push(ivec2(4,3));
                    }
                    TetriminoTypeEnum::LeftGun => {
                        minos.push(ivec2(3,0));
                        minos.push(ivec2(4,0));
                        minos.push(ivec2(5,0));
                        minos.push(ivec2(5,1));
                    }
                    TetriminoTypeEnum::RightGun => {
                        minos.push(ivec2(3,0));
                        minos.push(ivec2(4,0));
                        minos.push(ivec2(5,0));
                        minos.push(ivec2(3,1));
                    }
                    TetriminoTypeEnum::Square => {
                        minos.push(ivec2(0,3));
                        minos.push(ivec2(0,4));
                        minos.push(ivec2(1,3));
                        minos.push(ivec2(1,4));
                    }
                    TetriminoTypeEnum::RightSnake => {
                        minos.push(ivec2(3,1));
                        minos.push(ivec2(4,1));
                        minos.push(ivec2(4,0));
                        minos.push(ivec2(5,0));
                    }
                    TetriminoTypeEnum::LeftSnake => {
                        minos.push(ivec2(3,0));
                        minos.push(ivec2(4,0));
                        minos.push(ivec2(4,1));
                        minos.push(ivec2(5,1));
                    }
                    TetriminoTypeEnum::T => {
                        minos.push(ivec2(3,0));
                        minos.push(ivec2(4,0));
                        minos.push(ivec2(5,0));
                        minos.push(ivec2(4,1));
                    }
            _ => {
                        log("Tetrimino.rs","get_spotted_minos() ---> tetri enum is none",LogLevelEnum::Fatal);
                        panic!();
                    }
        }

        return minos;
    }
}