use crate::arena::Coord;



struct Blackboard {
    food: Coord,
    snake: Snake,
}

impl Blackboard {
    fn new() -> Self {
        Self {
            food: Coord::new(0, 0),
            body: Vec::new(),
            direction: Direction::Right,
        }
    }
}
