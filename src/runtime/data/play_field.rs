﻿use std::arch::x86_64::_MM_SET_FLUSH_ZERO_MODE;
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
    
    /// 获取方块区域 / get block area
    /// #Return
    /// * 返回方块区域 / return block area
    pub fn block_area(&self) -> &[[TetriGridCell;constant::BLOCK_AREA_MAX_HORIZONTAL_BLOCK_CNT];constant::BLOCK_AREA_MAX_VERTICAL_BLOCK_CNT]{
        return &self._block_arr;
    }
    
    /// 尝试旋转当前方块，如果旋转方块成功且无占位不冲突，则将更新grid area占位情况和对应的tetri / Try to rotate the current block, if the rotation block is successful and there is no conflict with the occupancy, the occupancy situation of the grid area and the corresponding tetri will be updated
    /// #Arguments
    /// * `turn_right` - 是否向右旋转，如果为false则向左旋转 / whether to rotate to the right, if false, rotate to the left
    /// #Return
    /// * 是否旋转成功 / whether the rotation is successful
    pub fn try_rotate_tetrimino(&mut self,turn_right:bool) -> bool{
        let mut rotate_succ = true;
        match self._curr_terimino { 
            Some(ref mut curr_terimino) => {
                let old_tetrimino = curr_terimino.clone();
                //turn right
                if(turn_right){
                    curr_terimino.rotate(true);
                }
                //turn left
                else { 
                    curr_terimino.rotate(false);
                }
                
                let block_actual_coord = curr_terimino.block_actual_coord();
                for coord in block_actual_coord.iter() {
                    //检查curr tetri更新后，占位坐标点在grid坐标系的位置中，是否已经被占用
                    if(self._block_arr[coord.x as usize][coord.y as usize].is_occupied()){
                        // rotate_succ = false;
                        // break;
                        *curr_terimino = old_tetrimino;
                        return false;
                    }
                }//end for
                
                for coord in block_actual_coord.iter(){
                    self._block_arr[coord.x as usize][coord.y as usize].set_occupied(1);
                }
            },
            None => {
                log("play_field.rs","curr tetrimino is none",LogLevelEnum::Error);
                return false;
            }
        }
        
        return true;
    }
    
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
        
        if let Some(ref mut curr_tetri) = self._curr_terimino{
            curr_tetri.clear();
        }
        
        for element in self._block_arr.iter_mut() {
            for block in element.iter_mut() {
                block.clear();
            }
        }
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