use ndarray::Array2;
use std::{collections::HashMap, str::FromStr};


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

type Consequent = usize;

#[repr(u8)]
enum Splitter {
  Horizontal = b'-',
  Vertical = b'|',
}

#[repr(u8)]
enum Mirror {
  Clockwise = b'\\',
  CounterClockwise = b'/',
}

enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Default)]
struct VisitRecord {
  right: bool,
  down: bool,
  left: bool,
  up: bool,
}

impl VisitRecord {
  fn has_visited(&self, direction: &Direction) -> bool {
    match direction {
      Direction::Right => self.right,
      Direction::Down => self.down,
      Direction::Left => self.left,
      Direction::Up => self.up,
    }
  }

  fn visit(&mut self, direction: &Direction) {
    match direction {
      Direction::Right => self.right = true,
      Direction::Down => self.down = true,
      Direction::Left => self.left = true,
      Direction::Up => self.up = true,
    }
  }
}

struct BeamMap {
  map: Array2<u8>,
}

impl BeamMap {
  fn beam_dfs(
    &self,
    y_in: i16,
    x_in: i16,
    visited: &mut HashMap<(i16, i16), VisitRecord>,
    heading: Direction,
  ) {
    // dismiss if out of bounds
    if x_in < 0 || y_in < 0 {
      return;
    }
    let x = x_in as usize;
    let y = y_in as usize;
    let shape = self.map.shape();
    if x >= shape[0] || y >= shape[1] {
      return;
    }
    // visit
    let entry = visited.entry((y_in, x_in)).or_default();
    if entry.has_visited(&heading) {
      /* if we don't have a new entry has_visited will be true for at least one direction. if it is true for ours, we've been down this path before */
      return;
    }
    entry.visit(&heading);

    // progress
    match self.map[[y, x]] {
      b'.' => match heading {
        Direction::Right => self.beam_dfs(y_in, x_in + 1, visited, heading),
        Direction::Down => self.beam_dfs(y_in + 1, x_in, visited, heading),
        Direction::Left => self.beam_dfs(y_in, x_in - 1, visited, heading),
        Direction::Up => self.beam_dfs(y_in - 1, x_in, visited, heading),
      },
      c if c == Mirror::CounterClockwise as u8 => {
        let (new_y, new_x, new_direction) = match heading {
          Direction::Right => (y_in - 1, x_in, Direction::Up),
          Direction::Up => (y_in, x_in + 1, Direction::Right),
          Direction::Left => (y_in + 1, x_in, Direction::Down),
          Direction::Down => (y_in, x_in - 1, Direction::Left),
        };

        self.beam_dfs(new_y, new_x, visited, new_direction);
      }
      c if c == Mirror::Clockwise as u8 => {
        let (new_y, new_x, new_direction) = match heading {
          Direction::Right => (y_in + 1, x_in, Direction::Down),
          Direction::Up => (y_in, x_in - 1, Direction::Left),
          Direction::Left => (y_in - 1, x_in, Direction::Up),
          Direction::Down => (y_in, x_in + 1, Direction::Right),
        };

        self.beam_dfs(new_y, new_x, visited, new_direction);
      }
      c if c == Splitter::Horizontal as u8 => match heading {
        Direction::Right => self.beam_dfs(y_in, x_in + 1, visited, heading),
        Direction::Left => self.beam_dfs(y_in, x_in - 1, visited, heading),
        Direction::Up | Direction::Down => {
          self.beam_dfs(y_in, x_in + 1, visited, Direction::Right);
          self.beam_dfs(y_in, x_in - 1, visited, Direction::Left);
        }
      },
      c if c == Splitter::Vertical as u8 => match heading {
        Direction::Down => self.beam_dfs(y_in + 1, x_in, visited, heading),
        Direction::Up => self.beam_dfs(y_in - 1, x_in, visited, heading),
        Direction::Right | Direction::Left => {
          self.beam_dfs(y_in + 1, x_in, visited, Direction::Down);
          self.beam_dfs(y_in - 1, x_in, visited, Direction::Up);
        }
      },
      _ => {}
    }
  }

  pub fn count_visited_tiles(
    &self,
    y_u: usize,
    x_u: usize,
    direction: Direction,
  ) -> usize {
    let mut visited = HashMap::new();
    self.beam_dfs(y_u as i16, x_u as i16, &mut visited, direction);

    visited.len()
  }
}

impl FromStr for BeamMap {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let input: Vec<&str> = s.lines().collect();
    let rows = input.len();
    let cols = input[0].len();
    let byte_map: Vec<u8> = input
      .iter()
      .flat_map(|line| line.as_bytes().to_vec())
      .collect();

    let map = Array2::from_shape_vec((rows, cols), byte_map).map_err(|e| {
      format!("Failed to create Array2 from input. error:\n{e}")
    })?;

    Ok(BeamMap { map })
  }
}


#[mry::mry]
fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}

fn extract() -> Result<BeamMap, String> {
  src_provider()?.parse::<BeamMap>()
}

