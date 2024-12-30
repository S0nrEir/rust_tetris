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