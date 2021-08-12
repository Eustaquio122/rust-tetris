use super::cell::Cell;
use super::tetromino::{Tetromino,new_tetro,sort_tetro};
use self::Movement::*;

const DEFAULT_INNER_HEIGHT: usize = 20;
const DEFAULT_INNER_WIDTH: usize = 10;

type Cells = Vec<Vec<Cell>>;

#[derive(Clone, Debug)]
pub struct Board {
    pub cells: Cells,
    pub current_tetro: Tetromino
}

#[derive(Debug, PartialEq)]
pub enum Movement {
    Left,
    Right,
    Down,
    Rotate
}

impl Default for Board {
    fn default() -> Self {
        Board::new(DEFAULT_INNER_HEIGHT, DEFAULT_INNER_WIDTH)
    }
}

impl Board {
    pub fn new(inner_height: usize, inner_width: usize) -> Self {
        Board {
            cells: initialize(inner_height + 4, inner_width + 4),
            current_tetro: vec![]
        }
    }

    pub fn new_tetromino(&mut self)  {
        self.current_tetro = new_tetro();

        for el in self.current_tetro.iter_mut() {
            el.cell = (self.cells[el.coords.1][el.coords.0] + el.cell)
                .unwrap_or_else(|err| panic!("{}", err.to_string()));
        }

        sort_tetro(&mut self.current_tetro);
    }

    pub fn simulate_move(&mut self, future_tetro: &mut Tetromino, movement: &Movement) {
        if *movement == Rotate {
            self.rotate_tetro(future_tetro);
        } else {        
            self.apply_translation(future_tetro, &movement);
        }

        for el in future_tetro.iter_mut() {
            el.cell = (self.cells[el.coords.1][el.coords.0] + el.cell)
                .unwrap_or_else(|err| panic!("{}", err.to_string()));
        }
    }

    pub fn mark_top_clash(&mut self) {
        use Cell::*;

        for el in self.current_tetro.iter_mut() {
            el.cell = Clash(&Top);
        };
    }

    pub fn check_lines(&mut self) -> usize {
        for index in (2..(self.cells.len() - 2)).rev() {
            if self.is_line(index) {
                return self.clear_lines(index);
            }
        }

        0
    }

    pub fn is_line(&mut self, index: usize) -> bool {
        let width = self.cells[0].len();
        let inner_slice = &self.cells[index][2..width-2];

        for cell in inner_slice {
            match cell {
                Cell::Tetro(_) => {},
                _        => return false
            }
        }
 
        true
    }

    pub fn clear_lines(&mut self, index: usize) -> usize {
        let mut lines_cleared = 0;

        for i in (3..=index).rev() {
            while self.is_line(i) {
                self.clear_line(i);
                lines_cleared += 1;
            }
        }

        lines_cleared
     }

    fn apply_translation(&mut self, future_tetro: &mut Tetromino, movement: &Movement) {
        let translation: (isize, isize) = match movement {
                Left   => (-1, 0),
                Right  => (1,  0),
                Down   => (0,  1),
                _      => panic!("Cannot apply translation to movement {:?}", movement)
        };

        for el in future_tetro.iter_mut() {
            let new_coords = get_new_coords(el.coords, translation);
            el.coords = new_coords;
        }
    }

    fn rotate_tetro(&mut self, future_tetro: &mut Tetromino) {        
        for el in future_tetro.iter_mut() {
            let translation = match el.cell {
                Cell::Tetro(rotation) => rotation,
                _                     => panic!("Can't rotate a cell of type {:?}", el.cell)
            };

            el.coords = (
                ((el.coords.0 as isize) + translation.0) as usize, 
                ((el.coords.1 as isize) + translation.1) as usize
            );

            el.cell = el.cell.rotate()
                .unwrap_or_else(|err| panic!("{}", err.to_string()));
        }
    }

    pub fn mark_tetro(&mut self) {
        for el in self.current_tetro.iter_mut() {
            let (x, y) = el.coords;
            el.cell = (self.cells[y][x] + el.cell).unwrap_or_else(|err| panic!("{}", err.to_string()));
            self.cells[y][x] = el.cell;
        };
    }

    fn clear_line(&mut self, index: usize) {
        let width = self.cells[0].len();
        let inner_slice = &mut self.cells[index][2..width-2];

        for cell in inner_slice.iter_mut() {
            *cell = Cell::Inner;
        }

        let inner_bound = index + 1;
        let rotation_length = inner_bound - 3;
        self.cells[2..inner_bound].rotate_left(rotation_length);
    }
}

fn initialize(total_height: usize, total_width: usize) ->  Cells {
    use Cell::*;

    let mut cells: Cells = vec![];

    let top_row: Vec<Cell> = vec![Top; total_width];
    cells.push(top_row.clone());
    cells.push(top_row);

    for _ in 2..total_height - 2 {
        let mut row: Vec<Cell> = vec![];
        for y in 0..total_width {
            if y < 2 {
                row.push(Left);
            } else if y > total_width - 3 {
                row.push(Right);
            } else {
                row.push(Inner);
            }
        }
        cells.push(row);
    };

    let bottom_row: Vec<Cell> = vec![Bottom; total_width];
    cells.push(bottom_row.clone());
    cells.push(bottom_row);

    cells
}

fn get_new_coords(current_coords: (usize, usize), translation: (isize, isize)) -> (usize, usize) {
    (
        ((current_coords.0 as isize) + translation.0) as usize,
        ((current_coords.1 as isize) + translation.1) as usize
    )
}

