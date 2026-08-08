use std::collections::HashSet;

use rand::Rng;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}

impl From<(u32, u32)> for Coord {
    fn from(value: (u32, u32)) -> Self {
        Coord {
            x: value.0,
            y: value.1,
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    pub body: Vec<Coord>,
    pub direction: Direction,
}

pub struct Arena {
    pub rows: u32,
    pub cols: u32,
    pub food: Coord,
    pub snake: Snake,
}

impl Arena {
    pub fn new(rows: u32, cols: u32) -> Self {
        let mut a = Self {
            rows,
            cols,
            // dummy value - immediately overwriten
            food: (0, 0).into(),
            // snake at center, starting right
            snake: Snake {
                body: vec![(rows / 2, cols / 2).into()],
                direction: Direction::Right,
            },
        };

        // randomly seed the food
        a.regen_food();
        a
    }

    pub fn regen_food(&mut self) {
        self.food = self.spawn_food();
    }

    fn spawn_food(&self) -> Coord {
        let mut excl = HashSet::new();
        // exclude its last position
        excl.insert(self.food);
        // exclude snake body
        excl.extend(self.snake.body.iter().copied());

        Coord {
            x: rand::thread_rng().gen_range(0..self.rows),
            y: rand::thread_rng().gen_range(0..self.cols),
        }
    }
}
