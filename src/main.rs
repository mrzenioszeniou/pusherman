extern crate piston_window;

mod state;

use crate::state::{Cell, Dir, State};
use piston_window::*;

const CELL_SIZE_PX: f64 = 18_f64;
const BLACK_RGB: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const RED_RGB: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const WHITE_RGB: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn main() {
  let mut state = State::default();

  let map_size = state.size();
  let window_size = [
    map_size.0 as f64 * CELL_SIZE_PX,
    map_size.1 as f64 * CELL_SIZE_PX,
  ];

  let mut window: PistonWindow = WindowSettings::new("Pusherman 🤜📦", window_size)
    .exit_on_esc(true)
    .build()
    .unwrap();
  while let Some(e) = window.next() {
    match e {
      Event::Input(Input::Button(args), _) => {
        if args.state == ButtonState::Press {
          match args.button {
            Button::Keyboard(Key::Up) => state.step(Dir::Up),
            Button::Keyboard(Key::Down) => state.step(Dir::Down),
            Button::Keyboard(Key::Left) => state.step(Dir::Left),
            Button::Keyboard(Key::Right) => state.step(Dir::Right),
            _ => {}
          }
        }
      }
      _ => {}
    }

    window.draw_2d(&e, |c, g, _| {
      for (i, row) in state.map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
          let color = match cell {
            Cell::Empty => WHITE_RGB,
            Cell::Wall => BLACK_RGB,
            Cell::Player => RED_RGB,
          };

          rectangle(
            color,
            [
              j as f64 * CELL_SIZE_PX,
              i as f64 * CELL_SIZE_PX,
              CELL_SIZE_PX,
              CELL_SIZE_PX,
            ],
            c.transform,
            g,
          );
        }
      }
    });
  }
}
