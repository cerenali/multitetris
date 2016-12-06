
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
extern crate websocket;

use std::thread;
use std::sync::mpsc::channel;
use websocket::{Message, Sender, Receiver};
use websocket::message::Type;
use websocket::client::request::Url;
use websocket::Client;
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
use block::TETROMINOS;


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
  cache: GlyphCache<'static>, // for drawing text
  token: i32,
  winner: i32
}

impl App {
  fn render(&mut self, args: &RenderArgs) {

    self.gl.draw(args.viewport(), |_, gl| {
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

  fn update(&mut self) {
    for mut board in &mut self.boards {
      if board.state != board::GameState::Playing {
        continue
      }
      board.advance_board();
    }
  }
}

fn main() {
  let url = Url::parse("ws://127.0.0.1:3012").unwrap();
  println!("Connecting to {}", url);

  let request = Client::connect(url).unwrap();
  let response = request.send().unwrap(); // Send the request and retrieve a response
  println!("Validating response...");

  response.validate().unwrap(); // Validate the response
  println!("Successfully connected");

  let mut token = 0;
  let (mut sender, mut receiver) = response.begin().split();
  let (tx, rx) = channel();

  let tx_1 = tx.clone();
  let tx_2 = tx.clone();

  let send_loop = thread::spawn(move || {
    loop {
      // Send loop
      let message: Message = match rx.recv() {
        Ok(m) => m,
        Err(e) => {
          println!("Error: Send Loop: {:?}", e);
          return;
        }
      };
      match message.opcode {
        Type::Close => {
          let _ = sender.send_message(&message);
          // If it's a close message, just send it and then return.
          return;
        },
        _ => (),
      }
      // Send the message
      match sender.send_message(&message) {
        Ok(()) => (),
        Err(e) => {
          println!("Error: Send Loop: {:?}", e);
          let _ = sender.send_message(&Message::close());
          return;
        }
      }
    }
  });

  let (tx1, rx1) = channel();
  let tx1_1 = tx1.clone();
  let receive_loop = thread::spawn(move || {
    // Receive loop
    for message in receiver.incoming_messages() {
      let message: Message = match message {
        Ok(m) => m,
        Err(e) => {
          println!("Error: Receive Loop: {:?}", e);
          let _ = tx_1.send(Message::close());
          return;
        }
      };
      match message.opcode {
        Type::Close => {
          // Got a close message, so send a close message and return
          let _ = tx_1.send(Message::close());
          return;
        }
        Type::Ping => match tx_1.send(Message::pong(message.payload)) {
          // Send a pong in response
          Ok(()) => (),
          Err(e) => {
            println!("Error: Receive Loop: {:?}", e);
            return;
          }
        },
        // Pass along to games the message we received
        _ => {
          let message1 = String::from_utf8(message.payload.into_owned()).unwrap();
          tx1_1.send(Message::text(message1)).unwrap(); 
        }
      }
    }
  });
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
  for _ in 0..NUM_BOARDS {
    boards.push(Board::init_board());
  }

  // Create a new game and run it.
  let mut app = App {
    gl: GlGraphics::new(opengl),
    boards: boards,
    cache: GlyphCache::new(font_path).unwrap(),
    token: -1,
    winner: -1
  };

  loop {
    match rx1.recv() {
      Ok(msg) => {
        let message = String::from_utf8(msg.payload.into_owned()).unwrap();
        let mut split_msg = message.split_whitespace();
        let command = split_msg.next().unwrap();
        match command {
          "NUM_CONNS" => {
            if token == 0 {
              let _ = split_msg.next().unwrap();
              let token_val = split_msg.next().unwrap();
              token = token_val.parse::<i32>().unwrap();
              app.token = token;
              println!("Joined game as Player {}!\n", app.token);
              tx_2.send(Message::text(format!("CLIENT_ACK {}", token_val))).unwrap();
            }
          },
          "START!" => {
            println!("START!\n");
            tx_2.send(Message::text(format!("FIRST_BLOCK {} {:?}", app.token, app.boards[(app.token - 1) as usize].current_piece.name))).unwrap();
            break;
          },
          _ => { }
        }
      },
      _ => { }
    }
  };

  let mut events = window.events().ups(5);
  while let Some(e) = events.next(&mut window) {
    // handle commands from the server
    loop {
      match rx1.try_recv() {
        Ok(msg) => {
          let message = String::from_utf8(msg.payload.into_owned()).unwrap();
          match message.as_str() {
            "" => {
              continue;
            },
            _ => { }
          };
          let mut split_msg = message.split_whitespace();
          let command = split_msg.next().unwrap();
          match command {
            "KEYSTROKE" => {
              let client_num_str = split_msg.next().unwrap();
              let client_num = client_num_str.parse::<i32>().unwrap();
              if client_num != app.token {
                let keystroke_str = split_msg.next().unwrap();
                // update board for client number client_num
                let input_keystroke: Input = match keystroke_str {
                  "UP" => Input::Press(Button::Keyboard(Key::Up)),
                  "DOWN" => Input::Press(Button::Keyboard(Key::Down)),
                  "LEFT" => Input::Press(Button::Keyboard(Key::Left)),
                  "RIGHT" => Input::Press(Button::Keyboard(Key::Right)),
                  "SPACE" => Input::Press(Button::Keyboard(Key::Space)),
                  _ => Input::Press(Button::Keyboard(Key::A))
                };
                app.boards[(client_num - 1) as usize].handle_key_press(&input_keystroke);
              }
            },
            "FIRST_BLOCK" => {
              let client_num_str = split_msg.next().unwrap();
              let client_num = client_num_str.parse::<i32>().unwrap();
              if client_num != app.token {
                let new_block_str = split_msg.next().unwrap();
                let new_block = match new_block_str {
                  "I" => TETROMINOS[0],
                  "O" => TETROMINOS[1],
                  "T" => TETROMINOS[2],
                  "S" => TETROMINOS[3],
                  "Z" => TETROMINOS[4],
                  "J" => TETROMINOS[5],
                  "L" => TETROMINOS[6],
                  _ => TETROMINOS[0]
                };
                app.boards[(client_num - 1) as usize].current_piece = new_block;
                app.boards[(client_num - 1) as usize].update_ghost_piece();
              }
            },
            "NEW_BLOCK" => {
              let client_num_str = split_msg.next().unwrap();
              let client_num = client_num_str.parse::<i32>().unwrap();
              if client_num != app.token {
                let new_block_str = split_msg.next().unwrap();
                let new_block = match new_block_str {
                  "I" => TETROMINOS[0],
                  "O" => TETROMINOS[1],
                  "T" => TETROMINOS[2],
                  "S" => TETROMINOS[3],
                  "Z" => TETROMINOS[4],
                  "J" => TETROMINOS[5],
                  "L" => TETROMINOS[6],
                  _ => TETROMINOS[0]
                };
                app.boards[(client_num - 1) as usize].next_piece = new_block;
                app.boards[(client_num - 1) as usize].update_ghost_piece();
              }
            },
            "GAME_OVER" => {
              let winner_num_str = split_msg.next().unwrap();
              let winner_num = winner_num_str.parse::<i32>().unwrap();
              println!("==== GAME OVER ====\n WINNER: PLAYER {}, SCORE: {}", winner_num,
                       app.boards[(winner_num - 1) as usize].score);
              // process::exit(0);
            },
            _ => {
              //println!("Unknown command\n");
            }
          }
        },
        _ => {
          break;
        }
      };
    }
    
    //   handle keystroke in board
    if let Some(r) = e.render_args() {
      app.render(&r);
    }

    if let Some(_) = e.update_args() {
      if app.boards[(app.token - 1) as usize].new_block {
        // send new block message
        match tx.send(Message::text(format!("NEW_BLOCK {} {:?}", app.token, app.boards[(app.token - 1) as usize].next_piece.name))) { 
          Ok(()) => (),
          Err(e) => {
            println!("Error: Main Loop: {:?}", e);
            break;
          }
        }
      }
      // app.update(&u);
      app.update();
    }

    if let Event::Input(i) = e {
      app.boards[(app.token - 1) as usize].handle_key_press(&i);

      let mut keystroke = "";
      match i {
        Input::Press(but) => {
          match but {
            Button::Keyboard(Key::Up) => {
              keystroke = "UP";
            }
            Button::Keyboard(Key::Left) => {
              keystroke = "LEFT";
            }
            Button::Keyboard(Key::Right) => {
              keystroke = "RIGHT";
            }
            Button::Keyboard(Key::Down) => {
              keystroke = "DOWN";
            }
            Button::Keyboard(Key::Space) => {
              keystroke = "SPACE";
            }
            _ => {}
          }
        }
        _ => {}
      }
      if keystroke.len() > 0 {
        match tx.send(Message::text(format!("KEYSTROKE {} {}", app.token, keystroke))) { 
          Ok(()) => (),
          Err(e) => {
            println!("Error: Main Loop: {:?}", e);
            break;
          }
        }
      }
    }
    // check for game over; if game over then send GAME OVER msg
    let mut num_ended_games = 0;
    let mut winner = 0;
    let mut i = 0;
    for board in app.boards.iter() {
      if board.state == board::GameState::Over {
        num_ended_games += 1;
      }
      else {
        winner = i + 1;
      }
      i += 1;
    }
    if num_ended_games == app.boards.len() - 1 && app.winner == -1 {
      // send game over message
      app.winner = winner;
      match tx.send(Message::text(format!("GAME_OVER {}", app.winner))) { 
        Ok(()) => (),
        Err(e) => {
          println!("Error: Main Loop: {:?}", e);
          break;
        }
      }
    }
  }

  // Exiting
  let _ = send_loop.join();
  let _ = receive_loop.join();
}

