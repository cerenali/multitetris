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

pub const BLOCK_SIZE: i64 = 30;
pub const BOARD_WIDTH: i64 = 10; // 10 cells across
pub const BOARD_HEIGHT: i64 = 22; // 22 cells up n down

pub const BOARD_BKD_COLOR: [f32; 4] = [0.18, 0.18, 0.18, 1.0]; // dark gray
pub const RED: [f32; 4] = [0.96, 0.12, 0.12, 1.0];
pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const BRIGHT_GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

pub const NUM_BOARDS: i64 = 2; // number of boards

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    boards: Vec<board::Board>, // the game board
}

impl App {
    fn render(&mut self, args: &RenderArgs) {

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BOARD_BKD_COLOR, gl);
        });


        for (i, board) in &mut self.boards.iter().enumerate() {
            // do nothing if paused
            if board.state == board::GameState::Paused {
                // return
                continue
            }

            // show game over screen if game is done
            if board.state == board::GameState::Over {
                self.gl.draw(args.viewport(), |c, gl| {
                    rectangle(RED,
                             [(BOARD_WIDTH * BLOCK_SIZE) as f64 * i as f64,
                                0.0,
                                (BOARD_WIDTH * BLOCK_SIZE) as f64,
                                (BOARD_HEIGHT * BLOCK_SIZE) as f64],
                             c.transform.trans(0.0, 0.0),
                             gl);
                });
                continue
            }

            // show gameplay screen
            let cells = board.cells;
            
            let blocks = board.current_piece.blocks;
            let current_x = board.current_piece.x_offset;
            let current_y = board.current_piece.y_offset;
            let piece_color = board.current_piece.color;

            self.gl.draw(args.viewport(), |c, gl| {
                // draw board???
                // iterate thru board cells and draw in filled-in blocks
                for row in 0..cells.len() {
                    for col in 0..cells[0].len() {
                        if cells[row][col] == 1 {
                            let x: f64 = col as f64 + (BOARD_WIDTH * i as i64) as f64;
                            let y: f64 = row as f64;
                            let size: f64 = BLOCK_SIZE as f64;
                            let b = rectangle::square(x * size, y * size, size);
                            rectangle(WHITE, b, c.transform.trans(0.0, 0.0), gl);
                        }
                    }
                }

                // iterate thru current piece and draw its current location
                for row in 0..blocks.len() {
                    for col in 0..blocks[0].len() {
                        if blocks[row][col] == 1 {
                            let x: f64 = (col as f64) + current_x + (BOARD_WIDTH * i as i64) as f64;
                            let y: f64 = (row as f64) + current_y;
                            let size: f64 = BLOCK_SIZE as f64;
                            let b = rectangle::square(x * size, y * size, size);
                            rectangle(piece_color, b, c.transform.trans(0.0, 0.0), gl);
                        }
                    }
                }
            });
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        for mut board in &mut self.boards {
            if board.state != board::GameState::Playing {
                // return
                continue
            }
            board.advance_board();
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "t e t r i s",
            [(BLOCK_SIZE * BOARD_WIDTH * NUM_BOARDS) as u32, (BLOCK_SIZE * BOARD_HEIGHT) as u32]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut boards: Vec<board::Board> = Vec::new();
    for i in 0..NUM_BOARDS {
        boards.push(Board::init_board());
    }
    let mut app = App {
        gl: GlGraphics::new(opengl),
        boards: boards,
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
            for board in &mut app.boards {
                board.handle_key_press(&i);
            }
        }
    }
}

