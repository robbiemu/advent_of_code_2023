use std::collections::{HashMap, HashSet, VecDeque};

#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
  x: usize,
  y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
  North,
  East,
  South,
  West,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Neighbors {
  north: Option<Coord>,
  east: Option<Coord>,
  south: Option<Coord>,
  west: Option<Coord>,
}

impl Neighbors {
  fn get_neighbors_connected_by_coord(
    board: &[Vec<char>],
    coord: &Coord,
  ) -> Neighbors {
    let mut neighbors =
      Neighbors { north: None, east: None, south: None, west: None };

    if coord.y > 0
      && (board[coord.y][coord.x] == '|'
        || board[coord.y][coord.x] == 'L'
        || board[coord.y][coord.x] == 'J')
    {
      neighbors.north = Some(Coord { y: coord.y - 1, x: coord.x });
    }
    if coord.x < board[0].len() - 1
      && (board[coord.y][coord.x] == '-'
        || board[coord.y][coord.x] == 'L'
        || board[coord.y][coord.x] == 'F')
    {
      neighbors.east = Some(Coord { y: coord.y, x: coord.x + 1 });
    }
    if coord.y < board.len() - 1
      && (board[coord.y][coord.x] == '|'
        || board[coord.y][coord.x] == '7'
        || board[coord.y][coord.x] == 'F')
    {
      neighbors.south = Some(Coord { y: coord.y + 1, x: coord.x });
    }
    if coord.x > 0
      && (board[coord.y][coord.x] == '-'
        || board[coord.y][coord.x] == 'J'
        || board[coord.y][coord.x] == '7')
    {
      neighbors.west = Some(Coord { y: coord.y, x: coord.x - 1 });
    }

    neighbors
  }

  fn iter_coords(&self) -> impl Iterator<Item = Coord> {
    let mut connected = Vec::new();
    if let Some(coord) = self.north {
      connected.push(coord);
    }
    if let Some(coord) = self.east {
      connected.push(coord);
    }
    if let Some(coord) = self.south {
      connected.push(coord);
    }
    if let Some(coord) = self.west {
      connected.push(coord);
    }
    connected.into_iter()
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Connections {
  north: bool,
  east: bool,
  south: bool,
  west: bool,
}

impl Connections {
  fn get_connected_directions(&self) -> HashSet<Direction> {
    let mut connected = HashSet::new();
    if self.north {
      connected.insert(Direction::North);
    }
    if self.east {
      connected.insert(Direction::East);
    }
    if self.south {
      connected.insert(Direction::South);
    }
    if self.west {
      connected.insert(Direction::West);
    }
    connected
  }
}

type Consequent = usize;

struct ProblemDefinition {
  board: Vec<Vec<char>>,
  start: Coord,
}

fn main() -> Result<(), String> {
  let data = extract()?;
  let result = transform(data);

  load(result)
}

#[mry::mry]
fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}

fn get_neighbors(board: &[Vec<char>], coord: &Coord) -> Neighbors {
  let mut north = None;
  let mut east = None;
  let mut south = None;
  let mut west = None;
  if coord.y > 0 {
    north = Some(Coord { y: coord.y - 1, x: coord.x });
  }
  if coord.y < board.len() - 1 {
    south = Some(Coord { y: coord.y + 1, x: coord.x });
  }
  if coord.x > 0 {
    west = Some(Coord { y: coord.y, x: coord.x - 1 });
  }
  if coord.x < board[0].len() - 1 {
    east = Some(Coord { y: coord.y, x: coord.x + 1 });
  }

  Neighbors { north, east, south, west }
}

fn resolve_start_character(board: &[Vec<char>], start: &Coord) -> Option<char> {
  let neighbors = get_neighbors(board, start);
  let connections = Connections {
    north: neighbors.north.map_or(false, |neighbor| {
      board[neighbor.y][neighbor.x] == '|'
        || board[neighbor.y][neighbor.x] == 'F'
        || board[neighbor.y][neighbor.x] == '7'
    }),
    east: neighbors.east.map_or(false, |neighbor| {
      board[neighbor.y][neighbor.x] == '-'
        || board[neighbor.y][neighbor.x] == 'J'
        || board[neighbor.y][neighbor.x] == '7'
    }),
    south: neighbors.south.map_or(false, |neighbor| {
      board[neighbor.y][neighbor.x] == '|'
        || board[neighbor.y][neighbor.x] == 'L'
        || board[neighbor.y][neighbor.x] == 'J'
    }),
    west: neighbors.west.map_or(false, |neighbor| {
      board[neighbor.y][neighbor.x] == '-'
        || board[neighbor.y][neighbor.x] == 'L'
        || board[neighbor.y][neighbor.x] == 'F'
    }),
  };

  let connected_directions = connections.get_connected_directions();

  match connected_directions.len() {
    2 => {
      if connected_directions.contains(&Direction::North)
        && connected_directions.contains(&Direction::East)
      {
        Some('L')
      } else if connected_directions.contains(&Direction::North)
        && connected_directions.contains(&Direction::West)
      {
        Some('J')
      } else if connected_directions.contains(&Direction::South)
        && connected_directions.contains(&Direction::East)
      {
        Some('F')
      } else if connected_directions.contains(&Direction::South)
        && connected_directions.contains(&Direction::West)
      {
        Some('7')
      } else if connected_directions.contains(&Direction::East)
        && connected_directions.contains(&Direction::West)
      {
        Some('-')
      } else if connected_directions.contains(&Direction::North)
        && connected_directions.contains(&Direction::South)
      {
        Some('|')
      } else {
        None
      }
    }
    _ => None,
  }
}


fn extract() -> Result<ProblemDefinition, String> {
  let mut start: Coord = Coord { y: 0, x: 0 };
  let mut found_start = false;
  let mut board = src_provider()?
    .lines()
    .enumerate()
    .map(|(y, l)| {
      let row = l.chars().collect::<Vec<_>>();
      if let Some(x) = row.iter().position(|c| c == &'S') {
        found_start = true;
        start = Coord { y, x };
      }

      row
    })
    .collect::<Vec<_>>();

  if !found_start {
    return Err("no start in data".to_string());
  }
  let Some(c) = resolve_start_character(&board, &start) else {
    return Err("Indeterminate Start position".to_string());
  };
  board[start.y][start.x] = c;

  Ok(ProblemDefinition { board, start })
}

fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
  let mut step_map = HashMap::new();
  let mut queue = VecDeque::from([(0, data.start)]);
  while let Some((step, coord)) = queue.pop_front() {
    let neighbors =
      Neighbors::get_neighbors_connected_by_coord(&data.board, &coord);
    neighbors.iter_coords().for_each(|c| {
      if !step_map.contains_key(&c) {
        queue.push_back((step + 1, c));
      }
      step_map.insert(c, step);
    });
  }

  if let Some((_key, value)) = step_map.iter().max_by_key(|&(_, v)| v) {
    return Ok(*value);
  }

  unreachable!()
}

fn load(result: Result<usize, String>) -> Result<(), String> {
  match result {
    Ok(value) => println!("{value} steps"),
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
