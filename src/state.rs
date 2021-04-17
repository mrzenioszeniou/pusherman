pub struct State {
  pub map: Vec<Vec<Cell>>,
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
          Cell::Player,
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
    }
  }
}

pub enum Cell {
  Wall,
  Empty,
  Player,
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
