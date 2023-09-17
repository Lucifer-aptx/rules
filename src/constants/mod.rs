
// 格子大小
pub const BLOCK_SIZE: f32 = 15.;
// 格子边框大小
pub const BLOCK_BOARD_SIZE: f32 = 0.5;
// 格子半径
pub static BLOCK_RADIUS: f32 = BLOCK_SIZE / 2.;
// 格子内部大小
pub static CACHE_SIZE: f32 = BLOCK_SIZE - BLOCK_BOARD_SIZE;
// 地图大小
pub const MAP_SIZE: (f32, f32) = (1000., 1000.);
// 地图半径
pub static MAP_HALF_SIZE: (f32, f32) = (MAP_SIZE.0 / 2., MAP_SIZE.1 / 2.);
// 视图位置移动速度大小
pub const MOVE_SPEED: f32 = 1.5;