use crate::define::enum_define::TetriminoTypeEnum;

/// 表示一个俄罗斯方块 / A tetrimino
pub struct Tetrimino{
    /// 方块类型 / type
    _tetri_type : TetriminoTypeEnum,
}

impl Tetrimino{
    pub fn new (tetri_type:TetriminoTypeEnum) -> Self{
        return Tetrimino{
            _tetri_type : tetri_type,
        };
    }
}