use std::str::FromStr;

use crate::state::State;

pub fn level1() -> State {
  let state_str = "
XXXXXXXX
X      X
X S    X
X      X
X      X
X      X
X    F X
X      X
XXXXXXXX";

  State::from_str(state_str).expect("INTERNAL ERROR: Could not parse level 1")
}

pub fn level2() -> State {
  let state_str = "
XXXXXXXX
X      X
X    S X
X      X
XXXXXXXX
X      X
X F    X
X      X
XXXXXXXX";

  State::from_str(state_str).expect("INTERNAL ERROR: Could not parse level 1")
}
