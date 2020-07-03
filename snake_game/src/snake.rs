use crate::color;
use crate::draw::draw_block;
use crate::game::Position;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
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
pub struct Snake {
    direction: Direction,
    body: LinkedList<Position>,
    tail: Option<Position>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Position> = LinkedList::new();
        body.push_back(Position::new(x, y));

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_position = self.body.front().unwrap();
        (
            head_position.get_position().0,
            head_position.get_position().1,
        )
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn move_forward(&mut self, direct: Direction) {
        self.direction = direct;

        let (new_x, new_y): (i32, i32) = self.head_position();

        let new_position = match self.direction {
            Direction::Up => Position::new(new_x, new_y - 1),
            Direction::Down => Position::new(new_x, new_y + 1),
            Direction::Left => Position::new(new_x - 1, new_y),
            Direction::Right => Position::new(new_x + 1, new_y),
        };

        self.body.push_front(new_position);
        self.tail = Some(self.body.pop_back().unwrap());
    }

    pub fn next_move(&self, dir: Direction) -> Position {
        let (new_x, new_y): (i32, i32) = self.head_position();
        let move_dir = dir;
        match move_dir {
            Direction::Up => Position::new(new_x, new_y - 1),
            Direction::Down => Position::new(new_x, new_y + 1),
            Direction::Left => Position::new(new_x - 1, new_y),
            Direction::Right => Position::new(new_x + 1, new_y),
        }
    }
    pub fn restore_tail(&mut self) {
        let tail = self.tail.clone().unwrap();
        self.body.push_back(tail);
    }

    pub fn overlap(&self, point: Position) -> bool {
        for block in &self.body {
            if block.get_position() == point.get_position() {
                return true;
            }
        }
        return false;
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(
                block.get_position().0,
                block.get_position().1,
                color::SNAKE_COLOR,
                con,
                g,
            );
        }
    }
}
