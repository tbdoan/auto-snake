use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::time::Duration;

use bonsai_bt as bbt;

use crate::arena::Arena;
use crate::arena::MoveDirection;
use crate::bfs::bfs;
use crate::blackboard::Blackboard;

#[derive(Clone, Debug)]
pub enum Action {
    FindPathToFood,
    ShouldTurnLeft,
    ShouldTurnRight,
    TurnLeft,
    TurnRight,
    MoveForward,
    StillAlive,
    GameOver,

    // bbt wrappers
    BbtRunning,
}

pub fn snake_behavior<'a>() -> bbt::Behavior<Action> {
    let checked_turn_left = bbt::Sequence(vec![
        bbt::Action(Action::ShouldTurnLeft),
        bbt::Action(Action::TurnLeft),
    ]);
    let checked_turn_right = bbt::Sequence(vec![
        bbt::Action(Action::ShouldTurnRight),
        bbt::Action(Action::TurnRight),
    ]);
    let make_move = bbt::Select(vec![
        checked_turn_left,
        checked_turn_right,
        bbt::Action(Action::MoveForward),
    ]);
    bbt::Sequence(vec![
        make_move,
        // if alive, continue, else gameover
        bbt::If(
            Box::new(bbt::Action(Action::StillAlive)),
            Box::new(bbt::Action(Action::BbtRunning)),
            Box::new(bbt::Action(Action::GameOver)),
        ),
    ])
    .memory(false)
}

pub fn snake_tick(
    arena: &mut Arena,
    dt: Duration,
    bt: &mut bbt::BT<Action, Blackboard>,
) -> bbt::Status {
    let evt: bbt::Event = bbt::UpdateArgs {
        dt: dt.as_secs_f64(),
    }
    .into();

    let (status, _) = bt
        .tick(&evt, &mut |args, bb| {
            let status = match args.action {
                Action::MoveForward => tick_move_forward(arena),
                Action::StillAlive => tick_still_alive(arena),
                Action::GameOver => tick_game_over(),
                Action::FindPathToFood => tick_find_path_to_food(arena, bb),
                Action::ShouldTurnLeft => tick_should_turn_left(arena, bb),
                Action::TurnLeft => tick_turn_left(arena),
                Action::ShouldTurnRight => tick_should_turn_right(arena, bb),
                Action::TurnRight => tick_turn_right(arena),
                Action::BbtRunning => bbt::Running,
            };
            (status, args.dt)
        })
        .expect("all ticks produce status and float");
    return status;
}

// TODO: decouple the bfs algorithm from the arena
fn tick_find_path_to_food(arena: &Arena, bb: &mut Blackboard) -> bbt::Status {
    let obstacles = Vec::from(arena.snake.body.clone());
    let path = bfs(arena.snake.head, arena.food, arena.dim, &obstacles);

    // no path found to food
    if path.is_empty() {
        return bbt::Status::Failure;
    }

    // cache the next direction
    let nxt = path.get(1).expect("path is at least 2 long");
    for dir in MoveDirection::enumerate() {
        if nxt == &arena.snake.head.move_in(dir) {
            bb.new_direction = Some(dir);
            return bbt::Status::Success;
        }
    }

    unreachable!("expect to find a direction");
}

fn tick_move_forward(arena: &mut Arena) -> bbt::Status {
    arena.snake.move_forward();
    bbt::Status::Success
}

fn tick_should_turn_left(arena: &Arena, bb: &mut Blackboard) -> bbt::Status {
    match bb.new_direction {
        Some(nd) if nd == arena.snake.direction.clockwise() => bbt::Status::Success,
        _ => bbt::Status::Failure,
    }
}

fn tick_turn_left(arena: &mut Arena) -> bbt::Status {
    arena.snake.turn_left();
    arena.snake.move_forward();
    bbt::Status::Success
}

fn tick_should_turn_right(arena: &Arena, bb: &mut Blackboard) -> bbt::Status {
    match bb.new_direction {
        Some(nd) if nd == arena.snake.direction.counterclockwise() => bbt::Status::Success,
        _ => bbt::Status::Failure,
    }
}

fn tick_turn_right(arena: &mut Arena) -> bbt::Status {
    arena.snake.turn_right();
    arena.snake.move_forward();
    bbt::Status::Success
}

fn tick_still_alive(arena: &mut Arena) -> bbt::Status {
    // we assume the body is good - only head needs checking
    let head = arena.snake.head;

    // has it hit itself
    if arena.snake.body.contains(&head) {
        return bbt::Status::Failure;
    }

    // has it gone out of bounds
    if arena.dim.check_oob(head) {
        return bbt::Status::Failure;
    }

    bbt::Status::Success
}

fn tick_game_over() -> bbt::Status {
    println!("Game over");
    return bbt::Status::Success;
}
