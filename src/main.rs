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
mod nodes;
mod render;
mod tree;

use arena::Arena;

fn main() -> Result<()> {
    let mut arena = Arena::new(50, 25);
    let mut out = stdout();

    execute!(out, EnterAlternateScreen, Hide, Clear(ClearType::All))?;

    loop {
        arena.regen_food();

        render::render(&arena)?;

        std::thread::sleep(Duration::from_millis(100));
    }
}
