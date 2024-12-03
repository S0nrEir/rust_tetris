
//横向10个方块，纵向20个方块
/// 方块放置区域 / Block Area
#[derive(Debug)]
pub struct BlockArea{
    
}

//------------------------------instance function------------------------------
impl BlockArea {
    
    /// 清理并重置放置区域的所有数据 / clear and reset all data of the placement area
    pub fn clear(&mut self){
    }
}

//------------------------------struct function------------------------------
impl BlockArea {
    ///新建 / new
    pub fn new() -> Self{
        return BlockArea{};
    }
}