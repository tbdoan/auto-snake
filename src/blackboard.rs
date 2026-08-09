use crate::arena::Coord;
use crate::arena::Direction;

pub struct ShortestPathToFood {
    // for visibility
    pub path: Vec<Coord>,
    // for convenience - move direction from head to next coord
    // assumed to be valid (left turn, right turn, or straight, but NOT backward)
    pub first_direction: Direction,
}

pub struct Blackboard {
    /// If None, path planner has never run.
    pub shortest_path_to_food: Option<ShortestPathToFood>,
    /// At the end of the tick, this must be present.
    pub decided_move: Option<Direction>,
}

impl Blackboard {
    pub fn new() -> Self {
        Self {
            shortest_path_to_food: None,
            decided_move: None,
        }
    }
}
