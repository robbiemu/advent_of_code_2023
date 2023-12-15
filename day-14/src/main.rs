use indexmap::IndexMap;
use ndarray::{Array2, Axis};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use lib::rotator::prelude::*;
use lib::rotator::rotate_board;
mod lib {
  pub mod rotator;
}


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

#[cfg(feature = "part2")]
const STEPS: usize = 4_000_000_000;

type ProblemDefinition = Array2<u8>;
type Consequent = usize;


fn main() -> Result<(), String> {
  let data = extract()?;
  let result = transform(data);

  load(result)
}

#[mry::mry]
fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}

fn extract() -> Result<ProblemDefinition, String> {
  let input = src_provider()?;
  let lines: Vec<&str> = input.lines().collect();
  let data: Vec<u8> = lines
    .iter()
    .flat_map(|line| line.as_bytes().iter().cloned())
    .collect();

  let rows = lines.len();
  let cols = data.len() / rows;

  Array2::from_shape_vec((rows, cols), data)
    .map_err(|e| format!("[extract] error with data\n{e}\n{:?}", lines))
}

fn score(data: ProblemDefinition) -> usize {
  let rocks = find_positions(&data, b'O');
  let len = data.dim().0;

  rocks.iter().fold(0, |acc, (y, _)| acc + len - y)
}

fn hash_array(array: &Array2<u8>) -> u64 {
  let mut hasher = DefaultHasher::new();
  for elem in array.iter() {
    elem.hash(&mut hasher);
  }
  hasher.finish()
}

fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
  let cycle = vec![
    CardinalDirection::North,
    CardinalDirection::West,
    CardinalDirection::South,
    CardinalDirection::East,
  ];
  #[allow(unused_assignments)]
  let mut break_point: usize = 1;
  #[cfg(feature = "part2")]
  {
    break_point = STEPS;
  }

  let mut board = data.clone();
  let mut iterations = 0;
  let mut memo: IndexMap<(CardinalDirection, u64), Array2<u8>> =
    IndexMap::new();
  loop {
    for direction in &cycle {
      // consider memoization
      let h = hash_array(&board);
      let key = (*direction, h);
      if memo.contains_key(&key) {
        let k = memo.keys().position(|&k| k == key).unwrap();
        let loop_len = iterations - k;

        let long_pos = (STEPS - iterations) % loop_len;
        let final_entry = memo.get_index(k + long_pos - 1).unwrap();
        board = final_entry.1.to_owned();

        break_point = iterations;
        break;
      }
      // else solve manually
      let o = rotate_board(board.clone(), *direction);
      match o {
        Some(b) => {
          memo.insert(key, b.clone());

          board = b
        }
        None => {
          return Err(
            "[transform] no solution found after rotation!".to_string(),
          );
        }
      }
      iterations += 1;
      if iterations >= break_point {
        break;
      }
    }
    if iterations >= break_point {
      break;
    }
  }

  dbg!(iterations);

  board.axis_iter(Axis(0)).for_each(|row| {
    let chars: String = row.iter().map(|&r| char::from(r)).collect();
    eprintln!("{}", chars);
  });

  Ok(score(board))
}

fn load(result: Result<Consequent, String>) -> Result<(), String> {
  match result {
    Ok(score) => println!("Score: {score}"),
    Err(e) => eprintln!("error: {e}"),
  }

  Ok(())
}


#[cfg(test)]
mod tests {
  // use super::*;

  // MARK extract

  // MARK transform

  // MARK load
}
