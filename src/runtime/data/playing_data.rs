/// 运行时玩家数据 / Runtime playing data
#[derive(Debug)]
pub struct PlayingData{
    /// 分数 / score
    _score : u32,
}

impl PlayingData {
    /// 创建一个新的PlayingData实例 / Create a new PlayingData instance
    pub fn new() -> Self{
        return PlayingData{
            _score:0,
        };
    }
    
    /// 获取分数 / Get score
    /// #Return
    /// * `u32` - 分数 / score
    pub fn get_score(&self) -> u32{
        return self._score;
    }
    
    /// 添加分数 / Add score
    /// #Arguments
    /// * `score_to_add` - 要添加的分数 / score to add
    pub fn add_score(&mut self,score_to_add:u32){
        self._score += score_to_add;
    }
    
    /// 清理数据 / Clear data
    pub fn clear(&mut self){
        self._score = 0;
    }
}