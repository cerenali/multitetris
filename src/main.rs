extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use graphics::*;

mod block;
mod board;

use board::Board;

pub const BLOCK_SIZE: f64 = 30.0;
pub const BOARD_WIDTH: u32 = 10; // 10 cells across
pub const BOARD_HEIGHT: u32 = 22; // 22 cells up n down

pub const BOARD_BKD_COLOR: [f32; 4] = [0.18, 0.18, 0.18, 1.0]; // dark gray
pub const RED: [f32; 4] = [0.96, 0.12, 0.12, 1.0];
pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const BRIGHT_GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    board: board::Board // the game board
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        if self.board.state != board::GameState::Continue {
            return
        }
        let cells = self.board.cells;
        
        let blocks = self.board.current_piece.blocks;
        let current_x = self.board.current_piece.x_offset;
        let current_y = self.board.current_piece.y_offset;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BOARD_BKD_COLOR, gl);

            // draw board???
            // iterate thru board cells and draw in filled-in blocks
            for (r, row) in cells.iter().enumerate() {
                for (col, colblock) in row.iter().enumerate() {
                    if *colblock == 1 {
                        let x: f64 = col as f64;
                        let y: f64 = r as f64;
                        let b = rectangle::square(x * BLOCK_SIZE, y * BLOCK_SIZE, BLOCK_SIZE);
                        rectangle(BRIGHT_GREEN, b, c.transform.trans(0.0, 0.0), gl);
                    }
                }
            }


            // iterate thru current piece and draw its current location
            for (r, row) in blocks.iter().enumerate() {
                for (col, colblock) in row.iter().enumerate() {
                    if *colblock == 1 {
                        let x: f64 = (col as f64) + (current_x as f64);
                        let y: f64 = (r as f64) + (current_y as f64);
                        let b = rectangle::square(x * BLOCK_SIZE, y * BLOCK_SIZE, BLOCK_SIZE);
                        rectangle(RED, b, c.transform.trans(0.0, 0.0), gl);
                    }
                }
            }
        });

        
    }

    fn update(&mut self, args: &UpdateArgs) {
        if self.board.state != board::GameState::Continue {
            return
        }
        self.board.advance_board();
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "t e t r i s",
            [(BLOCK_SIZE as u32) * BOARD_WIDTH, (BLOCK_SIZE as u32) * BOARD_HEIGHT]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        board: Board::init_board()
    };

    let mut events = window.events();
    events = events.ups(5);

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Event::Input(i) = e {
            app.board.handle_key_press(i);
        }
    }
}

