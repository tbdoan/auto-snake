use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::time::Duration;

use bonsai_bt as bbt;

use crate::arena::Arena;
use crate::arena::MoveDirection;
use crate::bfs::bfs;
use crate::blackboard::Blackboard;
use crate::blackboard::ShortestPathToFood;

#[derive(Clone, Debug)]
pub enum Action {
    FindPathToFood,
    SelectMove,
    // ShouldTurnLeft,
    // ShouldTurnRight,
    // TurnLeft,
    // TurnRight,
    // MoveForward,
    // StillAlive,
    // GameOver,

    // bbt wrappers
    BbtRunning,
}

pub fn snake_behavior<'a>() -> bbt::Behavior<Action> {
    bbt::Sequence(vec![
        // always succeed allows for fallback
        bbt::AlwaysSucceed(Box::new(bbt::Action(Action::FindPathToFood))),
        bbt::Action(Action::SelectMove),
        // never exit the tree - will change when we have multiple snakes
        bbt::Action(Action::BbtRunning),
        // bbt::If(
        //     Box::new(bbt::Action(Action::StillAlive)),
        //     Box::new(bbt::Action(Action::BbtRunning)),
        //     Box::new(bbt::Action(Action::GameOver)),
        // ),
    ])
    .memory(false)
}

pub fn snake_tick(
    arena: &mut Arena,
    dt: Duration,
    bt: &mut bbt::BT<Action, Blackboard>,
) -> bbt::Status {
    // clean slate the blackboard
    let bb = bt.blackboard_mut();
    bb.decided_move = None;
    bb.shortest_path_to_food = None;

    let evt: bbt::Event = bbt::UpdateArgs {
        dt: dt.as_secs_f64(),
    }
    .into();

    let (status, _) = bt
        .tick(&evt, &mut |args, bb| {
            let status = match args.action {
                // Action::StillAlive => tick_still_alive(arena),
                // Action::GameOver => tick_game_over(),
                Action::FindPathToFood => tick_find_path_to_food(arena, bb),
                Action::SelectMove => tick_select_move(arena, bb),

                // sentinel states
                Action::BbtRunning => bbt::Running,
            };
            (status, args.dt)
        })
        .expect("all ticks produce status and float");
    return status;
}

fn tick_find_path_to_food(arena: &Arena, bb: &mut Blackboard) -> bbt::Status {
    let food = match arena.food {
        Some(f) => f,
        None => {
            log::warn!("no food to route to");
            return bbt::Failure;
        }
    };
    let obstacles = Vec::from(arena.snake.body.clone());
    let path = bfs(arena.snake.head, food, arena.dim, &obstacles);

    // no path found to food
    if path.is_empty() {
        log::warn!("no path to food found");
        return bbt::Failure;
    }

    // cache the next direction
    let next_planned = path
        .get(1)
        .expect("element at index 1 is first non-head coord");
    let direction_to = arena.snake.head.direction_to(*next_planned);
    assert!(
        direction_to != arena.snake.direction.opposite(),
        "snake cannot do a 180"
    );
    bb.shortest_path_to_food = Some(ShortestPathToFood {
        path,
        first_direction: direction_to,
    });
    // we can decide inline because this is the only pathing solution
    bb.decided_move = Some(direction_to);
    bbt::Success
}

fn tick_select_move(arena: &Arena, bb: &mut Blackboard) -> bbt::Status {
    bb.decided_move = Some(match &bb.shortest_path_to_food {
        Some(sp) => sp.first_direction,
        // no path, keep going in current direction
        None => arena.snake.direction,
    });
    bbt::Success
}

// fn tick_still_alive(arena: &mut Arena) -> bbt::Status {
//     // we assume the body is good - only head needs checking
//     let head = arena.snake.head;

//     // has it hit itself
//     if arena.snake.body.contains(&head) {
//         return bbt::Failure;
//     }

//     // has it gone out of bounds
//     if arena.dim.check_oob(head) {
//         return bbt::Failure;
//     }

//     bbt::Status::Success
// }

// fn tick_game_over() -> bbt::Status {
//     println!("Game over");
//     return bbt::Status::Success;
// }
