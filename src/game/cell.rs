use std::ops::{Add};
use self::Cell::*;

type NextRotation = (isize, isize);

const LIGHT_GREY: [f32; 4] = [0.75, 0.75, 0.75, 1.0];
const DARK_GREY:  [f32; 4] = [0.18, 0.15, 0.15, 1.0];
const MID_GREY:   [f32; 4] = [0.55, 0.55, 0.55, 1.0];

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Cell {
    Tetro(NextRotation),
    Left,
    Right,
    Bottom,
    Inner,
    Top,
    Clash(&'static Self),
}

impl Add for Cell {
    type Output = Result<Cell, String>;

    fn add(self, other: Cell) -> Result<Cell, String> {
        match (self, other) {
            (Inner, Tetro(a))    => Ok(Tetro(a)),
            (Tetro(_), Tetro(_)) => Ok(Clash(&Inner)),
            (Bottom, Tetro(_))   => Ok(Clash(&Bottom)),
            (Left, Tetro(_))     => Ok(Clash(&Left)),
            (Right, Tetro(_))    => Ok(Clash(&Right)),
            (Top, Tetro(_))      => Ok(Clash(&Top)),
            _                    => Err(format!("{:?} + {:?} is not a valid operation", self, other))
        }
    }
}

impl Cell {
    pub fn render_color(&self) -> Result<[f32; 4], String> {
        match self {
            Inner         => Ok(LIGHT_GREY),
            Tetro(_)      => Ok(DARK_GREY),
            Bottom        => Ok(LIGHT_GREY),
            Left          => Ok(LIGHT_GREY),
            Right         => Ok(LIGHT_GREY),
            Top           => Ok(LIGHT_GREY),
            Clash(&Top)   => Ok(MID_GREY),
            Clash(&Inner) => Ok(MID_GREY),
            _             => Err(format!("Type {:?} doesn't render a color", self))
        }
    }

    pub fn rotate(&mut self) -> Result<Cell, String> {
        let current_rotation = match self {
            Tetro(rotation) => rotation,
            _               => return Err(format!("Can't rotate a cell of type {:?}", self))
        };

        Ok(
            Tetro((-current_rotation.1, current_rotation.0))
        )
    }
}

