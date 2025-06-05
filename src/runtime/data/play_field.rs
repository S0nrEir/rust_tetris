use std::io::Lines;
use std::num::ParseFloatError;
use ggez::glam::{IVec2, Vec2};
use ggez::graphics::Color;
use crate::constant;
use crate::define::enum_define::{PlayFieldColorEnum, TetriminoColorEnum, TetriminoTypeEnum};
use crate::runtime::data::teri_grid::TetriGridCell;
use crate::runtime::data::tetrimino::Tetrimino;
use crate::tools::{self};
use crate::tools::logger::*;

/// 游玩区域 / Play area
#[derive(Debug)]
pub struct PlayField {
    _block_arr : [[TetriGridCell;constant::PLAY_FIELD_RAWS];constant::PLAY_FIELD_COLS],
    _curr_terimino : Option<Tetrimino>
}

//------------------------------instance function------------------------------
impl PlayField {

    /// 将所有悬空的方块下落到底部 / Make all floating blocks fall to the bottom
    pub fn fall_all_floating_blocks(&mut self) {
        
        let mut has_changes = true;
        // 重复直到没有方块可以继续下落
        while has_changes {
            has_changes = false;
            for j in (0..constant::PLAY_FIELD_RAWS-1).rev() {
                for i in 0..constant::PLAY_FIELD_COLS {
                    if self._block_arr[i][j].is_occupied() && !self._block_arr[i][j+1].is_occupied() {
                        let color = self._block_arr[i][j].color().clone();

                        self._block_arr[i][j+1].set_occupied(1);
                        self._block_arr[i][j+1].set_color(color);

                        self._block_arr[i][j].set_occupied(0);
                        self._block_arr[i][j].set_color(Color::BLACK);

                        has_changes = true;
                    }
                }
            }
        }
    }
    
    /// 获取方块区域 / get block area
    pub fn get_block_area(&self) -> & [[TetriGridCell;constant::PLAY_FIELD_RAWS];constant::PLAY_FIELD_COLS]{
        return &self._block_arr;
    }
    