fn transform(data: BeamMap) -> Result<Consequent, String> {
  #[cfg(not(feature = "part2"))]
  {
    Ok(data.count_visited_tiles(0, 0, Direction::Right))
  }
  #[cfg(feature = "part2")]
  {
    let rows = data.map.shape()[0];
    let cols = data.map.shape()[1];

    let mut max_count = 0;
    // Iterate over top and bottom edge
    for x in 0..cols {
      let count = data.count_visited_tiles(0, x, Direction::Down);
      max_count = max_count.max(count);
      let count = data.count_visited_tiles(rows - 1, x, Direction::Up);
      max_count = max_count.max(count);
    }

    // Iterate over left and right edge
    for y in 0..rows {
      let count = data.count_visited_tiles(y, 0, Direction::Right);
      max_count = max_count.max(count);
      let count = data.count_visited_tiles(y, cols - 1, Direction::Left);
      max_count = max_count.max(count);
    }

    Ok(max_count)
  }
}

fn load(result: Result<Consequent, String>) -> Result<(), String> {
  match result {
    Ok(count) => println!("count: {count}"),
    Err(e) => eprintln!("{e}"),
  }

  Ok(())
}

fn main() -> Result<(), String> {
  let data = extract()?;
  let result = transform(data);

  load(result)
}


#[cfg(test)]
mod tests {
  use super::*;

  // MARK extract

  // MARK transform
  #[test]
  fn test_empty_space_right() {
    let mut visited = HashMap::new();
    let map = BeamMap { map: Array2::from_elem((5, 5), b'.') };
    map.beam_dfs(0, 0, &mut visited, Direction::Right);

    assert!(visited.contains_key(&(0, 4)));
    assert_eq!(visited.len(), 5);
  }
  #[test]
  fn test_empty_space_down() {
    let mut visited = HashMap::new();
    let map = BeamMap { map: Array2::from_elem((5, 5), b'.') };
    map.beam_dfs(0, 0, &mut visited, Direction::Down);

    assert!(visited.contains_key(&(4, 0)));
    assert_eq!(visited.len(), 5);
  }
  #[test]
  fn test_empty_space_left() {
    let mut visited = HashMap::new();
    let map = BeamMap { map: Array2::from_elem((5, 5), b'.') };
    map.beam_dfs(0, 4, &mut visited, Direction::Left);

    assert!(visited.contains_key(&(0, 0)));
    assert_eq!(visited.len(), 5);
  }
  #[test]
  fn test_empty_space_up() {
    let mut visited = HashMap::new();
    let map = BeamMap { map: Array2::from_elem((5, 5), b'.') };
    map.beam_dfs(4, 0, &mut visited, Direction::Up);

    assert!(visited.contains_key(&(0, 0)));
    assert_eq!(visited.len(), 5);
  }

  #[test]
  fn test_clockwise_mirror_right() {
    let mut visited = HashMap::new();
    let mut map = Array2::from_elem((5, 5), b'.');
    map[[2, 2]] = Mirror::Clockwise as u8;
    let beam_map = BeamMap { map };
    beam_map.beam_dfs(2, 0, &mut visited, Direction::Right);

    assert!(visited.contains_key(&(4, 2)));
    assert_eq!(visited.len(), 5);
  }
  #[test]
  fn test_clockwise_mirror_down() {
    let mut visited = HashMap::new();
    let mut map = Array2::from_elem((5, 5), b'.');
    map[[2, 2]] = Mirror::Clockwise as u8;
    let beam_map = BeamMap { map };
    beam_map.beam_dfs(0, 2, &mut visited, Direction::Down);

    assert!(visited.contains_key(&(0, 2)));
    assert_eq!(visited.len(), 5);
  }
  #[test]
  fn test_clockwise_mirror_left() {
    let mut visited = HashMap::new();
    let mut map = Array2::from_elem((5, 5), b'.');
    map[[2, 2]] = Mirror::Clockwise as u8;
    let beam_map = BeamMap { map };
    beam_map.beam_dfs(2, 4, &mut visited, Direction::Left);

    assert!(visited.contains_key(&(0, 2)));
    assert_eq!(visited.len(), 5);
  }
  #[test]
  fn test_clockwise_mirror_up() {
    let mut visited = HashMap::new();
    let mut map = Array2::from_elem((5, 5), b'.');
    map[[2, 2]] = Mirror::Clockwise as u8;
    let beam_map = BeamMap { map };
    beam_map.beam_dfs(4, 2, &mut visited, Direction::Up);

    assert!(visited.contains_key(&(4, 2)));
    assert_eq!(visited.len(), 5);
  }

