use std::collections::HashSet;
use std::collections::VecDeque;

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

enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    pub head: Coord,
    /// not including head
    pub body: VecDeque<Coord>,
    pub direction: MoveDirection,
    pub dead: bool,
}

enum TurnDirection {
    Left,
    Right,
}

impl Snake {
    // unconditionally move the snake - we must check for collisions at a
    // later step, to handle the case of multiple snakes headbutting.
    pub fn move_forward(&mut self) {
        // set the new head
        let head = self.head;
        let new_head = match self.direction {
            MoveDirection::Up => (head.x - 1, head.y),
            MoveDirection::Down => (head.x + 1, head.y),
            MoveDirection::Left => (head.x, head.y - 1),
            MoveDirection::Right => (head.x, head.y + 1),
        }
        .into();

        // set the new head + shift the body
        self.head = new_head;
        self.body.push_front(head);
        self.body.pop_back();
    }

    /// does not move the snake
    pub fn turn(&mut self, turn_direction: TurnDirection) {
        self.direction = match turn_direction {
            TurnDirection::Left => match self.direction {
                MoveDirection::Up => MoveDirection::Left,
                MoveDirection::Down => MoveDirection::Right,
                MoveDirection::Left => MoveDirection::Down,
                MoveDirection::Right => MoveDirection::Up,
            },
            TurnDirection::Right => match self.direction {
                MoveDirection::Up => MoveDirection::Right,
                MoveDirection::Down => MoveDirection::Left,
                MoveDirection::Left => MoveDirection::Up,
                MoveDirection::Right => MoveDirection::Down,
            },
        };
    }
}

/// global, game state
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
                head: (rows / 2, cols / 2).into(),
                body: VecDeque::new(),
                direction: MoveDirection::Right,
                dead: false,
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
