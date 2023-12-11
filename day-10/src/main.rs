#[cfg(not(feature = "part2"))]
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};


#[cfg(feature = "sample")]
#[cfg(not(feature = "part2"))]
const DATA: &str = include_str!("../sample.txt");
#[cfg(all(feature = "sample", feature = "part2"))]
const DATA: &str = include_str!("../sample_3.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
  x: usize,
  y: usize,
}

#[cfg(feature = "part2")]
impl Coord {
  fn get_neighbors(&self) -> HashSet<Coord> {
    let mut neighbors = HashSet::new();
    for &dx in [-1, 0, 1].iter() {
      for &dy in [-1, 0, 1].iter() {
        if dx != 0 || dy != 0 {
          neighbors.insert(Coord {
            x: (self.x as isize + dx) as usize,
            y: (self.y as isize + dy) as usize,
          });
        }
      }
    }
    neighbors
  }

  fn is_at_edge(&self, width: usize, height: usize) -> bool {
    self.x == 0 || self.y == 0 || self.x == width - 1 || self.y == height - 1
  }
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

fn get_neighbors_on_board(board: &[Vec<char>], coord: &Coord) -> Neighbors {
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
  let neighbors = get_neighbors_on_board(board, start);
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

fn flood_fill(
  width: usize,
  height: usize,
  loop_coords: HashSet<Coord>,
) -> HashSet<Coord> {
  let mut outside = HashSet::new();
  let mut all_coords = HashSet::new();
  for y in 0..height {
    for x in 0..width {
      let c = Coord { x, y };
      all_coords.insert(c);
      if !loop_coords.contains(&c) && c.is_at_edge(width, height) {
        outside.insert(c.to_owned());
      }
    }
  }

  let mut queue: VecDeque<_> = outside.clone().into_iter().collect();
  while let Some(curr) = queue.pop_front() {
    for neighbor in curr.get_neighbors() {
      if neighbor.x < width
        && neighbor.y < height
        && !loop_coords.contains(&neighbor)
        && !outside.contains(&neighbor)
      {
        outside.insert(neighbor);
        queue.push_back(neighbor);
      }
    }
  }

  let outside_and_loop: HashSet<_> =
    outside.union(&loop_coords).cloned().collect();
  let inside: HashSet<Coord> =
    all_coords.difference(&outside_and_loop).cloned().collect();

  inside
}

fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
  #[cfg(not(feature = "part2"))]
  {
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
    } else {
      return Err("Empty step map".to_string());
    }
  }
  #[cfg(feature = "part2")]
  {
    // find the coords of the largest loop
    let mut largest_loop = HashSet::new();
    let mut queue = VecDeque::from([(0, data.start)]);
    while let Some((step, coord)) = queue.pop_front() {
      let neighbors =
        Neighbors::get_neighbors_connected_by_coord(&data.board, &coord);
      neighbors.iter_coords().for_each(|c| {
        if !largest_loop.contains(&c) {
          queue.push_back((step + 1, c));
        }
        largest_loop.insert(c);
      });
    }
    let height = data.board.len();
    let width = data.board[0].len();

    // Adjust loop coordinates to include new '.' cells
    let mut new_largest_loop = HashSet::new();
    for original_coord in &largest_loop {
      let coord =
        Coord { y: original_coord.y * 2 + 1, x: original_coord.x * 2 + 1 };
      new_largest_loop.insert(coord); // Add original loop coordinates

      let original_neighbors = Neighbors::get_neighbors_connected_by_coord(
        &data.board,
        original_coord,
      );
      original_neighbors.iter_coords().for_each(|oc| {
        let c = Coord { y: oc.y * 2 + 1, x: oc.x * 2 + 1 };
        let space_between =
          Coord { y: (c.y + coord.y) / 2, x: (c.x + coord.x) / 2 };
        new_largest_loop.insert(space_between);
      });
    }
    //dbg!((&largest_loop.len(), &new_largest_loop.len()));

    // find all points in the larger board that are inside
    let enlarged_inside =
      flood_fill(width * 2 + 1, height * 2 + 1, new_largest_loop);
    let inside_positions = enlarged_inside
      .iter()
      .filter(|coord| coord.x % 2 != 0 && coord.y % 2 != 0)
      .map(|c| Coord { y: (c.y - 1) / 2, x: (c.x - 1) / 2 }) // get the original coordinate
      .collect::<HashSet<_>>();

    Ok(inside_positions.len())
  }
}

fn load(result: Result<usize, String>) -> Result<(), String> {
  match result {
    Ok(value) => {
      #[cfg(not(feature = "part2"))]
      println!("{value} steps");
      #[cfg(feature = "part2")]
      println!("{value} inside");
    }
    Err(e) => eprintln!("{e}"),
  }

  Ok(())
}
