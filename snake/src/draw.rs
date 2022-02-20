use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

// 创建一个块大小常量
const BLOCK_SIZE: f64 = 25.0;

// 坐标转换，接收一个i32类型的游戏坐标
pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}
// 绘制一个块
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        // x, y, 宽度，高度
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        // 转换矩阵
        con.transform,
        g,
    );
}

// 创建一个公共函数，用于控制矩形的大小
pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}




