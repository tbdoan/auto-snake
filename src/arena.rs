use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Display;

use rand::Rng;

use crate::blackboard::Blackboard;

/// negative coordinates are convenient for detecting collision with left/up boundary
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(i32, i32)> for Coord {
    fn from(value: (i32, i32)) -> Self {
        Coord {
            x: value.0,
            y: value.1,
        }
    }
}

impl Coord {
    // returns none if moving would underflow
    pub fn move_in(&self, dir: MoveDirection) -> Self {
        let mut after = *self; // copy it
        match dir {
            MoveDirection::Up => after.x -= 1,
            MoveDirection::Down => after.x += 1,
            MoveDirection::Left => after.y -= 1,
            MoveDirection::Right => after.y += 1,
        };
        after
    }

    /// Panics if adj is not adjacent to self
    pub fn direction_to(&self, adj: Coord) -> MoveDirection {
        assert!(
            (self.x - adj.x).abs() + (self.y - adj.y).abs() == 1,
            "coordinates must be adjacent"
        );
        for dir in MoveDirection::enumerate() {
            if adj == self.move_in(dir) {
                return dir;
            }
        }

        unreachable!("a direction must be found")
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

impl Display for MoveDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MoveDirection::Up => "up",
            MoveDirection::Down => "down",
            MoveDirection::Left => "left",
            MoveDirection::Right => "right",
        };
        write!(f, "{}", s.to_uppercase())
    }
}

impl MoveDirection {
    pub fn enumerate() -> Vec<Self> {
        vec![Self::Up, Self::Right, Self::Down, Self::Left]
    }

    pub fn clockwise(&self) -> Self {
        match self {
            MoveDirection::Up => MoveDirection::Left,
            MoveDirection::Left => MoveDirection::Down,
            MoveDirection::Down => MoveDirection::Right,
            MoveDirection::Right => MoveDirection::Up,
        }
    }

    pub fn counterclockwise(&self) -> Self {
        match self {
            MoveDirection::Up => MoveDirection::Right,
            MoveDirection::Right => MoveDirection::Down,
            MoveDirection::Down => MoveDirection::Left,
            MoveDirection::Left => MoveDirection::Up,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            MoveDirection::Up => MoveDirection::Down,
            MoveDirection::Down => MoveDirection::Up,
            MoveDirection::Left => MoveDirection::Right,
            MoveDirection::Right => MoveDirection::Left,
        }
    }
}

pub struct Snake {
    pub head: Coord,
    /// not including head
    pub body: VecDeque<Coord>,
    pub direction: MoveDirection,
    pub dead: bool,
}

impl Snake {
    // unconditionally move the snake - we must check for collisions at a
    // later step, to handle the case of multiple snakes headbutting.
    pub fn move_forward(&mut self) {
        let head = self.head;
        let new_head = self.head.move_in(self.direction);
        log::info!("new head: {new_head}");

        // set the new head + shift the body
        self.head = new_head;
        self.body.push_front(head);
        self.body.pop_back();
    }

    /// `new_direction` assumed to be valid.
    /// does not move the snake.
    pub fn turn(&mut self, new_direction: MoveDirection) {
        self.direction = new_direction;
    }
}

#[derive(Clone, Copy)]
pub struct Dimensions {
    pub rows: u32,
    pub cols: u32,
}

impl Dimensions {
    pub fn check_oob(&self, c: Coord) -> bool {
        c.x < 0 || c.x >= self.rows as i32 || c.y < 0 || c.y >= self.cols as i32
    }
}

/// global, game state
pub struct Arena {
    pub dim: Dimensions,
    pub food: Coord,
    pub snake: Snake,
}

impl Arena {
    pub fn new(rows: u32, cols: u32) -> Self {
        let mut a = Self {
            dim: Dimensions { rows, cols },
            // dummy value - immediately overwriten
            food: (0, 0).into(),
            // snake at center, starting right
            snake: Snake {
                head: (rows as i32 / 2, cols as i32 / 2).into(),
                body: VecDeque::new(),
                direction: MoveDirection::Right,
                dead: false,
            },
        };

        // randomly seed the food
        a.regen_food();
        a
    }

    pub fn rows(&self) -> u32 {
        self.dim.rows
    }

    pub fn cols(&self) -> u32 {
        self.dim.cols
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
            x: rand::thread_rng().gen_range(0..self.rows()) as i32,
            y: rand::thread_rng().gen_range(0..self.cols()) as i32,
        }
    }

    /// reconcile the state with the result of behavior trees
    pub fn reconcile(&mut self, bb: &Blackboard) {
        // move the snake
        let decided_move = bb.decided_move.expect("move has been decided in tick");
        self.snake.turn(decided_move);
        self.snake.move_forward();

        // decide if its alive - has the head collided
        let head = self.snake.head;
        let has_hit_self = self.snake.body.contains(&head);
        let has_gone_oob = self.dim.check_oob(head);
        self.snake.dead = has_hit_self || has_gone_oob;
    }
}
