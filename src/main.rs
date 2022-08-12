use std::{io, io::Read, io::Write, thread, time::Duration};

type Cell = u8;
type Grid = Vec<Vec<Cell>>;

/// Prints a grid of cells on the screen.
fn print_grid(grid: &Grid) {
    for row in grid {
        for cell in row {
            print!("{} ", if *cell == 0 { "." } else { "■" });
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
            .map(|word| word.parse::<u8>().unwrap())
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

fn sum_neighbors(grid: &Grid, i: usize, j: usize) -> u8 {
    // These functions help us find the minimum and maximum index values in the
    // 8-neighborhood.
    let imin0 = |x| if x > 0 { x - 1 } else { 0 };
    let imax = |x, y| if x < y { x + 1 } else { y };
    // These are the minimum and maximum indexes in the 8-neighborhood.
    // Row indexes.
    let min_r_idx = imin0(i);
    let max_r_idx = imax(i, grid.len() - 1);
    // Column indexes.
    let min_c_idx = imin0(j);
    let max_c_idx = imax(j, grid[0].len() - 1);
    // Summing up the values of all neighbors of the cell (i, j).
    (min_r_idx..=max_r_idx).fold(0, |acc, ii| {
        // Iterating on row indexes.
        acc + (min_c_idx..=max_c_idx).fold(0, |acc, jj| {
            // Iterating on column indexes.
            acc + grid[ii][jj]
        })
        // We want only the sum of the neighbors of the cell (i, j); we
        // need to subtract the value of the cell.
    }) - grid[i][j]
}

/// Returns "true" if "cell" is alive when considering the number of live
/// neighbors "n_neig".
fn is_alive(cell: u8, n_neig: u8) -> bool {
    (cell > 0 && 1 < n_neig && n_neig < 4) || (cell == 0 && n_neig == 3)
}

fn next_generation(grid: &Grid) -> Grid {
    let mut sumgrid = vec![vec![0 as u8; grid[0].len()]; grid.len()];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let cell = grid[i][j];
            let sum = sum_neighbors(grid, i, j);
            sumgrid[i][j] = if is_alive(cell, sum) { 1 } else { 0 }
        }
    }
    sumgrid
}

fn main() -> io::Result<()> {
    // We expect the initial grid from STDIN.
    let mut grid = stdin_to_grid()?;
    // This clears the shell.
    print!("{esc}[2J", esc = 27 as char);
    // We iterate forever. We should stop the program with "Ctrl-c".
    for i in 0.. {
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
        // We calculate the next generation of cells following the game rules.
        grid = next_generation(&grid);
    }

    Ok(())
}

#[cfg(test)]
mod main_test {
    use super::{next_generation, text_to_grid};

    #[test]
    fn test_next_generation() {
        let prev_gen = "0 1 0 0\n\
                        1 0 0 0\n\
                        0 1 0 0";
        let next_gen = next_generation(&text_to_grid(prev_gen));
        assert_eq!(next_gen, [[0, 0, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0]]);
    }
}
