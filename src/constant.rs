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
pub const WINDOW_WIDTH: f32 = 1280.0;
///窗体高度 / Window height
pub const WINDOW_HEIGHT: f32 = 720.0;

//------------------------------procedure------------------------------
/// 标题文本缩放 / Title text scale
pub const PROC_MAIN_UI_ITEM_TEXT_SCALE : f32 = 60.0;

//------------------------------color------------------------------
pub const COLOR_RGB_BLACK : [f32;3] = [0.0 ,0.0 ,0.0];
pub const COLOR_RGBA_BLACK_1 : [f32;4] = [0.0 ,0.0 ,0.0, 1.0];
pub const COLOR_ALPHA_1 : [f32;1] = [1.0];
pub const COLOR_ALPHA_0 : [f32;1] = [0.0];