  #[test]
  fn test_counterclockwise_mirror_right() {
    let mut visited = HashMap::new();
    let mut map = Array2::from_elem((5, 5), b'.');
    map[[2, 2]] = Mirror::CounterClockwise as u8;
    let beam_map = BeamMap { map };
    beam_map.beam_dfs(2, 0, &mut visited, Direction::Right);

    assert!(visited.contains_key(&(2, 0)));
    assert_eq!(visited.len(), 5);
  }
  #[test]
  fn test_counterclockwise_mirror_down() {
    let mut visited = HashMap::new();
    let mut map = Array2::from_elem((5, 5), b'.');
    map[[2, 2]] = Mirror::CounterClockwise as u8;
    let beam_map = BeamMap { map };
    beam_map.beam_dfs(0, 2, &mut visited, Direction::Down);

    assert!(visited.contains_key(&(2, 0)));
    assert_eq!(visited.len(), 5);
  }
  #[test]
  fn test_counterclockwise_mirror_left() {
    let mut visited = HashMap::new();
    let mut map = Array2::from_elem((5, 5), b'.');
    map[[2, 2]] = Mirror::CounterClockwise as u8;
    let beam_map = BeamMap { map };
    beam_map.beam_dfs(2, 4, &mut visited, Direction::Left);

    assert!(visited.contains_key(&(2, 4)));
    assert_eq!(visited.len(), 5);
  }
  #[test]
  fn test_counterclockwise_mirror_up() {
    let mut visited = HashMap::new();
    let mut map = Array2::from_elem((5, 5), b'.');
    map[[2, 2]] = Mirror::CounterClockwise as u8;
    let beam_map = BeamMap { map };
    beam_map.beam_dfs(4, 2, &mut visited, Direction::Up);

    assert!(visited.contains_key(&(2, 4)));
    assert_eq!(visited.len(), 5);
  }

  #[test]
  fn test_horizontal_splitter_right() {
    let mut visited = HashMap::new();
    let mut map = Array2::from_elem((5, 5), b'.');
    map[[2, 2]] = Splitter::Horizontal as u8;

    let beam_map = BeamMap { map };
    beam_map.beam_dfs(2, 0, &mut visited, Direction::Right);

    assert!(visited.contains_key(&(2, 4)));
    assert_eq!(visited.len(), 5);
  }
  #[test]
  fn test_horizontal_splitter_down() {
    let mut visited = HashMap::new();
    let mut map = Array2::from_elem((5, 5), b'.');
    map[[2, 2]] = Splitter::Horizontal as u8;

    let beam_map = BeamMap { map };
    beam_map.beam_dfs(0, 2, &mut visited, Direction::Down);

    assert!(visited.contains_key(&(2, 4)));
    assert!(visited.contains_key(&(2, 0)));
    assert_eq!(visited.len(), 7);
  }
  #[test]
  fn test_horizontal_splitter_left() {
    let mut visited = HashMap::new();
    let mut map = Array2::from_elem((5, 5), b'.');
    map[[2, 2]] = Splitter::Horizontal as u8;

    let beam_map = BeamMap { map };
    beam_map.beam_dfs(2, 4, &mut visited, Direction::Left);

    assert!(visited.contains_key(&(2, 0)));
    assert_eq!(visited.len(), 5);
  }
  #[test]
  fn test_horizontal_splitter_up() {
    let mut visited = HashMap::new();
    let mut map = Array2::from_elem((5, 5), b'.');
    map[[2, 2]] = Splitter::Horizontal as u8;

    let beam_map = BeamMap { map };
    beam_map.beam_dfs(4, 2, &mut visited, Direction::Up);

    assert!(visited.contains_key(&(2, 4)));
    assert!(visited.contains_key(&(2, 0)));
    assert_eq!(visited.len(), 7);
  }

  #[test]
  fn test_vertical_splitter_right() {
    let mut visited = HashMap::new();
    let mut map = Array2::from_elem((5, 5), b'.');
    map[[2, 2]] = Splitter::Vertical as u8;

    let beam_map = BeamMap { map };
    beam_map.beam_dfs(2, 0, &mut visited, Direction::Right);

    assert!(visited.contains_key(&(0, 2)));
    assert!(visited.contains_key(&(4, 2)));
    assert_eq!(visited.len(), 7);
  }
  #[test]
  fn test_vertical_splitter_down() {
    let mut visited = HashMap::new();
    let mut map = Array2::from_elem((5, 5), b'.');
    map[[2, 2]] = Splitter::Vertical as u8;

    let beam_map = BeamMap { map };
    beam_map.beam_dfs(0, 2, &mut visited, Direction::Down);

    assert!(visited.contains_key(&(4, 2)));
    assert_eq!(visited.len(), 5);
  }
  #[test]
  fn test_vertical_splitter_left() {
    let mut visited = HashMap::new();
    let mut map = Array2::from_elem((5, 5), b'.');
    map[[2, 2]] = Splitter::Vertical as u8;

    let beam_map = BeamMap { map };
    beam_map.beam_dfs(2, 4, &mut visited, Direction::Left);

    assert!(visited.contains_key(&(0, 2)));
    assert!(visited.contains_key(&(4, 2)));
    assert_eq!(visited.len(), 7);
  }
  #[test]
  fn test_vertical_splitter_up() {
    let mut visited = HashMap::new();
    let mut map = Array2::from_elem((5, 5), b'.');
    map[[2, 2]] = Splitter::Vertical as u8;

    let beam_map = BeamMap { map };
    beam_map.beam_dfs(4, 2, &mut visited, Direction::Up);

    assert!(visited.contains_key(&(0, 2)));
    assert_eq!(visited.len(), 5);
  }

  // MARK load
}
