use ggez::glam::{IVec2, ivec2, Vec2};
use crate::constant;
use crate::constant::BLOCK_MAX_OCCUPIED;
use crate::define::enum_define::{TetriminoColorEnum, TetriminoTypeEnum};
use crate::runtime::data::teri_grid::TetriGridCell;
use crate::tools::logger::{log, LogLevelEnum};

/// 表示一个俄罗斯方块 / A tetrimino
#[derive(Debug,Clone)]
pub struct Tetrimino{
    /// 方块类型 / type
    _tetri_type : TetriminoTypeEnum,
    /// 方块当前的角度
    _curr_angle : u16,
    /// 每个方块的最小单位，表示在游玩区域中的坐标位置 / the minimum unit of each block, indicating the coordinate position in the play area
    _minos : Vec<IVec2>,

}

impl Tetrimino{

    /// 更改自身的属性，成为一个新的方块 / change its own attributes to become a new block
    /// #Arguments
    /// * blocl_area - 游戏区域的方块数据 / block data of the game area
    /// #Return
    /// * 返回是否生成成功 / return whether it is generated successfully
    pub fn gen_as_new(&mut self,blocl_area:&[[TetriGridCell;constant::BLOCK_AREA_MAX_HORIZONTAL_BLOCK_CNT];constant::BLOCK_AREA_MAX_VERTICAL_BLOCK_CNT]) -> bool{

    }
    
    /// 获取占位方块在grid坐标中的坐标位置 / get the coordinate position of the block in the grid coordinate
    /// #Return
    /// * 返回占位方块在grid坐标中的坐标位置 / return the coordinate position of the block in the grid coordinate
    pub fn block_actual_coord(&mut self) -> &Vec<IVec2>{
        return &self._minos;
    }

    /// 旋转当前方块 / rotate the current block
    /// #Arguments
    /// * direction - 旋转方向方块，true表示顺时针，false表示逆时针 / rotation direction of the block, true means clockwise, false means counterclockwise
    pub fn rotate(&mut self, clock_wise : bool) -> bool
    {
        let old_angle = self._curr_angle;
        if clock_wise{
            self._curr_angle = if self._curr_angle == 270{ 0 } else { self._curr_angle + 90 };
        }
        else {
            self._curr_angle = if self._curr_angle == 0 { 270 } else { self._curr_angle - 90 };
        }
        
        let mut is_negate = 0;
        let tetri_offsets: &[IVec2];

        match self._tetri_type {
            TetriminoTypeEnum::RightSnake =>{
                is_negate = if self._curr_angle == 0 || self._curr_angle == 180 {1} else {-1};
                tetri_offsets = &constant::TETRI_OFFSET_RIGHT_SNAKE;
                // let mut i : usize = 0;
                //     for offset in constant::TETRI_OFFSET_RIGHT_SNAKE {
                //         self._minos[i].x = offset.x * is_negate + self._minos[i].x;
                //         self._minos[i].y = offset.y * is_negate + self._minos[i].y;
                //         i += 1;
                //     }
            }//right sanke

            TetriminoTypeEnum::LeftSnake => {
                is_negate = if self._curr_angle == 0 || self._curr_angle == 180 {1} else {-1};
                tetri_offsets = &constant::TETRI_OFFSET_LEFT_SNAKE;
                // let mut i : usize = 0;
                //     for offset in constant::TETRI_OFFSET_LEFT_SNAKE {
                //         self._minos[i].x = offset.x * is_negate + self._minos[i].x;
                //         self._minos[i].y = offset.y * is_negate + self._minos[i].y;
                //         i += 1;
                //     }
            }

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
                    _ => {
                        log("Tetrimino.rs",&format!("invalid old_angle, old_angle: {} , tetri type : {:?}", old_angle,self._tetri_type),LogLevelEnum::Fatal);
                        panic!();
                    }
                }
            }//T

            TetriminoTypeEnum::Stick => {
                is_negate = if self._curr_angle == 0 || self._curr_angle == 180 {1} else {-1};
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
    
    /// 更新方块在grid中的坐标位置 / update the coordinate position of the block in the grid
    /// #Arguments
    /// * offset - 偏移量 / offset
    pub fn update_coord(&mut self,offset:IVec2){
        for i in 0..self._minos.len(){
            self._minos[i].x += offset.x;
            self._minos[i].y += offset.y;
        }
    }
    
    /// 获取方块颜色 / get the block color
    #[inline]
    pub fn color(&self) -> TetriminoColorEnum{
        let index = self._tetri_type as usize;
        if index >= constant::BLOCK_COLOR_GEN_SEQUENCE.len(){
            log("Tetrimino.rs","color() ---> index out of range",LogLevelEnum::Fatal);
            panic!();
        }
        return constant::BLOCK_COLOR_GEN_SEQUENCE[index];
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
                // _mino_rotation_flag : false,
            };
            
            return Some(new_tetrimino);
        }
        else {
            log("Tetrimino.rs","new() ---> tetri enum is none",LogLevelEnum::Fatal);
            panic!();
        }

