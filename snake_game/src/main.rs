extern crate find_folder;
extern crate piston_window;
extern crate rand;

use piston_window::*;

mod color;
mod draw;
mod food;
mod game;
mod snake;

use crate::color::*;
use crate::draw::to_coord;
use crate::game::Game;

fn main() {
    let canvas_width: i32 = 30;
    let canvas_height: i32 = 30;

    let mut game = Game::new(canvas_width, canvas_height);

    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        [
            to_coord(canvas_width) as u32,
            to_coord(canvas_height) as u32,
        ],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, _| {
            clear(CANVAS_COLOR_1, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update_game(arg.dt);
        });
    }
}
