use crate::arena::MoveDirection;

pub struct Blackboard {
    // written to by the path planner
    pub new_direction: Option<MoveDirection>,
}

impl Blackboard {
    pub fn new() -> Self {
        Self {
            new_direction: None,
        }
    }
}