        // let tetri_enum = TetriminoTypeEnum::try_from(tetri_type);
        // if let Ok(tetri_type) = tetri_enum {
        //     let index_need_to_spotted = Self::get_spotted_idx(&tetri_enum.unwrap());
        //     let mut tetrimino = Tetrimino{
        //         _tetri_type          : tetri_type,
        //         _occupied_coord      : [[0;BLOCK_MAX_OCCUPIED];BLOCK_MAX_OCCUPIED],
        //         _occupied_index      : Vec::new(),
        //         _occupied_actual_pos : Vec::new(),
        //         _coord               : IVec2::ZERO,
        //         _pos_change_flag     : false,
        //         _color               : 0,
        //     };
            
        //     Self::set_occupied(&mut tetrimino._occupied_coord, &mut tetrimino._occupied_index, index_need_to_spotted);
        //     return Some(tetrimino);
        // }
        // else{
        //     log("Tetrimino.rs","new() ---> tetri enum is none",LogLevelEnum::Fatal);
        //     panic!();
        // }
    }
    
    ///设置占位方块 / set the block
    pub fn set_occupied(
        occupied : &mut [[u8;BLOCK_MAX_OCCUPIED];BLOCK_MAX_OCCUPIED],
        occupied_index:&mut Vec<IVec2>,
        idx_need_to_spotted:Vec<IVec2>){
        
    }

    fn set_spotted_minos(&mut self, tetri_type : TetriminoTypeEnum,minos:&mut Vec<IVec2>){
        minos.clear();
        match tetri_type {
            TetriminoTypeEnum::Stick => {
                minos.push(ivec2(0,4));
                minos.push(ivec2(1,4));
                minos.push(ivec2(2,4));
                minos.push(ivec2(3,4));
            }
            TetriminoTypeEnum::LeftGun => {
                minos.push(ivec2(0,3));
                minos.push(ivec2(0,4));
                minos.push(ivec2(0,5));
                minos.push(ivec2(1,5));
            }
            TetriminoTypeEnum::RightGun => {
                minos.push(ivec2(0,3));
                minos.push(ivec2(0,4));
                minos.push(ivec2(0,5));
                minos.push(ivec2(1,0));
            }
            TetriminoTypeEnum::Square => {
                minos.push(ivec2(0,3));
                minos.push(ivec2(0,4));
                minos.push(ivec2(1,3));
                minos.push(ivec2(1,4));
            }
            TetriminoTypeEnum::RightSnake => {
                minos.push(ivec2(1,3));
                minos.push(ivec2(1,4));
                minos.push(ivec2(0,4));
                minos.push(ivec2(0,5));
            }
            TetriminoTypeEnum::LeftSnake => {
                minos.push(ivec2(0,3));
                minos.push(ivec2(0,4));
                minos.push(ivec2(1,4));
                minos.push(ivec2(1,5));
            }
            TetriminoTypeEnum::T => {
                minos.push(ivec2(0,3));
                minos.push(ivec2(0,4));
                minos.push(ivec2(0,5));
                minos.push(ivec2(1,3));
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
                        minos.push(ivec2(0,4));
                        minos.push(ivec2(1,4));
                        minos.push(ivec2(2,4));
                        minos.push(ivec2(3,4));
                    }
            TetriminoTypeEnum::LeftGun => {
                        minos.push(ivec2(1,5));
                        minos.push(ivec2(0,5));
                        minos.push(ivec2(0,4));
                        minos.push(ivec2(0,3));
                    }
            TetriminoTypeEnum::RightGun => {
                        minos.push(ivec2(1,0));
                        minos.push(ivec2(0,3));
                        minos.push(ivec2(0,4));
                        minos.push(ivec2(0,5));
                    }
            TetriminoTypeEnum::Square => {
                        minos.push(ivec2(0,3));
                        minos.push(ivec2(0,4));
                        minos.push(ivec2(1,3));
                        minos.push(ivec2(1,4));
                    }
            TetriminoTypeEnum::RightSnake => {
                        minos.push(ivec2(1,3));
                        minos.push(ivec2(1,4));
                        minos.push(ivec2(0,4));
                        minos.push(ivec2(0,5));
                    }
            TetriminoTypeEnum::LeftSnake => {
                        minos.push(ivec2(0,3));
                        minos.push(ivec2(0,4));
                        minos.push(ivec2(1,4));
                        minos.push(ivec2(1,5));
                    }
            TetriminoTypeEnum::T => {
                        minos.push(ivec2(0,3));
                        minos.push(ivec2(0,4));
                        minos.push(ivec2(0,5));
                        minos.push(ivec2(1,3));
                    }
            _ => {
                        log("Tetrimino.rs","get_spotted_minos() ---> tetri enum is none",LogLevelEnum::Fatal);
                        panic!();
                    }
        }

        return minos;
    }
}