use piston_window::*;
use rand::{thread_rng, Rng};

use crate::color::*;
use crate::draw::*;
use crate::food::Food;
use crate::snake::*;

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 3.0;

#[derive(Debug, Clone)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x_pos: i32, y_pos: i32) -> Position {
        Position { x: x_pos, y: y_pos }
    }
    pub fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

pub struct Game {
    snake: Snake,
    food: Food,
    food_status: bool,
    width: i32,
    height: i32,

    pub gameover: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(canvas_width: i32, canvas_height: i32) -> Game {
        Game {
            snake: Snake::new(canvas_width / 2, canvas_height / 2),
            food: Food::new(),
            food_status: true,
            width: canvas_width,
            height: canvas_height,
            gameover: false,
            waiting_time: 0.0,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_status {
            draw_block(
                self.food.get_position().0,
                self.food.get_position().1,
                FOOD_COLOR,
                con,
                g,
            )
        }

        draw_rectangle(0, 0, self.width, 1, CANVAS_COLOR_2, con, g);
        draw_rectangle(0, self.height - 1, self.width, 1, CANVAS_COLOR_2, con, g);
        draw_rectangle(0, 0, 1, self.height, CANVAS_COLOR_2, con, g);
        draw_rectangle(self.width - 1, 0, 1, self.height, CANVAS_COLOR_2, con, g);

        if self.gameover {
            draw_rectangle(0, 0, self.width, self.height, GAMEOVER_COLOR, con, g);
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.gameover {
            return;
        }
        let mut direction_next = match key {
            Key::Up => Direction::Up,
            Key::Down => Direction::Down,
            Key::Left => Direction::Left,
            Key::Right => Direction::Right,
            _ => self.snake.get_direction(),
        };

        if direction_next == direction_next.opposite() {
            direction_next = self.snake.get_direction();
        }
        self.update_snake(Some(direction_next));
    }

    pub fn update_game(&mut self, elapse_time: f64) {
        self.waiting_time += elapse_time;
        if self.gameover {
            if self.waiting_time > RESTART_TIME {
                self.restart_game();
            }
            return;
        }

        if !self.food_status {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn restart_game(&mut self) {
        self.snake = Snake::new(self.width / 2, self.height / 2);
        self.food = Food::new();
        self.food_status = true;
        self.gameover = false;
        self.waiting_time = 0.0;
    }

    fn check_snake(&self, dir: Direction) -> bool {
        let next_pos = self.snake.next_move(dir);
        let next_pos_x = next_pos.get_position().0;
        let next_pos_y = next_pos.get_position().1;
        if self.snake.overlap(next_pos) {
            return false;
        }
        return next_pos_x > 0
            && next_pos_y > 0
            && next_pos_x < self.width - 1
            && next_pos_y < self.height - 1;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        let direction = dir.unwrap_or(self.snake.get_direction());
        if self.check_snake(direction) == true {
            self.snake.move_forward(direction);
            self.check_food();
        } else {
            self.gameover = true;
        }
        self.waiting_time = 0.0;
    }

    fn check_food(&mut self) {
        if self.food_status && self.food.get_position() == self.snake.head_position() {
            self.food_status = false;
            self.snake.restore_tail();
        }
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1, self.width - 1);
        let mut new_y = rng.gen_range(1, self.height - 1);
        while self.snake.overlap(Position { x: new_x, y: new_y }) {
            new_x = rng.gen_range(1, self.width - 1);
            new_y = rng.gen_range(1, self.height - 1);
        }
        self.food = Food::new();
        self.food.set_position(new_x, new_y);
        self.food_status = true;
    }
}
