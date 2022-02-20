// 该文件用于实现snake所需的大部分逻辑

use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw;
use draw::draw_block;

// 为snake的颜色创建一个常量，它是一个由四个元素组成的数组,数组元素依次表示红色、绿色、蓝色、不透明色
const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

// 处理snake的方向
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// 为枚举实现方法
impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

// 不需要公开
#[derive(Debug, Clone)]
struct Block {
    x:i32,
    y: i32,
}

// 为snake创建一个结构
pub struct Snake {
    // 头部方向
    direction: Direction,
    // 身体
    body: LinkedList<Block>,
    // 尾
    tail: Option<Block>,
}

// 为Snake实现方法
impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block {
            x: x+2,
            y,
        });

        body.push_back(Block {
            x: x+1,
            y,
        });

        body.push_back(Block {
            x,
            y,
        });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    // 绘制图形缓冲区，渲染出snake,
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        // 遍历，在snake的每个块上绘制块函数，包含了snake的颜色为绿色
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    // snake的头部位置
    pub fn head_position(&self) -> (i32, i32) {
        // front函数用于对LinkedList的前元素的引用。
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    // snake移动逻辑
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block = match self.direction {
            Direction::Up => Block{
                x: last_x,
                y: last_y - 1,
            },

            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },

            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },

            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };

        // 当snake移动时，实际上是删除尾部的一个块，并在头部新增一个块。所以在这里匹配以获取要添加的新块。
        // 所以如果我们向右转。那么该块将在蛇的右侧；如果向前，该块将在蛇的前面；

        // 头部新增块
        self.body.push_front(new_block);
        // 尾部删除块
        let removed_block= self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    // 接收snake,得到snake移动的方向
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32,i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {},
        }

        match moving_dir {
            // head_y + 1 ???
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    // 创建基于尾部的分支
    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        // 将尾部推到后面。头部吃了一个apple后，尾部将被推到链表体中，snake的身体将变长。
        self.body.push_back(blk);
    }

    // 尾重叠
    // 如果snake与其身体的任何其他部分有重叠，则返回true
    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            // snake运行一圈，如果头部和尾部重叠在一个块中的话，实际上它只有一个时刻是这种情况，这种情况我们不希望它失败。
            // 因为尾巴知道头部移动了。
            ch += 1;
            if ch == self.body.len() - 1 {
                break;
            }
        }

        return false;
    }
}



