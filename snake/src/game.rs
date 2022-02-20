use piston_window::*;
use piston_window::types::Color;
// thread_rng允许我们创建一个由系统控制的本地线程随机数生成器
// Rng是一个随机数生成器
use rand::{thread_rng, Rng};
// snake行走方向和snake本身
use crate::snake::{Direction, Snake};
// 绘图块和绘图矩形
use crate::draw::{draw_block, draw_rectangle};

// 三个关于颜色的常量，第-1个元素表示不透明度
const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color= [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.50];

// 移动周期，蛇每秒移动的帧数
const MOVING_PERIOD: f64 = 0.1;
// snake发生故障的重启时间为1s.
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,
    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    // 游戏状态
    game_over: bool,
    // 等待时间，即RESTART_TIME
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height:i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
        }
    }

    // 确认用户是否按下了键，然后作出相应的反应
    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        // 检查snake移动的方向是否等于snake的头部方向和它的相反方向，是就退出
        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    // 公共绘图
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        // if true, 绘制food块
        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        // 绘制边界
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.height - 1, 0, 1, self.height, con, g);

        // 游戏结束时，绘制game over的screen.
        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    // 更新函数, 游戏中的传递作为一个可变的游戏状态停留在这里
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    // 检查snake是否吃过东西,传入的是可变游戏状态
    fn check_eating(&mut self) {
        // 通过头部位置找到头部的x和y
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            // 头部与食物重叠，则食物不存在
            self.food_exists = false;
            // 则snake会吃掉该食物，尾部将增长一个块。
            self.snake.restore_tail();
        }
    }

    //
    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        // 是否头部与尾部重叠
        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        // 头部和尾部重叠，将返回false, 如果在窗口超出范围游戏将结束，并在1秒钟后重新开始
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.height - 1);
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }

        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 1);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }


}


