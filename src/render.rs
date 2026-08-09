use crossterm::cursor::MoveTo;
use crossterm::execute;
use std::io::stdout;
use std::io::Write;

use crate::arena::Arena;
use crate::arena::Coord;

pub fn render(arena: &Arena) -> std::io::Result<()> {
    let mut out = stdout();
    execute!(out, MoveTo(0, 0))?;

    let rows = arena.rows() as usize;
    let cols = arena.cols() as usize;

    // setup grid - extra room for the borders
    let mut grid = vec![vec![' '; cols + 2]; rows + 2];

    // draw the borders
    #[allow(clippy::needless_range_loop)]
    for x in 0..rows + 2 {
        for y in 0..cols + 2 {
            if x == 0 || y == 0 || x == rows + 1 || y == cols + 1 {
                grid[x][y] = '#';
            }
        }
    }

    // draw the snake(s)
    let snake = &arena.snake;
    {
        let (x, y) = shift(snake.head);
        grid[x][y] = if snake.dead { '░' } else { '█' };
    }
    for coord in &snake.body {
        let (x, y) = shift(*coord);
        grid[x][y] = if snake.dead { '░' } else { '▓' };
    }

    // add the food
    if let Some(food) = arena.food {
        let (x, y) = shift(food);
        assert_eq!(grid[x][y], ' ', "food is drawn on empty cell");
        grid[x][y] = '●';
    }

    // print the grid
    for row in &grid {
        println!("{}", row.iter().collect::<String>());
    }
    out.flush()
}

// to drawing frame
pub fn shift(c: Coord) -> (usize, usize) {
    // account for the borders
    (c.x as usize + 1, c.y as usize + 1)
}
