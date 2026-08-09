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
    pub fn move_in(&self, dir: Direction) -> Self {
        let mut after = *self; // copy it
        match dir {
            Direction::Up => after.x -= 1,
            Direction::Down => after.x += 1,
            Direction::Left => after.y -= 1,
            Direction::Right => after.y += 1,
        };
        after
    }

    /// Panics if adj is not adjacent to self
    pub fn direction_to(&self, adj: Coord) -> Direction {
        assert!(
            (self.x - adj.x).abs() + (self.y - adj.y).abs() == 1,
            "coordinates must be adjacent"
        );
        for dir in Direction::enumerate() {
            if adj == self.move_in(dir) {
                return dir;
            }
        }

        unreachable!("a direction must be found")
    }

    pub fn neighbors(&self) -> Vec<Coord> {
        let mut ns = Vec::new();
        for dir in Direction::enumerate() {
            ns.push(self.move_in(dir));
        }
        ns
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
        };
        write!(f, "{}", s.to_uppercase())
    }
}

impl Direction {
    pub fn enumerate() -> Vec<Self> {
        vec![Self::Up, Self::Right, Self::Down, Self::Left]
    }

    pub fn _clockwise(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    pub fn _counterclockwise(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

pub struct Snake {
    pub head: Coord,
    /// not including head
    pub body: VecDeque<Coord>,
    pub direction: Direction,
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
    pub fn turn(&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }

    /// returns tail and favored tail direction
    pub fn grow(&mut self) -> (Coord, Direction) {
        let body_len = self.body.len();
        let (second_to_last, last): (Option<Coord>, Coord) = match body_len {
            0 => (None, self.head),
            1 => (Some(self.head), *self.body.get(0).expect("len = 1")),
            _ => (
                Some(*self.body.get(body_len - 2).expect("len > 1")),
                *self.body.get(body_len - 1).expect("len > 1"),
            ),
        };
        let direction_from_tail = match second_to_last {
            // if second to last exists, then we can find the direction
            Some(s) => s.direction_to(last),
            // else just make it the opposite of current
            None => self.direction.opposite(),
        };

        (last, direction_from_tail)
    }
}

#[derive(Clone, Copy)]
pub struct Dimensions {
    pub rows: u32,
    pub cols: u32,
}

impl Dimensions {
    pub fn check_oob(&self, c: &Coord) -> bool {
        c.x < 0 || c.x >= self.rows as i32 || c.y < 0 || c.y >= self.cols as i32
    }
}

/// global, game state
pub struct Arena {
    pub dim: Dimensions,
    pub food: Option<Coord>,
    pub snake: Snake,
}

impl Arena {
    pub fn new(rows: u32, cols: u32) -> Self {
        let mut a = Self {
            dim: Dimensions { rows, cols },
            // dummy value - immediately overwriten
            food: None,
            // snake at center, starting right
            snake: Snake {
                head: (rows as i32 / 2, cols as i32 / 2).into(),
                body: VecDeque::new(),
                direction: Direction::Right,
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
        let mut excl = HashSet::new();
        // exclude its last position
        if let Some(food) = self.food {
            excl.insert(food);
        }
        // exclude snake body
        excl.extend(self.snake.body.iter().copied());

        // reset food
        self.food = None;

        // if we cant find an empty slot, no food this tick
        let n_attempts = 3;
        for _ in 0..n_attempts {
            let cand = Coord {
                x: rand::thread_rng().gen_range(0..self.rows()) as i32,
                y: rand::thread_rng().gen_range(0..self.cols()) as i32,
            };

            if !excl.contains(&cand) {
                self.food = Some(cand);
                return;
            }
        }

        log::warn!("no locations for food found after {n_attempts} attempts");
    }

    pub fn collision(&self, c: &Coord) -> bool {
        let body_collision = self.snake.body.contains(c);
        let boundary_collision = self.dim.check_oob(c);
        body_collision || boundary_collision
    }

    /// reconcile the state with the result of behavior trees
    pub fn reconcile(&mut self, bb: &Blackboard) {
        // move the snake
        let decided_move = bb.decided_move.expect("move has been decided in tick");
        let predicted_head = self.snake.head.move_in(decided_move);
        if self.collision(&predicted_head) {
            // dont move snake into collided spot. simply mark it for dead
            self.snake.dead = true;
            return;
        }

        self.snake.turn(decided_move);
        self.snake.move_forward();

        // has it eaten the food
        if Some(self.snake.head) == self.food {
            let (cur_tail, direction_from_tail) = self.snake.grow();
            let prefer = cur_tail.move_in(direction_from_tail);
            let mut tail = None;
            if !self.collision(&prefer) {
                tail = Some(prefer);
            } else {
                log::warn!("cannot grow tail ideally");
                for nbor in cur_tail.neighbors() {
                    if !self.collision(&nbor) {
                        tail = Some(nbor);
                        break;
                    }
                }
            }

            // add the tail, if have
            if let Some(t) = tail {
                self.snake.body.push_back(t);
            } else {
                log::warn!("could not grow snake, no place to grow tail");
            }
            self.regen_food();
        }
    }
}
