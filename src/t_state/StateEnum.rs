use num_enum::IntoPrimitive;

///状态枚举

#[derive(IntoPrimitive,Copy,Clone)]
#[repr(i32)]
pub enum ProcedureEnum {
    ///主界面 / MainUI
    MainUI = 0,
    ///游玩中 / Playing
    Playing = 1,
    ///游戏结束 / Game Over
    Over = 2,
}