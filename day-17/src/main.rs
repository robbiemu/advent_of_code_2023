use std::collections::{BinaryHeap, HashSet};


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

#[cfg(not(feature = "part2"))]
const LEGAL_CONSECUTIVE_MOVES: LegalMoves = LegalMoves { min: 1, max: 3 };
#[cfg(feature = "part2")]
const LEGAL_CONSECUTIVE_MOVES: LegalMoves = LegalMoves { min: 4, max: 7 };

pub struct LegalMoves {
  pub min: usize,
  pub max: usize,
}

#[derive(Debug)]
struct Path {
  y: usize,
  x: usize,
  dir: Direction,
  heat_loss: u32,
}

impl Ord for Path {
  fn cmp(&self, other: &Path) -> std::cmp::Ordering {
    self.heat_loss.cmp(&other.heat_loss)
  }
}

impl PartialOrd for Path {
  fn partial_cmp(&self, other: &Path) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for Path {
  fn eq(&self, other: &Path) -> bool {
    self.heat_loss == other.heat_loss
  }
}

impl Eq for Path {}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
  #[default]
  #[allow(clippy::enum_variant_names)]
  NoDirection,
  Right,
  Down,
  Left,
  Up,
}

impl Direction {
  pub fn get_directed_neighbors(
    (y, x): (usize, usize),
    rows: usize,
    cols: usize,
  ) -> Vec<((usize, usize), Direction)> {
    let mut neighbors = Vec::new();

    for i in LEGAL_CONSECUTIVE_MOVES.min..=LEGAL_CONSECUTIVE_MOVES.max {
      if x >= i {
        neighbors.push(((y, x - i), Direction::Left));
      }
      if y + i < rows {
        neighbors.push(((y + i, x), Direction::Down));
      }
      if x + i < cols {
        neighbors.push(((y, x + i), Direction::Right));
      }
      if y >= i {
        neighbors.push(((y - i, x), Direction::Up));
      }
    }

    neighbors
  }

  pub fn invert(direction: &Direction) -> Direction {
    match direction {
      Direction::NoDirection => Direction::NoDirection,
      Direction::Right => Direction::Left,
      Direction::Down => Direction::Up,
      Direction::Left => Direction::Right,
      Direction::Up => Direction::Down,
    }
  }
}

struct ProblemDefinition {
  start: (usize, usize),
  end: (usize, usize),
  rows: usize,
  cols: usize,
  grid: Vec<Vec<u32>>,
}
type Consequent = u32;


fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}

fn extract() -> Result<ProblemDefinition, String> {
  let s = src_provider()?;
  if s.chars().any(|c| !(c.is_ascii_digit() || c == '\n')) {
    return Err("non digit in map!".to_string());
  }

  let grid: Vec<Vec<u32>> = s
    .lines()
    .map(|l| {
      l.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();
  let rows = grid.len();
  let cols = grid.get(0).ok_or("no data in input")?.len();
  let start = (0, 0);
  let end = (rows - 1, cols - 1);

  Ok(ProblemDefinition { start, end, grid, rows, cols })
}

fn get_legal_paths(
  y: usize,
  x: usize,
  data: &ProblemDefinition,
  previous_direction: Direction,
  current_heat_loss: u32,
) -> Vec<Path> {
  Direction::get_directed_neighbors((y, x), data.rows, data.cols)
    .into_iter()
    .filter_map(|((next_y, next_x), dir)| {
      if previous_direction == dir
        || previous_direction == Direction::invert(&dir)
      {
        None
      } else {
        let mut current_coord = (y, x);
        let inital_loss = data.grid[current_coord.0][current_coord.1];
        let mut accumulated_loss = 0;

        while current_coord != (next_y, next_x) {
          accumulated_loss += data.grid[current_coord.0][current_coord.1];

          current_coord = match dir {
            Direction::Left => (current_coord.0, current_coord.1 - 1),
            Direction::Right => (current_coord.0, current_coord.1 + 1),
            Direction::Up => (current_coord.0 - 1, current_coord.1),
            Direction::Down => (current_coord.0 + 1, current_coord.1),
            Direction::NoDirection => unreachable!(), // Shouldn't happen
          };
        }
        accumulated_loss -= inital_loss;
        let heat_loss = current_heat_loss + accumulated_loss;
        Some(Path { y: next_y, x: next_x, dir, heat_loss })
      }
    })
    .collect::<Vec<_>>()
}

fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
  let mut paths: BinaryHeap<Path> = [Direction::Right, Direction::Down]
    .into_iter()
    .map(|dir| Path { y: data.start.0, x: data.start.1, dir, heat_loss: 0 })
    .collect();

  let mut visited: HashSet<usize> = HashSet::new();

  loop {
    if let Some(Path { x, y, dir, heat_loss }) = paths.pop() {
      dbg!(((y, x), dir, heat_loss), &paths);
      if !visited.contains(&(2 * (y * data.cols + x) + (dir as usize % 2))) {
        if (y, x) == data.end {
          return Ok(heat_loss);
        } else {
          let previous_direction = dir;
          visited.insert(2 * (y * data.cols + x) + (dir as usize % 2));
          paths.extend(get_legal_paths(
            y,
            x,
            &data,
            previous_direction,
            heat_loss,
          ));
        }
      }
    } else {
      return Err("No path found!".to_string());
    }
  }
}

fn load(result: Result<Consequent, String>) -> Result<(), String> {
  match result {
    Ok(heat_loss) => println!("heat_loss {heat_loss}"),
    Err(e) => eprintln!("{:?}", e),
  }

  Ok(())
}

fn main() -> Result<(), String> {
  let data = extract()?;
  let result = transform(data);

  load(result)
}
