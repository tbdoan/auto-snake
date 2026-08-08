use crossterm::cursor::MoveTo;
use crossterm::execute;
use std::io::stdout;
use std::io::Write;

use crate::arena::Arena;

pub fn render(arena: &Arena) -> std::io::Result<()> {
    let mut out = stdout();
    execute!(out, MoveTo(0, 0))?;

    let w = arena.width;
    let h = arena.height;
    let food = arena.food;

    // top and bottom border
    for y in 0..h + 2 {
        // left and right border
        for x in 0..w + 2 {
            let ch = if x == 0 || y == 0 || x == w + 1 || y == h + 1 {
                '#'
            } else if x - 1 == food.x && y - 1 == food.y {
                '●'
            } else {
                ' '
            };
            print!("{ch}");
        }
        println!();
    }

    out.flush()
}
