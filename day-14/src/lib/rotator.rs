pub mod prelude {
  use ndarray::Array2;

  #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
  pub enum CardinalDirection {
    North,
    East,
    South,
    West,
  }

  pub fn find_positions(arr: &Array2<u8>, target: u8) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();

    for (index, &value) in arr.indexed_iter() {
      if value == target {
        positions.push(index);
      }
    }

    positions
  }
}

use memoize::memoize;
use ndarray::{s, Array2, Slice};

use prelude::*;


fn sort_positions(
  positions: &mut [(usize, usize)],
  direction: CardinalDirection,
) {
  match direction {
    CardinalDirection::North => positions.sort_by(|a, b| a.0.cmp(&b.0)),
    CardinalDirection::East => positions.sort_by(|a, b| b.1.cmp(&a.1)),
    CardinalDirection::South => positions.sort_by(|a, b| b.0.cmp(&a.0)),
    CardinalDirection::West => positions.sort_by(|a, b| a.1.cmp(&b.1)),
  }
}

fn get_slice(
  board: &Array2<u8>,
  (y, x): (usize, usize),
  direction: CardinalDirection,
) -> Vec<u8> {
  match direction {
    CardinalDirection::North => {
      if y == 0 {
        vec![]
      } else {
        board
          .slice(s![Slice::from(0..y), x])
          .iter()
          .rev()
          .cloned()
          .collect()
      }
    }
    CardinalDirection::East => {
      let slice = Slice::from(x..board.shape()[1]);

      board.slice(s![y, slice]).iter().cloned().collect()
    }
    CardinalDirection::South => {
      let slice = Slice::from(y..board.shape()[0]);
      board.slice(s![slice, x]).iter().cloned().collect()
    }
    CardinalDirection::West => {
      if x == 0 {
        vec![]
      } else {
        board
          .slice(s![y, Slice::from(0..x)])
          .iter()
          .rev()
          .cloned()
          .collect()
      }
    }
  }
}

fn get_offset(
  distance: usize,
  (y, x): (usize, usize),
  direction: CardinalDirection,
) -> Option<(usize, usize)> {
  match direction {
    CardinalDirection::North => Some((y - distance, x)),
    CardinalDirection::East => Some((y, x + distance)),
    CardinalDirection::South => Some((y + distance, x)),
    CardinalDirection::West => Some((y, x - distance)),
  }
}

fn move_rock(
  (y, x): (usize, usize),
  (new_y, new_x): (usize, usize),
  board: &mut Array2<u8>,
) {
  board[(y, x)] = b'.';
  board[(new_y, new_x)] = b'O';
}

#[memoize]
pub fn rotate_board(
  board: Array2<u8>,
  direction: CardinalDirection,
) -> Option<Array2<u8>> {
  let mut new_board = board.clone();
  let mut rocks = find_positions(&board, b'O');
  sort_positions(&mut rocks, direction);
  for (y, x) in rocks.iter() {
    // eprintln!("moving (y{y}, x{x})");
    let slice = get_slice(&new_board, (*y, *x), direction);
    // eprintln!(
    //   "slice {:?}",
    //   slice.iter().map(|c| *c as char).collect::<String>()
    // );
    let pos = slice
      .iter()
      .position(|c| c != &b'.')
      .unwrap_or(if !slice.is_empty() { slice.len() } else { 0 });
    // dbg!(pos);
    if pos > 0 {
      let Some(new_pos) = get_offset(pos, (*y, *x), direction) else {
        return None;
      };
      move_rock((*y, *x), new_pos, &mut new_board);
    }
  }

  Some(new_board)
}
