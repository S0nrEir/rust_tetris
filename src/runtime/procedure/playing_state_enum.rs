use std::fmt::Display;

/// 游玩状态枚举 / Playing State Enum
#[derive(Debug,PartialEq)]
pub enum PlayingStateEnum{
    /// 开始 / Start
    Start = 0,
    /// 下落中 / Falling
    Falling,
    /// 表现效果 / Performing
    Performing,
    /// 结算 / Settlement
    Settlement,
}

impl PlayingStateEnum {
    /// 将枚举转换为&str / Convert enum to &str
    pub fn as_str(&self) -> &str {
        match self {
            PlayingStateEnum::Start => "Start",
            PlayingStateEnum::Falling => "Falling",
            PlayingStateEnum::Performing => "Performing",
            PlayingStateEnum::Settlement => "Settlement",
            _ => "Unknown",
        }
    }    
}

impl Display for PlayingStateEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}