    /// 生成新方块，代替当前的方块
    /// #Return
    /// * 是否生成成功，如果生成成功则更新grid area占位情况，失败则表示空间不足 / Whether the generation is successful, if the generation is successful, the occupancy situation of the grid area will be updated, and failure indicates insufficient space
    pub fn generate_new_tetrimino(&mut self) -> bool{
        match self._curr_terimino{
            
            Some(ref mut curr_tetrimino ) => {
                let gen_result = curr_tetrimino.gen_as_new(&self._block_arr);
                let tetri_color=  curr_tetrimino.color();
                if gen_result.0 {
                    Self::update_block_area(&curr_tetrimino.block_actual_coord(), 1, &mut self._block_arr,PlayFieldColorEnum::BlockColor(tetri_color));
                }
                else{
                    log("play_field.rs" , &gen_result.1 , LogLevelEnum::Error);
                }
                return gen_result.0;
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
    pub fn block_area(&self) -> & [[TetriGridCell;constant::PLAY_FIELD_RAWS];constant::PLAY_FIELD_COLS]{
        return &self._block_arr;
    }

    /// 获取方块区域 / get block area
    /// #Return
    /// * 返回方块区域 / return block area
    pub fn block_area_mut(&mut self) -> &mut [[TetriGridCell;constant::PLAY_FIELD_RAWS];constant::PLAY_FIELD_COLS]{
        return &mut self._block_arr;
    }
    
    /// 将当前方块下降一格 / Drop the current block by one grid
    /// #Return
    /// * 返回值1表示是否下降成功，返回值2表示是否到达顶部 / Return value 1 indicates whether the drop is successful, and return value 2 indicates whether the top is reached
    pub fn fall_one(&mut self) -> (bool,bool){
        match self._curr_terimino {
            
            Some(ref mut curr_tetrimino) => {
                let tetri_color = curr_tetrimino.color();
                //先检测下一格有没有可移动的格子
                
                Self::update_block_area(curr_tetrimino.block_actual_coord(), 0, &mut self._block_arr,PlayFieldColorEnum::Black);
                if !curr_tetrimino.update_coord(IVec2::new(0,1)){
                    Self::update_block_area(curr_tetrimino.block_actual_coord(), 1, &mut self._block_arr,PlayFieldColorEnum::BlockColor(tetri_color));
                    return (false,self.is_top_occupied());
                }
                if Self::detect_tetrimino_collision(&self._block_arr, curr_tetrimino.block_actual_coord()) {
                    curr_tetrimino.update_coord(IVec2::new(0,-1));
                    Self::update_block_area(&curr_tetrimino.block_actual_coord(), 1, &mut self._block_arr,PlayFieldColorEnum::BlockColor(tetri_color));
                    return (false , self.is_top_occupied());
                }
                else{
                    //下一格无碰撞，当前位置标记位无占位，下一格位置标记位有占位
                    Self::update_block_area(curr_tetrimino.block_actual_coord(), 1, &mut self._block_arr,PlayFieldColorEnum::BlockColor(tetri_color));
                    return (true,false);
                }
                
            }
            None => {
                return (false,false);
            }
        }
    }
    
    /// 尝试下落方块到底部，下落后同步更新grid / Try to drop the block to the bottom, and update the grid synchronously after the drop
    /// #Return
    /// * item0表示下落是否成功，item1表示是否到达顶部，即不可再生成新的方块 / item1 indicates whether the drop is successful, item2 indicates whether the top is reached, that is, no new blocks can be generated
    pub fn try_fall_to_bottom(&mut self) -> (bool,bool){
        
        if let Some(ref mut curr_tetrimino) = self._curr_terimino{
            let mut old_coords = curr_tetrimino.block_actual_coord().clone();
            let mut move_counter  = 0;
            Self::update_block_area(curr_tetrimino.block_actual_coord(), 0, &mut self._block_arr,PlayFieldColorEnum::Black);

            for i in 0..old_coords.len() {
                old_coords[i].y += 1;
            }
            
            while !Self::detect_tetrimino_collision_one_by_one(&self._block_arr, &old_coords) {
                for i in 0..old_coords.len() {
                    old_coords[i].y += 1;
                }
                move_counter += 1;
            }

            if move_counter > 0 {
                let color = curr_tetrimino.color();
                curr_tetrimino.update_coord(IVec2::new(0,move_counter));
                Self::update_block_area(curr_tetrimino.block_actual_coord(), 1, &mut self._block_arr,PlayFieldColorEnum::BlockColor(color));
                return (true,self.is_top_occupied());
            }
            else {
                Self::update_block_area(&curr_tetrimino.block_actual_coord(), 1, &mut self._block_arr,PlayFieldColorEnum::Black);
            }

            return (false,false);
            
        }
        else{
            log("play_field.rs","try_fall_tetrimino() ---> curr tetrimino is none",LogLevelEnum::Error);
            return (false,false);
            
        }
    }
    
    
    /// 尝试水平移动方块，移动成功则同步更新grid / Try to move the block horizontally, if the move is successful, update the grid synchronously
    /// #Arguments
    /// * `move_right` - 是否向右移动，如果为false则向左移动 / whether to move to the right, if false, move to the left
    /// #Return
    /// * 是否移动成功 / whether the move is successful
    pub fn try_horizontal_move_tetrimino(&mut self,offset:i32) -> bool{
        
        if offset != 1 && offset != -1{
            return false;
        }
        
        match self._curr_terimino{
            Some(ref mut curr_tetrimino) => {
                let mut new_coords = curr_tetrimino.block_actual_coord().clone();
                for i in 0..new_coords.len() {
                    new_coords[i].x += offset;
                    if new_coords[i].x < 0 || new_coords[i].x as usize >= constant::PLAY_FIELD_COLS {
                        log("play_field.rs",&format!("try_horizontal_move_tetrimino() ---> move out of range,curr tetrimino x coord : {},y coord : {}",new_coords[i].x,new_coords[i].y),LogLevelEnum::Info);
                        return false;
                    }
                }
                
                let color = curr_tetrimino.color();
                Self::update_block_area(curr_tetrimino.block_actual_coord(), 0, &mut self._block_arr,PlayFieldColorEnum::Black);
                if !Self::detect_tetrimino_collision(&self._block_arr, &new_coords){
                    curr_tetrimino.update_coord_by_vec2(new_coords);
                    Self::update_block_area(curr_tetrimino.block_actual_coord(),1, &mut self._block_arr,PlayFieldColorEnum::BlockColor(color));
                    return true;
                }
                Self::update_block_area(curr_tetrimino.block_actual_coord(), 1, &mut self._block_arr,PlayFieldColorEnum::BlockColor(color));
                return false;
            }//
            
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
    pub fn try_rotate_tetrimino(&mut self,close_wise:bool) -> bool{
        match self._curr_terimino {
            Some(ref mut curr_terimino) => {
                let old_coord = curr_terimino.block_actual_coord().clone();
                Self::update_block_area(&old_coord, 0, &mut self._block_arr , PlayFieldColorEnum::Black);
                if close_wise {
                    curr_terimino.rotate(true);
                }
                else {
                    curr_terimino.rotate(false);
                }

                let color = curr_terimino.color();
                if Self::detect_tetrimino_collision(&self._block_arr, curr_terimino.block_actual_coord()) {
                    curr_terimino.update_coord_by_vec2(old_coord);
                    Self::update_block_area(curr_terimino.block_actual_coord(), 1, &mut self._block_arr , PlayFieldColorEnum::BlockColor(color));
                    return false;
                }
                Self::update_block_area(&curr_terimino.block_actual_coord(), 1, &mut self._block_arr,PlayFieldColorEnum::BlockColor(color));
                return true;
            },

            None => {
                log("play_field.rs","try_rotate_tetrimino() ---> curr tetrimino is none",LogLevelEnum::Error);
                return false;
            }
        }
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
    /// * 返回消除的行数和对应的游玩区域坐标 / return the number of lines cleared and the corresponding coordinates
    pub fn try_clear_line(&mut self) -> (u8,Vec<IVec2>){

        let mut cleared_cells : Vec<IVec2> = Vec::new();
        let mut line_index = 0;
        let mut is_line_full = true;
        let mut cleared_lines = 0;
        let mut curr_col = 0;
        for j in 0..constant::PLAY_FIELD_RAWS{
            curr_col = j;
            for i in 0..constant::PLAY_FIELD_COLS{
                
                if !self._block_arr[i][j].is_occupied() {
                    is_line_full = false;
                    break;
                }
            }

            if is_line_full {
                cleared_lines += 1;
                for i in 0..constant::PLAY_FIELD_COLS{
                    cleared_cells.push(IVec2::new(i as i32,curr_col as i32));
                    self._block_arr[i][curr_col].set_occupied(0);
                    self._block_arr[i][curr_col].set_color(Color::BLACK);
                }
            }

            is_line_full = true;

        }
        return (cleared_lines , cleared_cells);

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
        log("play_field.rs","PlayField::new() ---> new play field created",LogLevelEnum::Info);
        return PlayField {
            _block_arr : PlayField::gen_block_arr(),
            _curr_terimino : None
        };
    }
    
    fn gen_block_arr() -> [[TetriGridCell;constant::PLAY_FIELD_RAWS];constant::PLAY_FIELD_COLS] {
        //#todo这里数组在声明的时候给了个初值，后面又做了一次初始化，看看怎么把这两步合并一下
        let mut block_arr = [[TetriGridCell::new(Vec2::new(0.0,0.0),IVec2::ZERO);constant::PLAY_FIELD_RAWS];constant::PLAY_FIELD_COLS];
        let mut x : f32;
        let mut y : f32;

        for i in 0..constant::PLAY_FIELD_COLS {
            for j in 0..constant::PLAY_FIELD_RAWS {
                x = constant::BLOCK_INIT_START_COORD.0 + i as f32 * (constant::BLOCK_SIZE + constant::BLOCK_COORD_SPACING as f32);
                y = constant::BLOCK_INIT_START_COORD.1 + j as f32 * (constant::BLOCK_SIZE + constant::BLOCK_COORD_SPACING as f32);
                block_arr[i][j] = TetriGridCell::new(Vec2::new(x,y),IVec2::new(i as i32,j as i32));
                block_arr[i][j].set_occupied(0);
            }
        }
        return block_arr;
    }

    /// 根据输入的坐标更新方块占位情况 / update block according to input coordinates
    fn update_block_area
    (
        coords_to_update:&Vec<IVec2>, 
        occupied_flag:u8, 
        block_area:&mut [[TetriGridCell;constant::PLAY_FIELD_RAWS];constant::PLAY_FIELD_COLS],
        color_to_set: PlayFieldColorEnum
    ) -> bool{
        
        for coord in coords_to_update.iter(){
            let x = coord.x as usize;
            let y = coord.y as usize;
            let curr_cell = &mut block_area[coord.x as usize][coord.y as usize];
            if x >= constant::PLAY_FIELD_COLS || y < 0 || y >= constant::PLAY_FIELD_RAWS {
                log("play_field.rs","update_block_area() ---> coord out of range",LogLevelEnum::Error);
                return false;
            }
            curr_cell.set_occupied(occupied_flag);
            match color_to_set {
                PlayFieldColorEnum::BlockColor(block_color) => {
                    match block_color {
                        TetriminoColorEnum::Cyan => {
                            curr_cell.set_color(Color::CYAN);
                        },
                        TetriminoColorEnum::Yellow => {
                            curr_cell.set_color(Color::YELLOW);
                        },
                        TetriminoColorEnum::Red => {
                            curr_cell.set_color(Color::RED);
                        },
                        TetriminoColorEnum::Green => {
                            curr_cell.set_color(Color::GREEN);
                        },
                        TetriminoColorEnum::Blue => {
                            curr_cell.set_color(Color::BLUE);
                        },
                        TetriminoColorEnum::Purple => {
                            curr_cell.set_color(Color::new(0.5,0.0,0.5,1.0));
                        }
                        TetriminoColorEnum::White => {
                            curr_cell.set_color(Color::WHITE);
                        }
                    }
                },
                PlayFieldColorEnum::Black => {
                    curr_cell.set_color(Color::BLACK);
                }
            }
        }
        return true;
    }
    
    /// 检查是否有最顶层的方块坐标被放置了 / check if the topmost block coordinates are placed
    /// #Arguments
    /// * `block_area` - 方块区域 / block area
    /// #Return
    /// * 是否有最顶层的方块坐标被放置了，是返回true / whether the topmost block coordinates are placed, return true
    fn top_occupied(block_area:&[[TetriGridCell;constant::PLAY_FIELD_RAWS];constant::PLAY_FIELD_COLS]) -> bool{
        let len = block_area.len();
        for i in 0..constant::PLAY_FIELD_COLS {
            if block_area[i][0].is_occupied() {
                return true;
            }
        }
        return false;
    }
    
    /// 重置游玩区域的所有数据
    pub fn reset(&mut self){
        
        if let Some(ref mut curr_tetri) = self._curr_terimino{
            curr_tetri.clear();
        }
        
        for i in 0..constant::PLAY_FIELD_COLS {
            for j in 0..constant::PLAY_FIELD_RAWS {
                self._block_arr[i][j].set_occupied(0);
            }
        }
    }
    
    fn detect_tetrimino_collision_one_by_one
    (
        block_area:&[[TetriGridCell;constant::PLAY_FIELD_RAWS];constant::PLAY_FIELD_COLS],
        tetri_actual_coords : &Vec<IVec2>
    ) -> bool {
        
        for coord in tetri_actual_coords {
            
            if coord.x < 0 || coord.x as usize >= constant::PLAY_FIELD_COLS || coord.y < 0 || coord.y as usize >= constant::PLAY_FIELD_RAWS {
                // log("play_field.rs","detect_tetrimino_collision_one_by_one() ---> coord out of range",LogLevelEnum::Error);
                return true;
            }
            
            if block_area[coord.x as usize][coord.y as usize].is_occupied(){
                return true;
            }
        }
        return false;
    }
    
    /// 检查游玩区域的指定坐标位置是否有方块占位 / Check whether there is a block occupied at the specified coordinate position in the play area
    /// #Arguments
    /// * `block_area` - 方块区域 / block area
    /// * `x` - x坐标 / x coordinate
    /// * `y` - y坐标 / y coordinate
    /// #Return
    /// * 是否有方块占位，冲突返回true / whether there is a block occupied, return true if there is a conflict
    fn detect_tetrimino_collision_one_cell(
        block_area:&[[TetriGridCell;constant::PLAY_FIELD_RAWS];constant::PLAY_FIELD_COLS],
        x : usize,
        y : usize) -> bool
    {
        if x >= constant::PLAY_FIELD_COLS || y >= constant::PLAY_FIELD_RAWS {
            log("play_field.rs","detecte_tetrimino_collision_one_cell() ---> coord out of range",LogLevelEnum::Error);
            return false;
        }
        
        return  block_area[x][y].is_occupied();
    }
    
    /// 检查给定方块的区域坐标是否与游玩区域冲突 / check whether the area coordinates of the given block conflict with the play area
    /// #Arguments
    /// * `block_area` - 方块区域 / block area
    /// * `tetri_actual_coords` - 方块的实际坐标 / actual coordinates of the block
    /// #Return
    /// * 是否冲突，冲突然会true / whether there is a conflict, if there is a conflict, return true
    pub fn detect_tetrimino_collision(
        block_area:&[[TetriGridCell;constant::PLAY_FIELD_RAWS];constant::PLAY_FIELD_COLS],
        tetri_actual_coords : &Vec<IVec2>
    ) -> bool{

        for coords in tetri_actual_coords{
            if block_area[coords.x as usize][coords.y as usize].is_occupied() {
                return true;
            }
        }
        return false;
    }
}
