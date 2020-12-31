use rand::prelude::*;
use rand_pcg::Pcg64;
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum LinkDirections {
    East,
    West,
    North,
    South,
}

#[derive(Debug, Default, Clone)]
pub struct Cell {
    row: usize,
    column: usize,
    links: HashSet<LinkDirections>,
}

impl Cell {
    pub fn new(row: usize, column: usize) -> Cell {
        Cell {
            row,
            column,
            links: HashSet::new(),
        }
    }
    pub fn link(&mut self, direction: LinkDirections) {
        self.links.insert(direction);
    }

    pub fn unlink(&mut self, direction: LinkDirections) {
        self.links.remove(&direction);
    }
    pub fn linked(self, direction: LinkDirections) -> bool {
        self.links.contains(&direction)
    }
}

#[derive(Debug, Default)]
pub struct Grid {
    rows: usize,
    columns: usize,
    grid: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(rows: usize, columns: usize) -> Self {
        let mut grid = Vec::with_capacity(rows);
        for row in 0..rows {
            let mut cells = Vec::with_capacity(columns);
            for column in 0..columns {
                cells.push(Cell::new(row, column));
            }
            grid.push(cells);
        }
        //let mut grid = vec![vec![Cell::default(); columns]; rows];
        Grid {
            rows,
            columns,
            grid,
        }
    }

    pub fn link(&mut self, row: usize, column: usize, direction: LinkDirections) {
        let clone = direction.clone();
        let match_cell = match direction {
            LinkDirections::North if row + 1 < self.rows => {
                Some(((row + 1, column), LinkDirections::South))
            }
            LinkDirections::East if column + 1 < self.columns => {
                Some(((row, column + 1), LinkDirections::West))
            }
            LinkDirections::South if row > 0 => Some(((row - 1, column), LinkDirections::North)),
            LinkDirections::West if column > 0 => Some(((row, column - 1), LinkDirections::East)),
            _ => None,
        };
        if let Some(other_cell) = match_cell {
            self.grid[other_cell.0 .0][other_cell.0 .1].link(other_cell.1);
        }
        self.grid[row][column].link(clone);
    }

    pub fn neighbors(self, cell: Cell) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        // add west
        if cell.row > 0 {
            neighbors.push((cell.row - 1, cell.column));
        }

        // add east
        if cell.row < self.rows {
            neighbors.push((cell.row + 1, cell.column));
        }

        // add north
        if cell.column > 0 {
            neighbors.push((cell.row, cell.column - 1));
        }

        // add north
        if cell.column > self.columns {
            neighbors.push((cell.row, cell.column + 1));
        }
        neighbors
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted_title = format!("{:_^1$}", "", (self.columns * 3) + 1);
        write!(f, "{}\n", formatted_title)?;
        for row in self.grid.iter().rev() {
            write!(f, "|")?;
            for cell in row.iter() {
                let links = vec![
                    cell.links.contains(&LinkDirections::North),
                    cell.links.contains(&LinkDirections::East),
                    cell.links.contains(&LinkDirections::South),
                    cell.links.contains(&LinkDirections::West),
                ];
                let block = match links.as_slice() {
                    [true, false, false, false] => "__|",

                    [true, true, false, false] => "___",
                    [true, true, false, true] => "___",
                    [true, true, true, false] => "   ",
                    [true, true, true, true] => "   ", // no walls

                    [true, false, true, false] => "  |",
                    [true, false, true, true] => "  |",

                    [true, false, false, true] => "__|",

                    [false, false, false, false] => "|*|", //all walls

                    [false, true, false, false] => "___",
                    [false, true, false, true] => "___",
                    [false, true, true, false] => "   ",
                    [false, true, true, true] => "   ",

                    [false, false, true, false] => "  |",
                    [false, false, true, true] => "  |",

                    [false, false, false, true] => "__|",
                    _ => "✱✱✱",
                };
                write!(f, "{}", block)?;
            }

            write!(f, "\n")?;
        }
        Ok(())
    }
}
// left bottom is 0,0
//  0 1 2 3 4 5
// 0
// 1
// 2
// 3
// 4
// 5
pub fn sidewinder(base: &mut Grid) {
    let mut rng = Pcg64::seed_from_u64(2);
    for row in (0..base.rows) {
        let mut run = vec![];
        for column in (0..base.columns) {
            run.push((row, column));
            let far_east = column == base.columns - 1;
            let far_north = row == base.rows - 1;
            let coin: u8 = rng.gen();
            let close_out = far_east || (!far_north && coin % 2 == 0);
            if close_out {
                let member = run.choose(&mut rng).unwrap();
                if member.0 < base.rows - 1 {
                    // is not at the top
                    base.link(member.0, member.1, LinkDirections::North);
                }
                run.clear();
            } else {
                base.link(row, column, LinkDirections::East);
            }
        }
    }
}
