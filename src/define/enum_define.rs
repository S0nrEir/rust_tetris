use std::ops::Range;
use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};

const START_INDEX : isize = 0;
const END_INDEX   : isize = 7;

///状态枚举
#[derive(IntoPrimitive,Copy,Clone,Debug,PartialEq)]
#[repr(i32)]
pub enum ProcedureEnum {
    ///主界面 / MainUI
    MainUI = 0,
    ///游玩中 / Playing
    Playing,
    ///游戏结束 / Game Over
    Over,
    /// 测试绘制方块 / Test draw block
    TestDrawBlock,
    /// 测试绘制文本 / Test draw text
    TestDrawText,
}

impl ProcedureEnum {
    
    ///将枚举转换为&str / Convert enum to &str
    pub fn as_str(&self) -> &str {
        match self {
            ProcedureEnum::MainUI => "MainUI",
            ProcedureEnum::Playing => "Playing",
            ProcedureEnum::Over => "Over",
            ProcedureEnum::TestDrawBlock => "TestDrawBlock",
            ProcedureEnum::TestDrawText => "TestDrawText",
        }
    }
}

/// 方块类型 / Block type
#[derive(TryFromPrimitive,Copy,Clone,Debug)]
#[repr(isize)]
pub enum TetriminoTypeEnum{
    ///I
    Stick = START_INDEX,
    ///J
    LeftGun,
    ///L
    RightGun,
    ///O
    Square,
    ///S
    RightSnake,
    ///Z
    LeftSnake,
    ///T
    T,
    /// 未知
    None = END_INDEX,
}

impl TetriminoTypeEnum {
    ///将枚举转换为&str / Convert enum to &str
    pub fn as_str(&self) -> &str {
        match self {
            TetriminoTypeEnum::Stick => "Stick",
            TetriminoTypeEnum::LeftGun => "LeftGun",
            TetriminoTypeEnum::RightGun => "RightGun",
            TetriminoTypeEnum::Square => "Square", 
            TetriminoTypeEnum::RightSnake => "RightSnake",
            TetriminoTypeEnum::LeftSnake => "LeftSnake",
            TetriminoTypeEnum::T => "T",
            TetriminoTypeEnum::None => "None"
        }
    }
    
    /// 获取枚举的最小/最大范围值 / Get the minimum/maximum range value of the enumeration
    pub fn get_min_max_range() -> Range<isize>{
        return START_INDEX..END_INDEX;
    }
}

/// 当前控制的方块颜色 / Current controlled block color
#[derive(Debug, Clone, Copy)]
pub enum TetriminoColorEnum{
    Green = 0,
    Red,
    Blue,
    Yellow,
    Purple,
    Cyan,
}

/// 游玩区域方块颜色 / Play field block color
#[derive(Debug, Clone, Copy)]
pub enum PlayFieldColorEnum{
    BlockColor(TetriminoColorEnum),
    Black
}