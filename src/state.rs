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

  pub fn step(&mut self, dir: Dir) {
    let (i, j) = self.pos();

    match dir {
      Dir::Up => {
        if i > 0 && j < self.map[i - 1].len() && !self.map[i - 1][j].is_wall() {
          self.map[i][j] = Cell::Empty;
          self.map[i - 1][j] = Cell::Player;
        }
      }
      Dir::Down => {
        if i < self.map.len() - 1 && j < self.map[i + 1].len() && !self.map[i + 1][j].is_wall() {
          self.map[i][j] = Cell::Empty;
          self.map[i + 1][j] = Cell::Player;
        }
      }
      Dir::Left => {
        if j > 0 && !self.map[i][j - 1].is_wall() {
          self.map[i][j] = Cell::Empty;
          self.map[i][j - 1] = Cell::Player;
        }
      }
      Dir::Right => {
        if j < self.map[i].len() - 1 && !self.map[i][j + 1].is_wall() {
          self.map[i][j] = Cell::Empty;
          self.map[i][j + 1] = Cell::Player;
        }
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

impl Cell {
  pub fn is_wall(&self) -> bool {
    match self {
      Self::Wall => true,
      _ => false,
    }
  }
}

pub enum Dir {
  Up,
  Down,
  Left,
  Right,
}
