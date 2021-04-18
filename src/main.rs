extern crate piston_window;

mod cell;
mod levels;
mod state;

use piston_window::*;

use crate::cell::Cell;
use crate::state::Dir;

const CELL_SIZE_PX: f64 = 24_f64;
const BLACK_RGB: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const RED_RGB: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const WHITE_RGB: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const TARGET_BORDER_PX: f64 = 1_f64;

fn main() {
  let mut state = levels::level2();

  let map_size = state.size();
  let window_size = [
    map_size.0 as f64 * CELL_SIZE_PX,
    map_size.1 as f64 * CELL_SIZE_PX,
  ];

  let mut window: PistonWindow = WindowSettings::new("Pusherman 🤜📦", window_size)
    .resizable(false)
    .exit_on_esc(true)
    .build()
    .unwrap();

  // let mut font = window
  //   .load_font(PathBuf::from("src/res/UnicaOne-Regular.ttf"))
  //   .expect("INTERNAL ERROR: Could not load font");

  let target_rect: Rectangle = Rectangle::new_border(RED_RGB, TARGET_BORDER_PX);

  while let Some(e) = window.next() {
    if !state.victory() {
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

      if state.victory() {
        println!("Victory!!");
      }
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

      target_rect.draw(
        [
          state.target.1 as f64 * CELL_SIZE_PX + TARGET_BORDER_PX,
          state.target.0 as f64 * CELL_SIZE_PX + TARGET_BORDER_PX,
          CELL_SIZE_PX - 2.0 * TARGET_BORDER_PX,
          CELL_SIZE_PX - 2.0 * TARGET_BORDER_PX,
        ],
        &DrawState::default(),
        c.transform,
        g,
      );

      // if state.victory() {
      //   let transform = c
      //     .transform
      //     .trans(window_size[0] / 2.0, window_size[1] / 2.0);

      //   Text::new_color(WHITE_RGB, 16)
      //     .draw(
      //       "Noice 👌 Press N for next level",
      //       &mut font,
      //       &DrawState::default(),
      //       transform,
      //       g,
      //     )
      //     .expect("Could not draw text");

      //   font.factory.encoder.flush(device);
      // }
    });
  }
}
