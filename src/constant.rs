use ggez::glam::{IVec2, Vec2};
use crate::define::enum_define::TetriminoColorEnum;

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
pub const COLOR_R1G1B1 : [f32;3] = [1.0 ,1.0 ,1.0];
pub const COLOR_R1G1B1A1 : [f32;4]  = [1.0 ,1.0 ,1.0,1.0];
pub const COLOR_R0G0B0A1 : [f32;4] = [0.0 ,0.0 ,0.0, 1.0];
pub const COLOR_ALPHA_1 : [f32;1] = [1.0];
pub const COLOR_ALPHA_0 : [f32;1] = [0.0];

//------------------------------block------------------------------
/// 游玩区域行数 / play field rows
pub const PLAY_FIELD_RAWS : usize = 20;
/// 游玩区域列数 / play field cols
pub const PLAY_FIELD_COLS : usize = 10;
/// 单个方块的尺寸 / size of a single block
pub const BLOCK_SIZE : f32 = 30.0;
/// 方块间的坐标间隔 / spacing between blocks
pub const BLOCK_COORD_SPACING : u8 = 3;
/// 方块放置区域初始化的坐标 / init start coordinate
pub const BLOCK_INIT_START_COORD: (f32, f32) = (153.0, 53.0);
/// 一个方块最大的占用范围
pub const BLOCK_MAX_OCCUPIED : usize = 4;
/// 可处理输入的时间间隔 / Time interval for processing input
pub  const INPUT_HANDLE_INTERVAL : f32 = 0.5;

pub const BORDER_MIN_POSITION : Vec2 = Vec2::new(150.0, 50.0);
pub const BORDER_MAX_POSITION : Vec2 = Vec2::new(600.0, 750.0);

/// 演出效果时间 / Performing effect time 
pub const PLAYFIELD_PERFORMING_INTERVAL : f32 = 3.;

pub const PLAYFIELD_FLASHING_INTERVAL : f32 = 1.;

/// 方块颜色生成序列
pub const BLOCK_COLOR_GEN_SEQUENCE : [TetriminoColorEnum;7] = [
    TetriminoColorEnum::Cyan,
    TetriminoColorEnum::Yellow,
    TetriminoColorEnum::Red,
    TetriminoColorEnum::Green,
    TetriminoColorEnum::Cyan,
    TetriminoColorEnum::Blue,
    TetriminoColorEnum::White
];

pub const TETRI_OFFSET_RIGHT_SNAKE : [IVec2;4] = [
    IVec2::new(-1,1),
    IVec2::new(0,0),
    IVec2::new(1,1),
    IVec2::new(2,0)
];

pub const TETRI_OFFSET_LEFT_SNAKE : [IVec2;4] = [
    IVec2::new(0,2),
    IVec2::new(1,1),
    IVec2::new(0,-1),
    IVec2::new(1,-2)
];

pub const TETRI_OFFSET_RIGHT_GUN_0_90 : [IVec2;4] = [
    IVec2::new(-1,0),
    IVec2::new(0,1),
    IVec2::new(1,0),
    IVec2::new(2,-1)
];

pub const TETRI_OFFSET_RIGHT_GUN_90_180 : [IVec2;4] = [
    IVec2::new(0,2),
    IVec2::new(1,1),
    IVec2::new(0,0),
    IVec2::new(-1,-1)
];

pub const TETRI_OFFSET_RIGHT_GUN_180_270 : [IVec2;4] = [
    IVec2::new(2,-1),
    IVec2::new(1,-2),
    IVec2::new(0,-1),
    IVec2::new(-1,0)
];

pub const TETRI_OFFSET_RIGHT_GUN_270_0 : [IVec2;4] = [
    IVec2::new(-1,-1),
    IVec2::new(-2,0),
    IVec2::new(-1,1),
    IVec2::new(0,2)
];


pub const TETRI_OFFSET_LEFT_GUN_0_90 : [IVec2;4] = [
    IVec2::new(1,-2),
    IVec2::new(2,1),
    IVec2::new(1,0),
    IVec2::new(1,-2),
];

pub const TETRI_OFFSET_LEFT_GUN_90_180 : [IVec2;4] = [
    IVec2::new(-2,0),
    IVec2::new(-1,-1),
    IVec2::new(0,0),
    IVec2::new(0,2)
];

pub const TETRI_OFFSET_LEFT_GUN_180_270 : [IVec2;4] = [
    IVec2::new(0,1),
    IVec2::new(1,-2),
    IVec2::new(0,-1),
    IVec2::new(-1,0),
];

pub const TETRI_OFFSET_LEFT_GUN_270_0 : [IVec2;4] = [
    IVec2::new(1,1),
    IVec2::new(0,2),
    IVec2::new(-1,1),
    IVec2::new(-2,0)
];

pub const TETRI_OFFSET_T_0_90 : [IVec2;4] = [
    IVec2::new(0,1),
    IVec2::new(1,0),
    IVec2::new(2,-1),
    IVec2::new(0,-1)
];

pub const TETRI_OFFSET_T_90_180 : [IVec2;4] = [
    IVec2::new(1,1),
    IVec2::new(0,0),
    IVec2::new(-1,-1),
    IVec2::new(-1,1)
];

pub const TETRI_OFFSET_T_180_270 : [IVec2;4] = [
    IVec2::new(1,-2),
    IVec2::new(0,-1),
    IVec2::new(-1,0),
    IVec2::new(1,0)
];

pub const TETRI_OFFSET_T_270_0 : [IVec2;4] = [
    IVec2::new(-2,0),
    IVec2::new(-1,1),
    IVec2::new(0,2),
    IVec2::new(0,0)
];

pub const TETRI_OFFSET_STICK : [IVec2;4] = [
    IVec2::new(0,0),
    IVec2::new(-1,1),
    IVec2::new(-2,2),
    IVec2::new(-3,3)
];