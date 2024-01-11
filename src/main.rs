use grid::{Grid, KnownValue};
use itertools::Itertools;
use std::collections::VecDeque;

mod grid;

fn main() {
    let starting_grid = Grid::new(vec![
        KnownValue::new(1, 7, 2),
        KnownValue::new(2, 2, 8),
        KnownValue::new(2, 6, 7),
        KnownValue::new(2, 8, 9),
        KnownValue::new(3, 1, 6),
        KnownValue::new(3, 3, 2),
        KnownValue::new(3, 7, 5),
        KnownValue::new(4, 2, 7),
        KnownValue::new(4, 5, 6),
        KnownValue::new(5, 4, 9),
        KnownValue::new(5, 6, 1),
        KnownValue::new(6, 5, 2),
        KnownValue::new(6, 8, 4),
        KnownValue::new(7, 3, 5),
        KnownValue::new(7, 7, 6),
        KnownValue::new(7, 9, 3),
        KnownValue::new(8, 2, 9),
        KnownValue::new(8, 4, 4),
        KnownValue::new(8, 8, 7),
        KnownValue::new(9, 3, 6),
    ]);

    let mut search_queue: VecDeque<Grid> = VecDeque::new();
    let mut solutions = Vec::new();

    println!("Solving:");
    starting_grid.print_grid();
    println!();

    search_queue.push_front(starting_grid);

    while !search_queue.is_empty() {
        let grid = search_queue.pop_back().unwrap();

        // Find cell with fewest possibilities that hasn't been evaluated already
        let next_cell = grid
            .cells
            .iter()
            .enumerate()
            .filter(|(_, x)| !x.evaluated)
            .min_by_key(|(_, x)| x.possibilities.len());

        if next_cell.is_none() {
            println!("Solution:");
            grid.print_grid();
        }

        match next_cell {
            None => solutions.push(grid), // All cells have been evaluated
            Some((cell_position, cell)) => {
                for possibility in &cell.possibilities {
                    let mut next_grid = grid.clone();

                    // Update row, column and block and check for errors at the same time
                    let is_invalid =
                        get_neighbours(cell_position)
                            .into_iter()
                            .any(|neighbouring_cell| {
                                next_grid.remove_possibility(neighbouring_cell, *possibility);
                                next_grid.num_possibilities(neighbouring_cell) == 0
                            });

                    if is_invalid {
                        continue;
                    }

                    // Update cell
                    next_grid.set_value(cell_position, *possibility);

                    // Continue search from new state
                    search_queue.push_back(next_grid)
                }
            }
        }
    }
}

fn get_neighbours(cell: usize) -> Vec<usize> {
    let row = cell / 9; // Division rounds towards zero
    let column = cell % 9;

    let row_cells = (0..9).into_iter().map(|i| (row * 9) + i);
    let column_cells = (0..9).into_iter().map(|i| (i * 9) + column);

    let block_row = row / 3;
    let block_column = column / 3;

    let block_cells = (0..9).into_iter().map(|i| {
        let initial_position = (i % 3) + ((i / 3) * 9);
        let row_offset = block_row * 27;
        let column_offset = block_column * 3;

        initial_position + row_offset + column_offset
    });

    row_cells
        .chain(column_cells)
        .chain(block_cells)
        .unique()
        .filter(|&i| i != cell)
        .collect()
}
