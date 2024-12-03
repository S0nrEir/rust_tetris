use num_enum::IntoPrimitive;

///状态枚举

#[derive(IntoPrimitive,Copy,Clone,Debug)]
#[repr(i32)]
pub enum ProcedureEnum {
    ///主界面 / MainUI
    MainUI = 0,
    ///游玩中 / Playing
    Playing,
    ///游戏结束 / Game Over
    Over,
    /// 测试绘制方块 / Test draw block
    TestDrawBlock
}

impl ProcedureEnum {
    
    ///将枚举转换为&str / Convert enum to &str
    pub fn as_str(&self) -> &str {
        match self {
            ProcedureEnum::MainUI => "MainUI",
            ProcedureEnum::Playing => "Playing",
            ProcedureEnum::Over => "Over",
            ProcedureEnum::TestDrawBlock => "TestDrawBlock"
        }
    }
}