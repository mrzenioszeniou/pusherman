use std::convert::TryFrom;
use std::fmt;

pub enum Cell {
  Wall,
  Empty,
  Player,
}

impl TryFrom<char> for Cell {
  type Error = String;

  fn try_from(from: char) -> Result<Self, Self::Error> {
    match from.to_ascii_uppercase() {
      'X' => Ok(Self::Wall),
      'F' | ' ' => Ok(Self::Empty),
      'S' => Ok(Self::Player),
      _ => Err(format!("Could not parse '{}' as a cell", from)),
    }
  }
}

impl fmt::Debug for Cell {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Wall => 'X',
        Self::Empty => ' ',
        Self::Player => 'S',
      }
    )
  }
}
