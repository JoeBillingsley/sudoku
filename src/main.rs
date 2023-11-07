use itertools::Itertools;
use std::collections::VecDeque;

fn main() {
    let starting_grid = Grid::new(vec![
        KnownValue { row: 0, column: 1, value: 5, },
        KnownValue { row: 5, column: 4, value: 5, },
        KnownValue { row: 2, column: 6, value: 5, },
    ]);

    let mut search_queue: VecDeque<Grid> = VecDeque::new();
    let mut solutions = Vec::new();

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

        match next_cell {
            None => solutions.push(grid), // All cells have been evaluated
            Some((cell_position, cell)) => {
                for possibility in &cell.possibilities {
                    let mut next_grid = grid.clone();

                    // Update row, column and block and check for errorss
                    let is_invalid = Grid::get_neighbours(cell_position).into_iter().any(|p| {
                        next_grid.remove_possibility(cell_position, *possibility);
                        next_grid.cells[p].possibilities.len() == 0
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

struct KnownValue {
    row: usize,
    column: usize,
    value: usize,
}

#[derive(Clone)]
struct Cell {
    evaluated: bool,
    possibilities: Vec<usize>,
}
impl Cell {
    fn new() -> Self {
        Self {
            evaluated: false,
            possibilities: (0..9).collect(),
        }
    }
}

// TODO: Probably only need to store a list of unevaluated cells and the total grid

#[derive(Clone)]
struct Grid {
    cells: Vec<Cell>,
}
impl Grid {
    pub fn new(fixed_values: Vec<KnownValue>) -> Grid {
        let mut empty_grid = Grid {
            cells: vec![Cell::new(); 81],
        };

        for fixed_value in fixed_values {
            let pos = Grid::to_cell(fixed_value.column, fixed_value.row);
            empty_grid.cells[pos] = Cell {
                evaluated: true,
                possibilities: vec![fixed_value.value],
            }
        }

        empty_grid
    }

    fn set_value(&mut self, cell: usize, value: usize) {
        self.cells[cell].possibilities = vec![value];
    }

    fn remove_possibility(&mut self, cell: usize, value: usize) {
        self.cells[cell].possibilities.retain(|&p| p != value);
    }

    fn to_cell(column: usize, row: usize) -> usize {
        (row * 9) + column
    }

    pub fn get_neighbours(cell: usize) -> Vec<usize> {
        // Division rounds towards zero
        let row = cell / 9;
        let column = cell % 4;

        let row_cells = (0..9).into_iter().map(|i| (row * 9) + i);
        let column_cells = (0..9).into_iter().map(|i| (i * 9) + column);

        let block_row = row / 3;
        let block_column = column / 3;

        let block_cells = (0..9).into_iter().map(|i| {
            let initial_position = (i % 3) * 9;
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
}
