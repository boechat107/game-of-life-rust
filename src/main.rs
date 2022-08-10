use std::{io, io::Read, io::Write, thread, time::Duration};

type Cell = bool;
type Grid = Vec<Vec<Cell>>;

/// Prints a grid of cells on the screen.
fn print_grid(grid: &Grid) {
    for row in grid {
        for cell in row {
            print!("{} ", if *cell { "■ " } else { ". " });
        }
        println!();
    }
}

/// Parses a seed text and returns a Grid.
///
/// The seed is a multi-line text with 0s and 1s.
///
/// ```
/// 0 0 0
/// 0 1 0
/// 0 0 0
/// ```
fn text_to_grid(text: &str) -> Grid {
    fn line_to_cells(line: &str) -> Vec<Cell> {
        line.split_whitespace()
            .map(|word| word.chars().next() == Some('1'))
            .collect()
    }
    text.lines().map(line_to_cells).collect()
}

/// Returns a Grid from a seed given by STDIN.
fn stdin_to_grid() -> io::Result<Grid> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(text_to_grid(&buffer))
}

fn update_generation(grid: &mut Grid) {
    grid;
    // TODO
}

fn main() -> io::Result<()> {
    // We expect the initial grid from STDIN.
    let mut grid = stdin_to_grid()?;
    // This clears the shell.
    print!("{esc}[2J", esc = 27 as char);
    // We iterate forever. We should stop the program with "Ctrl-c".
    for i in 0..5 {
        // We move the cursor to "Home" again to print over the previous
        // characters.
        print!("{esc}[H", esc = 27 as char);
        println!("Generation {}", i);
        print_grid(&grid);
        // "print!" is buffered. We should flush it to get the visual effect we
        // want.
        io::stdout().flush().unwrap();
        // Sleeping between generations.
        thread::sleep(Duration::from_millis(700));
        // We update the current generation of cells following the game rules.
        update_generation(&mut grid);
    }

    Ok(())
}
