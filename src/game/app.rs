use super::super::opengl_graphics::{GlGraphics};
use super::super::piston_window::*;
use std::time::{Instant, Duration};
use super::board::{Board,Movement};
use super::tetromino::{sort_tetro};
use self::GameState::*;
use std::{thread};



#[derive(PartialEq, Debug)]
pub enum GameState {
    NewTetromino,
    Moving(Instant),
    Spacing,
    GameOver,
    Restart,
}

pub struct App {
    board: Board,
    state: GameState,
    width_blocks: usize,
    height_blocks: usize,
    block_edge: f64,
    top_left_x: f64,
    top_left_y: f64,
    width: u32,
    height: u32,
    tetros_played: usize,
    lines_cleared: usize,
    pub background_color: [f32; 4],
    pub elements_color: [f32; 4]
}

impl App {
    pub fn new(width: u32, height: u32) -> Self {
        App {
            board: Board::default(),
            state: NewTetromino,
            width_blocks: 14,
            height_blocks: 22,
            block_edge: 25.0,
            top_left_x: (width as f64) * 0.12,
            top_left_y: (height as f64) * 0.15,
            width,
            height,
            tetros_played: 0,
            lines_cleared: 0,
            background_color: [0.75, 0.75, 0.75, 1.0],
            elements_color: [0.18, 0.15, 0.15, 1.0]
        }
    }

    pub fn process_state(&mut self) {
        use super::cell::Cell::*;
        
        match self.state {
            Restart => {
                self.board = Board::default();
                self.tetros_played = 0;
                self.lines_cleared = 0;
                self.state = NewTetromino
            },

            NewTetromino => {

                self.board.new_tetromino();
                self.tetros_played += 1;

                match self.board.current_tetro[0].cell {
                    Clash(x) if x == &Top || x == &Inner => {
                        let cleared = self.board.check_lines();
                        if cleared > 0 {
                            self.lines_cleared += cleared;
                            self.state = NewTetromino;
                        } else {
                            self.board.mark_top_clash();
                            self.state = GameOver;
                        }
                    },

                    Tetro(_) => self.state = Moving(Instant::now()),

                    _ => panic!("The state is not an acceptable state: {:?}", self.board.current_tetro)
                }
            },

            Spacing => {
                if !self.attempt_move(Movement::Down) {
                    self.board.mark_tetro();
                    self.lines_cleared += self.board.check_lines();
                    self.state = NewTetromino;
                }
                
                thread::sleep(Duration::from_millis(5));
            }

            Moving(started_moving) => {
                if started_moving.elapsed() >= Duration::from_millis(self.current_speed()) {
                    if self.attempt_move(Movement::Down) {
                        self.state = Moving(Instant::now());
                    } else {
                        self.board.mark_tetro();
                        self.lines_cleared += self.board.check_lines();
                        self.state = NewTetromino;
                    }
                }
            },

            GameOver => {}
        };
    }

    pub fn process_key(&mut self, args: &Button) {
        let key: &Key = match args {
            Button::Keyboard(pressed) => pressed,
            _                         => return, 
        };

        match self.state {
            Moving(_) => match key {
                Key::Right => { self.attempt_move(Movement::Right); },
                Key::Left  => { self.attempt_move(Movement::Left); },
                Key::Down  => {
                    if self.attempt_move(Movement::Down) {
                        self.state = Moving(Instant::now());
                    } else {
                        self.board.mark_tetro();
                        self.lines_cleared += self.board.check_lines();
                        self.state = NewTetromino;
                    }
                },
                Key::Up    => { self.attempt_move(Movement::Rotate); },
                Key::Space => { self.state = Spacing; },
                 _         => {},
            },

            GameOver => match key {
                Key::R => self.state = Restart,
                _      => {}
            }

            _ => {}
        }
    }

    fn attempt_move(&mut self, movement: Movement) -> bool  {
        use super::cell::Cell::*;

        let mut future_tetro = self.board.current_tetro.clone();
        self.board.simulate_move(&mut future_tetro, &movement);
        sort_tetro(&mut future_tetro);


        match future_tetro[0].cell {
            Tetro(_) => {
                self.board.current_tetro = future_tetro;
                true
            },

            Clash(_) => false,

            _        => panic!("Obey your new master!!! {:?}", future_tetro)
        }
    }

    fn current_speed(&self) -> u64 {
        let mut speed_factor: usize = self.tetros_played / 10;
       
        if speed_factor > 11 {
            speed_factor = 11;
        }
       
        (700 - 50 * speed_factor) as u64
    } 


    // RENDER
    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        clear(self.background_color, gl);
        let ctx = &Context::new_abs(self.width as f64, self.height as f64);
        
        gl.draw(args.viewport(), |_, gl| {
            // Render Cells
            for x in 0..self.width_blocks {
                for y in 0..self.height_blocks {
                    let square = self.get_cell_draw_area(x as f64, y as f64);
                    let color = self.board.cells[y][x].render_color()
                        .unwrap_or_else(|err| panic!(err.to_string()));
                    Rectangle::new(color).draw(square, &DrawState::default(), ctx.transform, gl);
                }
            }

            // Render Current Tetro Over Cells
            for el in self.board.current_tetro.iter() {
                let square = self.get_cell_draw_area(el.coords.0 as f64, el.coords.1 as f64);
                let color = el.cell.render_color()
                        .unwrap_or_else(|err| panic!(err.to_string()));
                Rectangle::new(color).draw(square, &DrawState::default(), ctx.transform, gl);
            }

            // Render Bottom Line
            let line = self.get_line_draw_area(
                (self.board.cells.len() - 4) as f64,
                (self.board.cells[0].len() - 4) as f64
            );
            Line::new(self.elements_color, 0.1).draw(line, &DrawState::default(), ctx.transform, gl);

            //Render Score
            Rectangle::new(self.elements_color)
                .draw(self.get_score_draw_area(), &DrawState::default(), ctx.transform, gl);

        }) 
    }

    fn get_cell_draw_area(&self, current_x: f64, current_y: f64) -> [f64; 4] {
        [
            self.top_left_x + current_x * self.block_edge,
            self.top_left_y + current_y * self.block_edge,
            self.block_edge,
            self.block_edge
        ]
    }

    fn get_line_draw_area(&mut self, board_height: f64, board_width: f64) -> [f64; 4] {
        let inner_top_left_x = self.top_left_x + self.block_edge * 2.0;
        let inner_top_left_y = self.top_left_y + self.block_edge * 2.0;

        [
            inner_top_left_x,
            inner_top_left_y + board_height * self.block_edge,
            inner_top_left_x + board_width * self.block_edge,
            inner_top_left_y + board_height * self.block_edge
        ]
    }

    fn get_score_draw_area(&mut self) -> [f64; 4] {
        let score = if self.lines_cleared > 100 {
            500.0
        } else {
            (self.lines_cleared * 5) as f64
        };

        [
            self.top_left_x + self.block_edge * 17.0,
            self.top_left_y + 22.0 * self.block_edge - score,
            2.0,
            score
        ]
    }
}
