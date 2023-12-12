use itertools::Itertools;
use std::fmt::Debug;
#[cfg(not(feature = "part2"))]
use std::iter::repeat;


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

#[cfg(feature = "part2")]
const HUBBLE_CONSTANT: usize = 999_999;

enum Rotation {
  Clockwise,
  #[allow(dead_code)]
  CounterClockwise,
}

#[derive(Clone)]
struct Coord {
  y: usize,
  x: usize,
}

impl Coord {
  fn manhattan_distance(&self, other: &Coord) -> usize {
    ((self.x as isize - other.x as isize).abs()
      + (self.y as isize - other.y as isize).abs()) as usize
  }
}

impl Debug for Coord {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("(y:{},x:{})", self.y, self.x))
  }
}

type ProblemDefinition = Vec<Vec<bool>>;
type Consequent = Vec<usize>;

fn main() -> Result<(), String> {
  let mut data = extract()?;
  let result = transform(&mut data);

  load(result)
}

#[mry::mry]
fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}

fn extract() -> Result<ProblemDefinition, String> {
  let mut grid: Vec<Vec<bool>> = vec![];
  for line in src_provider()?.lines() {
    let mut row: Vec<bool> = vec![];
    for c in line.chars() {
      match c {
        '.' => row.push(false),
        '#' => row.push(true),
        _ => {
          return Err(format!(
            "[extract] illegal character in input at '{line}'"
          ))
        } // Ignore other characters
      }
    }
    grid.push(row);
  }

  Ok(grid)
}

fn rotate_matrix(matrix: &mut Vec<Vec<bool>>, direction: Rotation) {
  if matrix.is_empty() || matrix[0].is_empty() {
    return;
  }

  let rows = matrix.len();
  let cols = matrix[0].len();

  // Handle non-square matrices differently.
  match direction {
    Rotation::Clockwise => {
      let mut rotated = vec![vec![false; rows]; cols];
      for (i, row) in matrix.iter().enumerate().take(rows) {
        for (j, &value) in row.iter().enumerate().take(cols) {
          rotated[j][rows - i - 1] = value;
        }
      }

      *matrix = rotated;
    }
    Rotation::CounterClockwise => {
      let mut rotated = vec![vec![false; rows]; cols];
      for (i, row) in matrix.iter().enumerate().take(rows) {
        for (j, &value) in row.iter().enumerate().take(cols) {
          rotated[j][rows - i - 1] = value;
        }
      }

      *matrix = rotated;
    }
  }
}

#[cfg(not(feature = "part2"))]
fn insert_rows(grid: &mut Vec<Vec<bool>>, mut indices: Vec<usize>) {
  // Sort the indices in descending order to safely insert new rows without affecting the positions of existing ones.
  indices.sort_unstable_by(|a, b| b.cmp(a));

  let cols = if !grid.is_empty() { grid[0].len() } else { 0 };
  let new_row: Vec<bool> = repeat(false).take(cols).collect();

  for index in indices {
    grid.insert(index, new_row.to_owned());
  }
}

fn get_expanding_rows(data: &mut ProblemDefinition) -> Vec<usize> {
  data
    .iter()
    .enumerate()
    .filter_map(|(y, row)| {
      if row.iter().all(|item| !item) {
        Some(y)
      } else {
        None
      }
    })
    .collect()
}

fn get_expanding_cols(data: &mut ProblemDefinition) -> Vec<usize> {
  rotate_matrix(data, Rotation::Clockwise);
  let cols = get_expanding_rows(data);
  rotate_matrix(data, Rotation::CounterClockwise);

  cols
}

#[cfg(not(feature = "part2"))]
fn expand_galaxy(data: &mut ProblemDefinition) {
  let rows = get_expanding_rows(data);
  insert_rows(data, rows);
  rotate_matrix(data, Rotation::Clockwise);
  let cols = get_expanding_cols(data);
  insert_rows(data, cols);
}

fn locate_stars(galaxy: &[Vec<bool>]) -> Vec<Coord> {
  galaxy
    .iter()
    .enumerate()
    .flat_map(|(y, row)| {
      row
        .iter()
        .enumerate()
        .filter_map(
          |(x, &value)| if value { Some(Coord { y, x }) } else { None },
        )
        .collect::<Vec<Coord>>()
    })
    .collect()
}

#[cfg(feature = "part2")]
fn count_items_between(indices: &[usize], left: usize, right: usize) -> usize {
  let (lower, higher) = if left < right {
    (left, right)
  } else {
    (right, left)
  };

  let mut count = 0;

  for &index in indices {
    if index > lower && index < higher {
      count += 1;
    }
  }


  count
}

fn transform(data: &mut ProblemDefinition) -> Result<Consequent, String> {
  #[cfg(not(feature = "part2"))]
  {
    expand_galaxy(data);
    dbg!(data
      .iter()
      .map(|r| r
        .iter()
        .map(|p| if *p { '#' } else { '.' })
        .collect::<String>())
      .collect::<Vec<_>>());
  }
  let stars = locate_stars(data);
  #[cfg(feature = "part2")]
  let (expanding_rows, expanding_cols) =
    (get_expanding_rows(data), get_expanding_cols(data));
  let distances = stars
    .into_iter()
    .combinations(2)
    .collect::<Vec<_>>()
    .iter()
    .map(|v| {
      #[allow(unused_mut)]
      let mut d = v[0].manhattan_distance(&v[1]);
      #[cfg(feature = "part2")]
      {
        let rows_between = count_items_between(&expanding_rows, v[0].y, v[1].y);
        let cols_between = count_items_between(&expanding_cols, v[0].x, v[1].x);
        d += rows_between * HUBBLE_CONSTANT + cols_between * HUBBLE_CONSTANT;
      }
      d
    })
    .collect();

  Ok(distances)
}

fn load(result: Result<Consequent, String>) -> Result<(), String> {
  match result {
    Ok(items) => {
      println!("{} steps", items.iter().sum::<usize>());
    }
    Err(e) => eprintln!("{e}"),
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
