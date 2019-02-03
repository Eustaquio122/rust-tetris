mod game;

extern crate piston_window;
extern crate opengl_graphics;
extern crate rand;

use opengl_graphics::{GlGraphics, OpenGL};
use piston_window::*;
use self::game::app::App;

fn main() {
    let width: u32 = 700;
    let height: u32 = 768;

    let mut window: PistonWindow = WindowSettings::new("Tetris", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Unable to create Piston Window: {}", e) });
    let mut gl = GlGraphics::new(OpenGL::V3_2);
    let mut app = App::new(width, height);

    while let Some(e) = window.next() {
        app.process_state();

        if let Some(args) = e.render_args() {
            app.render(&mut gl, &args);
        };

        if let Some(args) = e.press_args() {
            app.process_key(&args)
        }
    }
}
