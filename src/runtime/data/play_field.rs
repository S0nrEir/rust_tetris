use std::cell;
use ggez::glam::{IVec2, Vec2};
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
    
    /// 生成新方块，代替当前的方块
    pub fn generate_new_tetrimino(&mut self) -> bool{
        match self._curr_terimino{
            Some(ref mut curr_tetrimino ) => {
                let gen_succ = curr_tetrimino.gen_as_new(&self._block_arr);
                if gen_succ {
                    Self::update_block_area(&curr_tetrimino.block_actual_coord(), 1, &mut self._block_arr);
                }
                return gen_succ;
            }
            None => {
                log("play_field.rs","generate_new_tetrimino() ---> curr tetrimino is none",LogLevelEnum::Error);
                return false;
            }
        }
    }
    
    /// 检查是否有最顶层的方块坐标被放置了 / check if the topmost block coordinates are placed
    /// #Arguments
    /// * `block_area` - 方块区域 / block area
    /// #Return
    /// * 是否有最顶层的方块坐标被放置了，是返回true / whether the topmost block coordinates are placed, return true
    pub fn is_top_occupied(&self) -> bool{
        return Self::top_occupied(&self._block_arr);
    }
    
    /// 获取方块区域 / get block area
    /// #Return
    /// * 返回方块区域 / return block area
    pub fn block_area(&self) -> &[[TetriGridCell;constant::BLOCK_AREA_MAX_HORIZONTAL_BLOCK_CNT];constant::BLOCK_AREA_MAX_VERTICAL_BLOCK_CNT]{
        return &self._block_arr;
    }
    
    /// 将当前方块下降一格 / Drop the current block by one grid
    /// #Return
    /// * 返回值1表示是否下降成功，返回值2表示是否到达顶部 / Return value 1 indicates whether the drop is successful, and return value 2 indicates whether the top is reached
    pub fn drop_by_one(&mut self) -> (bool,bool){
        match self._curr_terimino { 
            Some(ref mut curr_tetrimino) => {
                // for coord in new_actual_block_coords.iter_mut(){
                //     coord.y += 1;
                // }
                let old_actual_block_coords = curr_tetrimino.block_actual_coord().clone();
                curr_tetrimino.update_coord(IVec2::new(0,1));
                let new_actual_block_coords = curr_tetrimino.block_actual_coord().clone();
                //检查下落后是否有碰撞
                if Self::detect_tetrimino_collision(&self._block_arr, &new_actual_block_coords) {
                    //下一格有碰撞，就停在当前位置
                    Self::update_block_area(&old_actual_block_coords, 1, &mut self._block_arr);
                    return (true,self.is_top_occupied());
                }
                else{
                    //下一格无碰撞，当前位置标记位无占位，下一格位置标记位有占位
                    Self::update_block_area(&old_actual_block_coords, 0, &mut self._block_arr);
                    Self::update_block_area(&new_actual_block_coords, 1, &mut self._block_arr);
                    return (false,true);
                }
            }
            None => {
                return (false,false);
            }
        }
    }
    
    /// 尝试下落方块到底部，下落后同步更新grid / Try to drop the block to the bottom, and update the grid synchronously after the drop
    /// #Return
    /// * item1表示下落是否成功，item2表示是否到达顶部，即不可再生成新的方块 / item1 indicates whether the drop is successful, item2 indicates whether the top is reached, that is, no new blocks can be generated
    pub fn try_drop_to_bottom(&mut self) -> (bool,bool){
        match self._curr_terimino{
            Some(ref mut curr_tetrimino) => {
                let old_actual_coords = curr_tetrimino.block_actual_coord().clone();
                curr_tetrimino.update_coord(IVec2::new(0,1));
                let mut drop_succ = false;
                let new_actual_coords = curr_tetrimino.block_actual_coord().clone();
                while !drop_succ {
                    // for coord in new_actual_coords.iter_mut(){
                    //     coord.y += 1;
                    // }
                    
                    if Self::detect_tetrimino_collision(&self._block_arr, &new_actual_coords) {
                        Self::update_block_area(&curr_tetrimino.block_actual_coord(), 1, &mut self._block_arr);
                        drop_succ = true;
                    }
                    curr_tetrimino.update_coord(IVec2::new(0,1));
                }//end while
                Self::update_block_area(&old_actual_coords, 0, &mut self._block_arr);
                return (true,self.is_top_occupied());
            }
            None => {
                log("play_field.rs","try_fall_tetrimino() ---> curr tetrimino is none",LogLevelEnum::Error);
                return (false,false);
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
                let tetri_coords = curr_terimino.get_coord();
                let offset = if move_right {1} else {-1};
                let new_x = tetri_coords.x + offset;
                if new_x < 0 || new_x as usize >= constant::BLOCK_AREA_MAX_HORIZONTAL_BLOCK_CNT {
                    log("play_field.rs",&format!("try_horizontal_move_tetrimino() ---> move out of range,curr tetrimino x coord : {},y coord : {}",tetri_coords.x,tetri_coords.y),LogLevelEnum::Warning);
                    return false;
                }
                match self._curr_terimino { 
                    Some(ref mut curr_tetrimino) => {
                        let old_actual_coords = curr_tetrimino.block_actual_coord().clone();
                        curr_tetrimino.update_coord(IVec2::new(offset,0));
                        let new_actual_coords = curr_tetrimino.block_actual_coord();
                        for coords in new_actual_coords.iter() {
                            //有碰撞则视为失败
                            if self._block_arr[coords.x as usize][coords.y as usize].is_occupied() {
                                curr_tetrimino.update_coord(IVec2::new(-offset,0));
                                return false;
                            }
                        }
                        Self::update_block_area(&old_actual_coords, 0, &mut self._block_arr);
                        Self::update_block_area(&old_actual_coords, 1, &mut self._block_arr);
                        return true;
                    },
                    None => {
                        return false;
                    }
                }
                // let block_horizontal_len = self._block_arr[0].len();
                // let horizontal_offset = if move_right {1} else {-1};
                // let tetrimino_actual_coord = curr_terimino.get_coord().clone();
                // 
                // //检查左右超越边界
                // if(tetrimino_actual_coord.x + horizontal_offset < 0){
                //     log("play_field.rs","try_horizontal_move_tetrimino() ---> move left out of range",LogLevelEnum::Info);
                //     return false;
                // }
                // if(tetrimino_actual_coord.x + horizontal_offset + constant::BLOCK_MAX_OCCUPIED as i32 >= block_horizontal_len as i32){
                //     log("play_field.rs","try_horizontal_move_tetrimino() ---> move right out of range",LogLevelEnum::Info);
                //     return false;
                // }
                // 
                // let old_actual_block_coords = curr_terimino.block_actual_coord();
                // let mut new_actual_block_coords = old_actual_block_coords.clone();
                // let mut temp_idx = 0;
                // //检查左右移动后，是否有占位冲突
                // for coord in old_actual_block_coords{
                //     let x = coord.x + horizontal_offset;
                //     let y = coord.y;
                //     if(self._block_arr[x as usize][y as usize].is_occupied()){
                //         log("play_field.rs",&format!("x:{},y:{} occupied",x,y),LogLevelEnum::Info);
                //         return false;
                //     }
                //     new_actual_block_coords[temp_idx].x = x + horizontal_offset;
                //     temp_idx = temp_idx + 1;
                // }
                // //没有冲突则更新占位情况
                // //Self::update_block_area(&old_actual_block_coords, 0, &mut self._block_arr);
                // Self::update_block_area(&new_actual_block_coords, 1, &mut self._block_arr);
                // return true;
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
                //let old_actual_block_coords = curr_terimino.block_actual_coord().clone();
                let old_tetrimino = curr_terimino.clone();
                //turn right
                if turn_right {
                    curr_terimino.rotate(true);
                }
                //turn left
                else { 
                    curr_terimino.rotate(false);
                }
                
                let new_actual_block_coords = curr_terimino.block_actual_coord();
                for coord in new_actual_block_coords.iter() {
                    //检查curr tetri更新后，占位坐标点在grid坐标系的位置中，是否已经被占用
                    if self._block_arr[coord.x as usize][coord.y as usize].is_occupied() {
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
        if self._curr_terimino.is_none() {
            return &TetriminoTypeEnum::None;
        }
        return self._curr_terimino.as_ref().unwrap().get_type();
    }
    
    /// 初始化方块 / Initialize tetrimino
    pub fn init_tetrimino(&mut self){
        self._curr_terimino = tools::tetri_tools::gen_rand_tetrimino();
        if self._curr_terimino.is_none() {
            log("play_field.rs","init_tetrimino() ---> curr_tetrimino is none",LogLevelEnum::Fatal);
            panic!();
        }
    }
    
    /// 尝试消除一行，如果消除成功则更新对应的block grid / Try to clear a line, if the elimination is successful, update the corresponding block grid
    /// #Return
    /// * 消除的行数 / number of lines eliminated
    pub fn try_clear_line(&mut self) -> u8{
        let mut cleared_lines : u8 = 0;
        let mut is_line_full = true;
        //todo：优化，记录所有格子都是空的最高行数，从这里开始检查，避免遍历所有集合
        for line in self._block_arr.iter_mut() {
            for block in line.iter() {
                if !block.is_occupied() {
                    is_line_full = false;
                    break;
                }
            }
            
            if is_line_full {
                for block in line.iter_mut() {
                    block.set_occupied(0);
                }
                cleared_lines += 1;
            }
            is_line_full = true;
        }
        
        //有消除行，重新计算所有block位置
        //找到位置最深的空行，将上面的行往下移动，倒着，从下网上走
        //todo:优化,现在是从头往下遍历，改成从下往上遍历，找到第一个为空的格就把上面所有的格子都往下移动对应行数，并且记录该列，表示已经移动过，后面的遍历不再处理
        if cleared_lines != 0 {
            for i in 0..self._block_arr.len(){
                for j in 0..self._block_arr[i].len() {
                    //如果当前格子为空且上一个格子不为空，则将其上方的所有格子都移下来
                    if !self._block_arr[i][j].is_occupied() && 
                        i < self._block_arr.len() - 1&& 
                        j < self._block_arr[i].len() && 
                        self._block_arr[i - 1][j].is_occupied() 
                    {
                        self._block_arr[i][j].set_occupied(1);
                        self._block_arr[i - 1][j].set_occupied(0);
                    }
                }
            }
        }
        
        return cleared_lines;
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
            if x < 0 || x >= constant::BLOCK_AREA_MAX_VERTICAL_BLOCK_CNT || y < 0 || y >= constant::BLOCK_AREA_MAX_HORIZONTAL_BLOCK_CNT {
                log("play_field.rs","update_block_area() ---> coord out of range",LogLevelEnum::Error);
                return false;
            }
            block_area[coord.x as usize][coord.y as usize].set_occupied(occupied_flag);
        }
        return true;
    }
    
    /// 检查是否有最顶层的方块坐标被放置了 / check if the topmost block coordinates are placed
    /// #Arguments
    /// * `block_area` - 方块区域 / block area
    /// #Return
    /// * 是否有最顶层的方块坐标被放置了，是返回true / whether the topmost block coordinates are placed, return true
    fn top_occupied(block_area:&[[TetriGridCell;constant::BLOCK_AREA_MAX_HORIZONTAL_BLOCK_CNT];constant::BLOCK_AREA_MAX_VERTICAL_BLOCK_CNT]) -> bool{
        let first_row = block_area[0];
        for cell in first_row.iter(){
            if(cell.is_occupied()){
                return true;
            }
        }
        return false;
    }
    
    /// 检查给定方块的区域坐标是否与游玩区域冲突 / check whether the area coordinates of the given block conflict with the play area
    /// #Arguments
    /// * `block_area` - 方块区域 / block area
    /// * `tetri_actual_coords` - 方块的实际坐标 / actual coordinates of the block
    /// #Return
    /// * 是否冲突 / whether conflict
    pub fn detect_tetrimino_collision(block_area:&[[TetriGridCell;constant::BLOCK_AREA_MAX_HORIZONTAL_BLOCK_CNT];constant::BLOCK_AREA_MAX_VERTICAL_BLOCK_CNT],tetri_actual_coords : &Vec<IVec2>) -> bool{
        for coords in tetri_actual_coords{
            if block_area[coords.x as usize][coords.y as usize].is_occupied() {
                return true;
            }
        }
        return false;
    }
}