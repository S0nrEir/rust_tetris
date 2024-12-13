use ggez::glam::{IVec2, Vec2};
use crate::constant::BLOCK_MAX_OCCUPIED;
use crate::define::enum_define::TetriminoTypeEnum;
use crate::tools::logger::{log, LogLevelEnum};

/// 表示一个俄罗斯方块 / A tetrimino
pub struct Tetrimino{
    /// 方块类型 / type
    _tetri_type : TetriminoTypeEnum,
    ///被占位的grid位置 / grid position occupied
    _occupied : [[bool;BLOCK_MAX_OCCUPIED];BLOCK_MAX_OCCUPIED],
    /// 距离中心点的偏移量 / offset from the center
    //_occupied : Vec<IVec2>,
    /// grid中的实际位置，方块的中心位置 / actual position in the grid, the center position of the block
    _pos : IVec2,
}

impl Tetrimino{
    
    /// 旋转方块 / rotate the block
    /// #Arguments
    /// * is_right - 是否向右旋转，如果为false则向左旋转 / whether to rotate to the right, if false, rotate to the left
    pub fn rotate(&mut self,is_right:bool){
        if(is_right){
            self._occupied.rotate_right(1);
        }
        else{
            self._occupied.rotate_left(1);
        }
    }
    
    /// 更新方块在grid中的坐标位置 / update the coordinate position of the block in the grid
    /// #Arguments
    /// * offset - 偏移量 / offset
    pub fn update_pos(&mut self,offset:IVec2){
        self._pos = self._pos + offset;
    }
    
    ///获取在pos在grid中的实际坐标位置 / get the actual position in the grid
    pub fn get_pos(&self) -> IVec2{
        return self._pos.clone();
    }
    
    pub fn new(tetri_type : isize) -> Option<Self> {
        let tetri_enum = TetriminoTypeEnum::try_from(tetri_type);
        if let Ok(tetri_type) = tetri_enum {
            return Some(Tetrimino{
                _tetri_type: tetri_type,
                _occupied : Self::gen_occupied(&tetri_type,),
                _pos : IVec2::ZERO
            });
        }
        else{
            return None;
        }
    }

    /// 标记占用范围 / mark occupied range
    /// #Arguments
    /// * tetri_type - 方块类型 / block type
    /// #Return
    /// * 占用范围 / occupied range
    // fn gen_occupied(tetri_type : &TetriminoTypeEnum) -> Vec<IVec2> {
    //     let mut occupied = Vec::new();
    //     match tetri_type {
    //         TetriminoTypeEnum::Stick => {
    //             occupied.push(IVec2::new(0, 0));
    //             occupied.push(IVec2::new(1, 0));
    //             occupied.push(IVec2::new(2, 0));
    //             occupied.push(IVec2::new(3, 1));
    //         },
    //         TetriminoTypeEnum::LeftGun => {
    //             occupied.push(IVec2::new(1, 0));
    //             occupied.push(IVec2::new(1, 1));
    //             occupied.push(IVec2::new(2, 0));
    //             occupied.push(IVec2::new(2, 1));
    //         },
    //         TetriminoTypeEnum::RightGun => {
    //             occupied.push(IVec2::new(0, 0));
    //             occupied.push(IVec2::new(1, 0));
    //             occupied.push(IVec2::new(2, 0));
    //             occupied.push(IVec2::new(2, 1));
    //         },
    //         TetriminoTypeEnum::Square => {
    //             occupied.push(IVec2::new(0, 0));
    //             occupied.push(IVec2::new(0, 1));
    //             occupied.push(IVec2::new(1, 0));
    //             occupied.push(IVec2::new(1, 1));
    //         },
    //         TetriminoTypeEnum::RightSnake => {
    //             occupied.push(IVec2::new(0, 1));
    //             occupied.push(IVec2::new(0, 2));
    //             occupied.push(IVec2::new(1, 0));
    //             occupied.push(IVec2::new(1, 1));
    //         },
    //         TetriminoTypeEnum::LeftSnake => {
    //             occupied.push(IVec2::new(0, 0));
    //             occupied.push(IVec2::new(0, 1));
    //             occupied.push(IVec2::new(1, 1));
    //             occupied.push(IVec2::new(1, 2));
    //         },
    //         TetriminoTypeEnum::T => {
    //             occupied.push(IVec2::new(0, 0));
    //             occupied.push(IVec2::new(0, 1));
    //             occupied.push(IVec2::new(0, 2));
    //             occupied.push(IVec2::new(1, 1));
    //         },
    //         _ => {
    //             log("Tetrimino", "spot_occupied error", LogLevelEnum::Fatal);
    //             panic!();
    //         }
    //     }
    //     return occupied;
    // }
    
    fn gen_occupied(tetri_type : &TetriminoTypeEnum) -> [[bool; BLOCK_MAX_OCCUPIED]; BLOCK_MAX_OCCUPIED]{
        let mut occupied = [[false;BLOCK_MAX_OCCUPIED];BLOCK_MAX_OCCUPIED];
        match tetri_type {
            TetriminoTypeEnum::Stick => {
                occupied[0][0] = true;
                occupied[1][0] = true;
                occupied[2][0] = true;
                occupied[3][0] = true;
            },
            TetriminoTypeEnum::LeftGun => {
                occupied[0][1] = true;
                occupied[1][1] = true;
                occupied[2][1] = true;
                occupied[2][0] = true;
            },
            TetriminoTypeEnum::RightGun => {
                occupied[0][0] = true;
                occupied[1][0] = true;
                occupied[2][0] = true;
                occupied[2][1] = true;
            },
            TetriminoTypeEnum::Square => {
                occupied[0][0] = true;
                occupied[0][1] = true;
                occupied[1][0] = true;
                occupied[1][1] = true;
            },
            TetriminoTypeEnum::RightSnake => {
                occupied[0][1] = true;
                occupied[0][2] = true;
                occupied[1][0] = true;
                occupied[1][1] = true;
            },
            TetriminoTypeEnum::LeftSnake => {
                occupied[0][0] = true;
                occupied[0][1] = true;
                occupied[1][1] = true;
                occupied[1][2] = true;
            },
            TetriminoTypeEnum::T => {
                occupied[0][0] = true;
                occupied[0][1] = true;
                occupied[0][2] = true;
                occupied[1][1] = true;
            },
            _ => {
                log("Tetrimino","spot_occupied error",LogLevelEnum::Fatal);
                panic!();
            }
        }
        return occupied;
    }
}