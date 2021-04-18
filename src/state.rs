use crate::cell::Cell;
use std::convert::TryFrom;
use std::str::FromStr;

pub struct State {
  pub map: Vec<Vec<Cell>>,
  pub target: (usize, usize),
}

impl FromStr for State {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut map = vec![];
    let mut target = None;

    for (i, line_str) in s
      .split("\n")
      .map(|l| l.trim())
      .filter(|l| !l.is_empty())
      .enumerate()
    {
      let mut line = vec![];
      for (j, c) in line_str.chars().enumerate() {
        if c == 'F' || c == 'f' {
          if target.is_none() {
            target = Some((i, j))
          } else {
            panic!("Multiple finish points found");
          }
        }

        line.push(Cell::try_from(c)?);
      }
      map.push(line);
    }

    if map.is_empty() {
      return Err(format!("Encountered empty map"));
    }

    if map.iter().any(|l| l.len() != map[0].len()) {
      return Err(format!("Uneven map width encountered"));
    }

    Ok(Self {
      map,
      target: target.expect("No finish point found"),
    })
  }
}

impl State {
  pub fn size(&self) -> (usize, usize) {
    (
      self.map.iter().map(|r| r.len()).max().unwrap_or(0),
      self.map.len(),
    )
  }

  fn pos(&self) -> (usize, usize) {
    for (i, row) in self.map.iter().enumerate() {
      for (j, cell) in row.iter().enumerate() {
        if let Cell::Player = cell {
          return (i, j);
        }
      }
    }

    panic!("Couldn't find player in the map");
  }

  pub fn victory(&self) -> bool {
    if let Cell::Player = self.map[self.target.0][self.target.1] {
      true
    } else {
      false
    }
  }

  fn in_bounds(&self, i: isize, j: isize) -> bool {
    let (i, j) = if i >= 0 || j >= 0 {
      (i as usize, j as usize)
    } else {
      return false;
    };

    i < self.map.len() && j < self.map[i].len()
  }

  fn is_empty(&self, i: isize, j: isize) -> bool {
    if !self.in_bounds(i, j) {
      return false;
    }

    match self.map[i as usize][j as usize] {
      Cell::Empty => true,
      _ => false,
    }
  }

  fn is_wall(&self, i: isize, j: isize) -> bool {
    if !self.in_bounds(i, j) {
      return false;
    }

    match self.map[i as usize][j as usize] {
      Cell::Wall => true,
      _ => false,
    }
  }

  pub fn step(&mut self, dir: Dir) {
    let (i, j) = self.pos();

    let (i_new, j_new) = dir.shift(i as isize, j as isize);

    if self.is_empty(i_new, j_new) {
      // println!("Moving from ({},{}) to ({},{})", i, j, i_new, j_new);
      self.map[i][j] = Cell::Empty;
      self.map[i_new as usize][j_new as usize] = Cell::Player;
    } else if self.is_wall(i_new, j_new) {
      let (i_nnew, j_nnew) = dir.shift(i_new, j_new);

      if self.is_empty(i_nnew, j_nnew) {
        self.map[i][j] = Cell::Empty;
        self.map[i_new as usize][j_new as usize] = Cell::Player;
        self.map[i_nnew as usize][j_nnew as usize] = Cell::Wall;
      }
    }
  }
}

impl Default for State {
  fn default() -> Self {
    Self {
      map: vec![
        vec![
          Cell::Wall,
          Cell::Wall,
          Cell::Wall,
          Cell::Wall,
          Cell::Wall,
          Cell::Wall,
          Cell::Wall,
          Cell::Wall,
          Cell::Wall,
          Cell::Wall,
        ],
        vec![
          Cell::Wall,
          Cell::Empty,
          Cell::Empty,
          Cell::Wall,
          Cell::Empty,
          Cell::Empty,
          Cell::Empty,
          Cell::Empty,
          Cell::Empty,
          Cell::Wall,
        ],
        vec![
          Cell::Wall,
          Cell::Empty,
          Cell::Empty,
          Cell::Wall,
          Cell::Empty,
          Cell::Empty,
          Cell::Empty,
          Cell::Empty,
          Cell::Empty,
          Cell::Wall,
        ],
        vec![
          Cell::Wall,
          Cell::Empty,
          Cell::Empty,
          Cell::Empty,
          Cell::Wall,
          Cell::Wall,
          Cell::Wall,
          Cell::Empty,
          Cell::Empty,
          Cell::Wall,
        ],
        vec![
          Cell::Wall,
          Cell::Empty,
          Cell::Empty,
          Cell::Empty,
          Cell::Wall,
          Cell::Player,
          Cell::Wall,
          Cell::Empty,
          Cell::Empty,
          Cell::Wall,
        ],
        vec![
          Cell::Wall,
          Cell::Empty,
          Cell::Empty,
          Cell::Empty,
          Cell::Wall,
          Cell::Empty,
          Cell::Wall,
          Cell::Empty,
          Cell::Empty,
          Cell::Wall,
        ],
        vec![
          Cell::Wall,
          Cell::Empty,
          Cell::Empty,
          Cell::Empty,
          Cell::Empty,
          Cell::Empty,
          Cell::Empty,
          Cell::Empty,
          Cell::Empty,
          Cell::Wall,
        ],
        vec![
          Cell::Wall,
          Cell::Empty,
          Cell::Empty,
          Cell::Empty,
          Cell::Empty,
          Cell::Empty,
          Cell::Wall,
          Cell::Empty,
          Cell::Empty,
          Cell::Wall,
        ],
        vec![
          Cell::Wall,
          Cell::Empty,
          Cell::Empty,
          Cell::Empty,
          Cell::Empty,
          Cell::Empty,
          Cell::Wall,
          Cell::Empty,
          Cell::Empty,
          Cell::Wall,
        ],
        vec![
          Cell::Wall,
          Cell::Wall,
          Cell::Wall,
          Cell::Wall,
          Cell::Wall,
          Cell::Wall,
          Cell::Wall,
          Cell::Wall,
          Cell::Wall,
          Cell::Wall,
        ],
      ],
      target: (8, 1),
    }
  }
}

pub enum Dir {
  Up,
  Down,
  Left,
  Right,
}

impl Dir {
  pub fn shift(&self, i: isize, j: isize) -> (isize, isize) {
    match self {
      Self::Up => (i - 1, j),
      Self::Down => (i + 1, j),
      Self::Right => (i, j + 1),
      Self::Left => (i, j - 1),
    }
  }
}
