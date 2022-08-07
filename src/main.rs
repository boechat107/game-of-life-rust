type Cell = bool;
type Grid = Vec<Vec<Cell>>;

fn repr_cell(cell: Cell) -> &'static str {
    if cell {
        "■ "
    } else {
        ". "
    }
}

fn print_grid(grid: Grid) {
    for row in &grid {
        println!("");
        for cell in row {
            print!("{} ", repr_cell(*cell));
        }
    }
}

fn main() {
    print_grid(vec![vec![true, false], vec![false, true]]);
}
