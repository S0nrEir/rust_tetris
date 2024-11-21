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

impl ProcedureEnum {
    
    ///将枚举转换为&str / Convert enum to &str
    pub fn as_str(&self) -> &str {
        match self {
            ProcedureEnum::MainUI => "MainUI",
            ProcedureEnum::Playing => "Playing",
            ProcedureEnum::Over => "Over",
        }
    }
}