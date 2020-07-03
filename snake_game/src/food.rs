use crate::game::Position;

pub struct Food {
    position: Position,
}

impl Food {
    pub fn new() -> Food {
        Food {
            position: Position::new(5, 5),
        }
    }

    pub fn get_position(&self) -> (i32, i32) {
        (
            self.position.get_position().0,
            self.position.get_position().1,
        )
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.position.set_position(x, y);
    }
}
