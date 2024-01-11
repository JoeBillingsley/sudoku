pub struct KnownValue {
    row: usize,
    column: usize,
    value: usize,
}

impl KnownValue {
    pub fn new(row: usize, column: usize, value: usize) -> Self {
        Self {
            row: row - 1,
            column: column - 1,
            value,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub evaluated: bool,
    pub possibilities: Vec<usize>,
}
impl Cell {
    fn new() -> Self {
        Self {
            evaluated: false,
            possibilities: (1..10).collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Grid {
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new(fixed_values: Vec<KnownValue>) -> Grid {
        let mut grid = Grid {
            cells: vec![Cell::new(); 81],
        };

        for fixed_value in fixed_values {
            let pos = fixed_value.row * 9 + fixed_value.column;
            grid.cells[pos] = Cell {
                evaluated: false,
                possibilities: vec![fixed_value.value],
            }
        }

        grid
    }

    pub fn set_value(&mut self, cell: usize, value: usize) {
        self.cells[cell].possibilities = vec![value];
        self.cells[cell].evaluated = true
    }

    pub fn remove_possibility(&mut self, cell: usize, value: usize) {
        self.cells[cell].possibilities.retain(|&p| p != value);
    }

    pub fn num_possibilities(&self, cell: usize) -> usize {
        self.cells[cell].possibilities.len()
    }

    pub fn print_grid(&self) {
        self.cells.iter().enumerate().for_each(|(i, cell)| {
            if cell.possibilities.len() == 1 {
                print!("{} ", cell.possibilities[0]);
            } else {
                print!("  ");
            }

            if ((i + 1) % 9) == 0 {
                println!();
            }
        });
    }
}
