use crossterm::cursor::Hide;
use crossterm::cursor::Show;
use crossterm::execute;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use std::io::stdout;
use std::time::Duration;

mod arena;
mod blackboard;
mod render;

use arena::Arena;

fn main() -> std::io::Result<()> {
    let mut arena = Arena::new(50, 25);
    let mut out = stdout();

    execute!(out, EnterAlternateScreen, Hide, Clear(ClearType::All))?;

    loop {
        arena.regen_food();

        render::render(&arena)?;

        std::thread::sleep(Duration::from_millis(100));
    }

    execute!(out, Show, LeaveAlternateScreen)?;

    Ok(())
}
