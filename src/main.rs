use anyhow::Result;
use crossterm::cursor::Hide;
use crossterm::execute;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use crossterm::terminal::EnterAlternateScreen;
use std::io::stdout;
use std::time::Duration;

mod arena;
// mod blackboard;
mod render;
mod tree;

use arena::Arena;
use bonsai_bt as bbt;

use crate::tree::snake_behavior;
use crate::tree::snake_tick;

const RENDER: bool = true;

fn main() -> Result<()> {
    if !RENDER {
        env_logger::Builder::new()
            .filter_level(log::LevelFilter::Info)
            .init();
    }

    let mut arena = Arena::new(20, 40);

    if RENDER {
        let mut out = stdout();
        execute!(out, EnterAlternateScreen, Hide, Clear(ClearType::All))?;
        render::render(&arena)?; // initial render
    }

    let mut bt = bbt::BT::new(snake_behavior(), ()).with_telemetry(8080)?;

    // let fps = 10;
    let fps = 1;
    let tick_intvl = Duration::from_secs(1) / fps;

    loop {
        // arena.regen_food();

        let status = snake_tick(&mut arena, tick_intvl, &mut bt);
        match status {
            bbt::Status::Success => return Ok(()),
            bbt::Status::Failure => anyhow::bail!("tree failed"),
            bbt::Status::Running => {}
        }

        if RENDER {
            render::render(&arena)?;
        }

        std::thread::sleep(tick_intvl);
    }
}
