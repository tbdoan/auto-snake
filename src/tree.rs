use std::time::Duration;

use bonsai_bt as bbt;

use crate::arena::Arena;

#[derive(Clone, Debug)]
pub enum Action {
    MoveForward,
    StillAlive,
    GameOver,

    // bbt wrappers
    BbtRunning,
}

pub fn snake_behavior<'a>() -> bbt::Behavior<Action> {
    bbt::Sequence(vec![
        bbt::Action(Action::MoveForward),
        // if alive, continue, else gameover
        bbt::If(
            Box::new(bbt::Action(Action::StillAlive)),
            Box::new(bbt::Action(Action::BbtRunning)),
            Box::new(bbt::Action(Action::GameOver)),
        ),
    ])
    .memory(false)
}

pub fn snake_tick(arena: &mut Arena, dt: Duration, bt: &mut bbt::BT<Action, ()>) -> bbt::Status {
    let evt: bbt::Event = bbt::UpdateArgs {
        dt: dt.as_secs_f64(),
    }
    .into();

    let (status, _) = bt
        .tick(&evt, &mut |args, _| {
            let status = match args.action {
                Action::MoveForward => tick_move_forward(arena),
                Action::StillAlive => tick_still_alive(arena),
                Action::GameOver => tick_game_over(),
                Action::BbtRunning => bbt::Running,
            };
            (status, args.dt)
        })
        .expect("all ticks produce status and float");
    return status;
}

fn tick_move_forward(arena: &mut Arena) -> bbt::Status {
    log::error!("snake head: {}", arena.snake.head);
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
    // supposedly, it is wrapping subtract on a unsigned type, so it'll end up very very large
    if head.x >= arena.rows || head.y >= arena.cols {
        return bbt::Status::Failure;
    }

    bbt::Status::Success
}

fn tick_game_over() -> bbt::Status {
    println!("Game over");
    return bbt::Status::Success;
}
