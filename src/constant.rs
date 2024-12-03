//------------------------------font------------------------------
///字体名称
pub const FONT_NAME: &str = "consola";
///字体资产路径
pub const FONT_ASSET_PATH: &str = "/font/consola.ttf";

//----------------------------resource------------------------------
///资源目录
pub const RESOURCE_DIR: &str = "resource";

//------------------------------App------------------------------
///game id
pub const APP_GAME_ID : &str = "rust_tetris";
/// 作者 / author
pub const APP_AUTHOR_NAME : &str = "SonrEir";
///帧率
pub const APP_FPS : u32 = 60;

/// 1秒钟主逻辑更新时间 / 1 second main logic update time
pub const APP_MAIN_TICK_INTERVAL_1_SEC : f32 = 1.0;

//------------------------------runtime------------------------------
///初始流程下标索引
pub const RUNTIME_INITIAL_PROCEDURE_INDEX : i32 = 0;

//------------------------------window------------------------------
///窗体宽度 / Window width
pub const WINDOW_WIDTH: f32 = 620.0;
///窗体高度 / Window height
pub const WINDOW_HEIGHT: f32 = 860.0;

//------------------------------procedure------------------------------
/// 标题文本缩放 / Title text scale
pub const PROC_MAIN_UI_ITEM_TEXT_SCALE : f32 = 60.0;

//------------------------------color------------------------------
pub const COLOR_RGB_BLACK : [f32;3] = [0.0 ,0.0 ,0.0];
pub const COLOR_RGBA_BLACK_1 : [f32;4] = [0.0 ,0.0 ,0.0, 1.0];
pub const COLOR_ALPHA_1 : [f32;1] = [1.0];
pub const COLOR_ALPHA_0 : [f32;1] = [0.0];

//------------------------------block------------------------------
/// 横向可容纳的方块的最大数量 / Maximum number of blocks that can be accommodated horizontally
pub const BLOCK_AREA_MAX_HORIZONTAL_BLOCK_CNT : u8 = 10;
/// 纵向可容纳的方块的最大数量 / Maximum number of blocks that can be accommodated vertically
pub const BLOCK_AREA_MAX_VERTICAL_BLOCK_CNT : u8 = 20;
/// 单个方块的像素尺寸 / Pixel size of a single block
pub const BLOCK_PIXEL_SIZE : u8 = 10;
/// 方块间的像素间隔 / Pixel spacing between blocks
pub const BLOCK_PIXEL_SPACING : u8 = 3;
/// 方块放置区域初始化的像素位置 / Pixel position of block placement area initialization 
pub const BLOCK_INIT_START_PIXEL_POS : (f32,f32) = (100.0,100.0);