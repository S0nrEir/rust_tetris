use ggez::glam::{IVec2, ivec2, Vec2};
use crate::constant;
use crate::constant::BLOCK_COORD_SPACING;
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
    
    /// 尝试下落方块，下落后同步更新grid / Try to drop the block, update the grid synchronously after the drop
    /// #Return
    /// * 是否下落成功 / whether the drop is successful
    pub fn try_fall_tetrimino(&mut self) -> bool{
        match self._curr_terimino{
            Some(ref mut curr_tetrimino) => {
                let mut block_actual_coords = curr_tetrimino.block_actual_coord().clone();
                //下落检查
                while true{
                    
                    for coords in &block_actual_coords {
                        let x = coords.x as usize;
                        let y = coords.y as usize + 1;

                        //到达底检查
                        if(y >= constant::BLOCK_AREA_MAX_HORIZONTAL_BLOCK_CNT){
                            Self::update_block_area(&block_actual_coords, 1, &mut self._block_arr);
                            log("play_field.rs","try_fall_tetrimino() ---> fall to bottom",LogLevelEnum::Info);
                            return true;
                        }

                        //下一格检查
                        if(self._block_arr[x][y].is_occupied()){
                            //如果下一格被阻挡，则停留在当前，否则一直循环检查直到底部
                            Self::update_block_area(&block_actual_coords, 1, &mut self._block_arr);
                            return true;
                        }
                        
                    }//end for
                    
                    //让每个坐标下降一格，然后进行新一轮检查
                    for coord in block_actual_coords.iter_mut(){
                        coord.y += 1;
                    }
                }//end while
                return true;
            }
            None => {
                log("play_field.rs","try_fall_tetrimino() ---> curr tetrimino is none",LogLevelEnum::Error);
                return false;
            }
        }//end match
    }
    
    
    /// 尝试水平移动方块，移动成功则同步更新grid / Try to move the block horizontally, if the move is successful, update the grid synchronously
    /// #Arguments
    /// * `move_right` - 是否向右移动，如果为false则向左移动 / whether to move to the right, if false, move to the left
    /// #Return
    /// * 是否移动成功 / whether the move is successful
    pub fn try_horizontal_move_tetrimino(&mut self,move_right:bool) -> bool{
        match self._curr_terimino {
            Some(ref mut curr_terimino) => {
                let block_horizontal_len = self._block_arr[0].len();
                let horizontal_offset = if move_right {1} else {-1};
                let tetrimino_actual_coord = curr_terimino.get_coord().clone();
                
                //检查左右超越边界
                if(tetrimino_actual_coord.x + horizontal_offset < 0){
                    log("play_field.rs","try_horizontal_move_tetrimino() ---> move left out of range",LogLevelEnum::Info);
                    return false;
                }
                if(tetrimino_actual_coord.x + horizontal_offset + constant::BLOCK_MAX_OCCUPIED as i32 >= block_horizontal_len as i32){
                    log("play_field.rs","try_horizontal_move_tetrimino() ---> move right out of range",LogLevelEnum::Info);
                    return false;
                }
                
                let old_actual_block_coords = curr_terimino.block_actual_coord();
                let mut new_actual_block_coords = old_actual_block_coords.clone();
                let mut temp_idx = 0;
                //检查左右移动后，是否有占位冲突
                for coord in old_actual_block_coords{
                    let x = coord.x + horizontal_offset;
                    let y = coord.y;
                    if(self._block_arr[x as usize][y as usize].is_occupied()){
                        log("play_field.rs",&format!("x:{},y:{} occupied",x,y),LogLevelEnum::Info);
                        return false;
                    }
                    new_actual_block_coords[temp_idx].x = x + horizontal_offset;
                    temp_idx = temp_idx + 1;
                }
                //没有冲突则更新占位情况
                //Self::update_block_area(&old_actual_block_coords, 0, &mut self._block_arr);
                Self::update_block_area(&new_actual_block_coords, 1, &mut self._block_arr);
                return true;
            },
            None => {
                log("play_field.rs","try_horizontal_move_tetrimino() ---> curr tetrimino is none",LogLevelEnum::Error);
                return false;
            }
        }//end match
    }

    /// 尝试旋转当前方块，如果旋转方块成功且无占位不冲突，则将更新grid area占位情况和对应的tetri / Try to rotate the current block, if the rotation block is successful and there is no conflict with the occupancy, the occupancy situation of the grid area and the corresponding tetri will be updated
    /// #Arguments
    /// * `turn_right` - 是否向右旋转，如果为false则向左旋转 / whether to rotate to the right, if false, rotate to the left
    /// #Return
    /// * 是否旋转成功 / whether the rotation is successful
    pub fn try_rotate_tetrimino(&mut self,turn_right:bool) -> bool{
        match self._curr_terimino { 
            Some(ref mut curr_terimino) => {
                let old_actual_block_coords = curr_terimino.block_actual_coord().clone();
                let old_tetrimino = curr_terimino.clone();
                //turn right
                if(turn_right){
                    curr_terimino.rotate(true);
                }
                //turn left
                else { 
                    curr_terimino.rotate(false);
                }
                
                let new_actual_block_coords = curr_terimino.block_actual_coord();
                for coord in new_actual_block_coords.iter() {
                    //检查curr tetri更新后，占位坐标点在grid坐标系的位置中，是否已经被占用
                    if(self._block_arr[coord.x as usize][coord.y as usize].is_occupied()){
                        *curr_terimino = old_tetrimino;
                        return false;
                    }
                }//end for
                
                //Self::update_block_area(&old_actual_block_coords, 0, &mut self._block_arr);
                Self::update_block_area(new_actual_block_coords, 1, &mut self._block_arr);
            },
            None => {
                log("play_field.rs","try_rotate_tetrimino() ---> curr tetrimino is none",LogLevelEnum::Error);
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

    /// 根据输入的坐标更新方块占位情况 / update block according to input coordinates
    fn update_block_area(coords_to_update:&Vec<IVec2>, occupied_flag:u8, block_area:&mut [[TetriGridCell;constant::BLOCK_AREA_MAX_HORIZONTAL_BLOCK_CNT];constant::BLOCK_AREA_MAX_VERTICAL_BLOCK_CNT]) -> bool{
        for coord in coords_to_update.iter(){
            let x = coord.x as usize;
            let y = coord.y as usize;
            if(x < 0 || x >= constant::BLOCK_AREA_MAX_VERTICAL_BLOCK_CNT || y < 0 || y >= constant::BLOCK_AREA_MAX_HORIZONTAL_BLOCK_CNT){
                log("play_field.rs","update_block_area() ---> coord out of range",LogLevelEnum::Error);
                return false;
            }
            block_area[coord.x as usize][coord.y as usize].set_occupied(occupied_flag);
        }
        return true;
    }
}