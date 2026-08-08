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

    let fps = 10;

    loop {
        // arena.regen_food();

        render::render(&arena)?;

        // 10 fps
        std::thread::sleep(Duration::from_secs(1) / fps);
    }
}
