use ggez::glam::{IVec2, ivec2, Vec2};
use crate::constant;
use crate::define::enum_define::TetriminoTypeEnum;
use crate::runtime::data::teri_grid::TetriGridCell;
use crate::runtime::data::tetrimino::Tetrimino;
use crate::tools;
use crate::tools::logger::*;

//横向10个方块，纵向20个方块
/// 游玩区域 / Play area
#[derive(Debug)]
pub struct PlayField {
    _block_arr : [[TetriGridCell;constant::BLOCK_AREA_MAX_HORIZONTAL_BLOCK_CNT];constant::BLOCK_AREA_MAX_VERTICAL_BLOCK_CNT],
    _curr_terimino : Option<Tetrimino>
}

//------------------------------instance function------------------------------
impl PlayField {
    
    /// 获取当前的方块类型 / get the current tetrimino type
    /// #Return
    /// * 返回当前的方块类型 / return the current tetrimino type
    pub fn get_curr_tetrimino_type(&self) -> &TetriminoTypeEnum{
        if(self._curr_terimino.is_none()){
            return &TetriminoTypeEnum::None;
        }
        return self._curr_terimino.as_ref().unwrap().get_type();
    }
    
    /// 初始化方块 / Initialize tetrimino
    pub fn init_tetrimino(&mut self){
        self._curr_terimino = tools::tetri_tools::gen_rand_tetrimino();
        if(self._curr_terimino.is_none()){
            log("play_field.rs","init_tetrimino() ---> curr_tetrimino is none",LogLevelEnum::Fatal);
            panic!();
        }
    }
    
    /// 初始化区块数据，包含坐标和实际的位置 / Initialize block data, including coordinates and actual positions
    pub fn init_field_data(&mut self){
        let mut x : f32 = 0.;
        let mut y : f32 = 0.;
        let mut init_coord = constant::BLOCK_INIT_START_COORD;
        for element in self._block_arr.iter_mut() {
            for block in element.iter_mut() {
                block.set_world_position(Vec2::new(init_coord.0, init_coord.1));
                block.set_coord(x as i32, y as i32);
                y += 1.;
                //y offset
                init_coord.1 = (x + 1.) * (constant::BLOCK_SIZE + constant::BLOCK_COORD_SPACING) as f32;
            }
            x += 1.;
            //x offset
            init_coord.0 = (y + 1.) * (constant::BLOCK_SIZE + constant::BLOCK_COORD_SPACING) as f32;
        }
    }
    
    /// 清理并重置放置区域的所有数据 / clear and reset all data of the placement area
    pub fn clear(&mut self){
    }
}

//------------------------------struct function------------------------------
impl PlayField {
    ///新建 / new
    pub fn new() -> Self{
        return PlayField {
            _block_arr : PlayField::gen_block_arr(),
            _curr_terimino : None
        };
    }
    
    fn gen_block_arr() -> [[TetriGridCell;constant::BLOCK_AREA_MAX_HORIZONTAL_BLOCK_CNT];constant::BLOCK_AREA_MAX_VERTICAL_BLOCK_CNT] {
        let mut block_arr = [[TetriGridCell::new(Vec2::new(0.0,0.0),IVec2::ZERO);constant::BLOCK_AREA_MAX_HORIZONTAL_BLOCK_CNT];constant::BLOCK_AREA_MAX_VERTICAL_BLOCK_CNT];
        for i in 0..constant::BLOCK_AREA_MAX_VERTICAL_BLOCK_CNT {
            for j in 0..constant::BLOCK_AREA_MAX_HORIZONTAL_BLOCK_CNT {
                block_arr[i][j] = TetriGridCell::new(Vec2::new(0.0,0.0),IVec2::new(i as i32,j as i32));
            }
        }
        return block_arr;
    }
}