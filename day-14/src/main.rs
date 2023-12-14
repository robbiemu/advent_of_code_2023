use ndarray::Array2;

use lib::rotator::prelude::*;
use lib::rotator::rotate_board;
use ndarray::Axis;
mod lib {
  pub mod rotator;
}


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

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

fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
  let board = rotate_board(data, CardinalDirection::North);

  match board {
    Some(b) => {
      b.axis_iter(Axis(0)).for_each(|row| {
        let chars: String = row.iter().map(|&r| char::from(r)).collect();
        eprintln!("{}", chars);
      });
      Ok(score(b))
    }
    None => Err("[transform] no solution found after rotation!".to_string()),
  }
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
