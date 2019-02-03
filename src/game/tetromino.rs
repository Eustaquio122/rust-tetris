extern crate rand;

use rand::distributions::{Distribution, Standard};
use rand::Rng;
use self::Tetros::*;
use super::cell::Cell;

type StartingCoords = (usize, usize);
type FirstRotation  = (isize, isize);

pub type Tetromino = Vec<TetroCell>;

#[derive(Clone, Debug)]
pub struct TetroCell {
    pub cell: Cell,
    pub coords: StartingCoords
}

enum Tetros {
    I,
    O,
    T,
    J,
    L,
    S,
    Z
}

impl Distribution<Tetros> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tetros {
        match rng.gen_range(0,7) {
            0 => I,
            1 => O,
            2 => T,
            3 => J,
            4 => L,
            5 => S,
            _ => Z,
        }
    }
}

pub fn new_tetro() -> Tetromino {
    let kind: Tetros = rand::random();
    let new_tetro_cell = |(coords, (x, y)): (StartingCoords, FirstRotation)|
        TetroCell { cell: Cell::Tetro((x, y)), coords };

    to_points_vec(kind).into_iter().map(new_tetro_cell).collect()
}

pub fn sort_tetro(tetromino: &mut Tetromino) {
    tetromino.sort_by(|a, b| b.cell.cmp(&a.cell));
}

fn to_points_vec(kind: Tetros) -> Vec<(StartingCoords, FirstRotation)> {
    match kind {
        I => vec![
                    ((6, 3), (1, -1)),
                    ((7, 3), (0,  0)),
                    ((8, 3), (-1, 1)),
                    ((9, 3), (-2, 2))
                ],

        O => vec![
                    ((6, 3), (0, 0)),
                    ((7, 3), (0, 0)),
                    ((6, 4), (0, 0)),
                    ((7, 4), (0, 0))
                ],

        T => vec![
                    ((7, 3), (1,  1)),
                    ((6, 4), (1, -1)),
                    ((7, 4), (0,  0)),
                    ((8, 4), (-1, 1))
                ],

        J => vec![
                    ((6, 3), (1, -1)),
                    ((7, 3), (0,  0)),
                    ((8, 3), (-1, 1)),
                    ((8, 4), (-2, 0))
                ],

        L => vec![
                    ((6, 3), (1, -1)),
                    ((7, 3), (0,  0)),
                    ((8, 3), (-1, 1)),
                    ((6, 4), (0, -2))
                ],
        
        S => vec![
                    ((7, 3), (1,  1)),
                    ((8, 3), (0,  2)),
                    ((6, 4), (1, -1)),
                    ((7, 4), (0,  0))
                ],
        
        Z => vec![
                    ((6, 3), (2,  0)),
                    ((7, 3), (1,  1)),
                    ((7, 4), (0,  0)),
                    ((8, 4), (-1, 1))
                ],
    }
}




