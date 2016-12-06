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
use opengl_graphics::glyph_cache::GlyphCache;

use graphics::*;

use std::path::Path;
use std::fs::OpenOptions;
use board::Board;

mod block;
mod board;

pub const BLOCK_SIZE: i64 = 30;
pub const BOARD_WIDTH: i64 = 10; // 10 cells across
pub const BOARD_HEIGHT: i64 = 22; // 22 cells up n down

pub const NUM_BOARDS: i64 = 2; // number of boards

pub const FONT_SIZE: u32 = 24;
const SCORE_LEFT_MARGIN: f64 = 15.0;
const SCORE_TOP_MARGIN: f64 = 35.0;

const GAMEOVER_LEFT_MARGIN: f64 = 70.0;
const GAMEOVER_TOP_MARGIN: f64 = 250.0;

const BOARD_BKD_COLOR: [f32; 4] = [0.18, 0.18, 0.18, 1.0]; // dark gray
// const RED: [f32; 4] = [0.96, 0.12, 0.12, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
// const BRIGHT_GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    boards: Vec<board::Board>, // game boards
    cache: GlyphCache<'static> // for drawing text
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
                continue
            }

            let font_cache = &mut self.cache;

            // show game over screen if game is done
            if board.state == board::GameState::Over {
                self.gl.draw(args.viewport(), |c, gl| {
                    rectangle(BLACK,
                             [(BOARD_WIDTH * BLOCK_SIZE) as f64 * i as f64,
                                0.0,
                                (BOARD_WIDTH * BLOCK_SIZE) as f64,
                                (BOARD_HEIGHT * BLOCK_SIZE) as f64],
                             c.transform.trans(0.0, 0.0),
                             gl);

                    // draw game over message
                    let mut text = graphics::Text::new(FONT_SIZE);
                    text.color = WHITE;
                    let mut transform: graphics::context::Context =
                                c.trans(GAMEOVER_LEFT_MARGIN + (BLOCK_SIZE * BOARD_WIDTH * i as i64) as f64, GAMEOVER_TOP_MARGIN);
                    text.draw(&format!("GAME OVER"),
                              font_cache,
                              &c.draw_state,
                              transform.transform,
                              gl);
                    transform = transform.trans(0.0, FONT_SIZE as f64 + 40.0);
                    // not centered because i am a weenie
                    text.draw(&format!("   Score: {}", board.score),
                              font_cache,
                              &c.draw_state,
                              transform.transform,
                              gl);

                    // draw border
                    let rect_border = graphics::Rectangle::new_border(WHITE, 0.3);
                    rect_border.draw([(BOARD_WIDTH * BLOCK_SIZE) as f64 * i as f64,
                                     0.0,
                                     (BOARD_WIDTH * BLOCK_SIZE) as f64,
                                     (BOARD_HEIGHT * BLOCK_SIZE) as f64],
                        &c.draw_state,
                        c.transform,
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

            let ghost_blocks = board.ghost_piece.blocks;
            let ghost_x = board.ghost_piece.x_offset;
            let ghost_y = board.ghost_piece.y_offset;
            let ghost_piece_color = board.ghost_piece.color;

            self.gl.draw(args.viewport(), |c, gl| {
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

                // DRAW  G H O S T   P I E C E ?
                for row in 0..ghost_blocks.len() {
                    for col in 0..ghost_blocks[0].len() {
                        if ghost_blocks[row][col] == 1 {
                            let x: f64 = (col as f64) + ghost_x + (BOARD_WIDTH * i as i64) as f64;
                            let y: f64 = (row as f64) + ghost_y;
                            let size: f64 = BLOCK_SIZE as f64;
                            let b = rectangle::square(x * size, y * size, size);
                            rectangle(ghost_piece_color, b, c.transform.trans(0.0, 0.0), gl);
                        }
                    }
                }

                // draw score
                let mut text = graphics::Text::new(FONT_SIZE);
                text.color = WHITE;
                let transform: graphics::context::Context =
                            c.trans(SCORE_LEFT_MARGIN + (BLOCK_SIZE as f64) * (BOARD_WIDTH * i as i64) as f64, SCORE_TOP_MARGIN);
                text.draw(&format!("Score: {}", board.score),
                          font_cache,
                          &c.draw_state,
                          transform.transform,
                          gl);

                // draw border
                let rect_border = graphics::Rectangle::new_border(WHITE, 0.3);
                rect_border.draw([(BOARD_WIDTH * BLOCK_SIZE) as f64 * i as f64,
                                 0.0,
                                 (BOARD_WIDTH * BLOCK_SIZE) as f64,
                                 (BOARD_HEIGHT * BLOCK_SIZE) as f64],
                    &c.draw_state,
                    c.transform,
                    gl);
            });

        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        for mut board in &mut self.boards {
            if board.state != board::GameState::Playing {
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

    let font_path = match OpenOptions::new().read(true).open("Lato-Light.ttf") {
        Ok(_) => Path::new("Lato-Light.ttf"),
        Err(_) => {
            match OpenOptions::new().read(true).open("src/Lato-Light.ttf") {
                Ok(_) => Path::new("src/Lato-Light.ttf"),
                Err(_) => panic!("No font file found")
            }
        }
    };

    // Create a new game and run it.
    let mut boards: Vec<board::Board> = Vec::new();
    for i in 0..NUM_BOARDS { // TODO: update num_boards from server
        boards.push(Board::init_board());
    }
    let mut app = App {
        gl: GlGraphics::new(opengl),
        boards: boards,
        cache: GlyphCache::new(font_path).unwrap()
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
            // TODO: send new keystroke to server

            // TODO: send messages:
            // new block name
            // keystroke
            // new score
            // GameState
        }
    }
}

