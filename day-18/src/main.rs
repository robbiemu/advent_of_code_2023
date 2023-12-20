use sscanf::sscanf;
use std::str::FromStr;


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

enum Direction {
  Right,
  Down,
  Left,
  Up,
}

impl Direction {
  fn coordinate_rotation(&self) -> (isize, isize) {
    match self {
      Direction::Right => (0, 1),
      Direction::Down => (1, 0),
      Direction::Left => (0, -1),
      Direction::Up => (-1, 0),
    }
  }
}

impl FromStr for Direction {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    #[cfg(not(feature = "part2"))]
    {
      match s {
        "R" => Ok(Direction::Right),
        "D" => Ok(Direction::Down),
        "L" => Ok(Direction::Left),
        "U" => Ok(Direction::Up),
        _ => Err("Invalid direction".to_string()),
      }
    }
    #[cfg(feature = "part2")]
    {
      match s {
        "0" => Ok(Direction::Right),
        "1" => Ok(Direction::Down),
        "2" => Ok(Direction::Left),
        "3" => Ok(Direction::Up),
        _ => Err("Invalid direction".to_string()),
      }
    }
  }
}

struct Traversal {
  direction: Direction,
  distance: isize,
}

struct ProblemDefinition {
  trench: Vec<Traversal>,
}

impl ProblemDefinition {
  fn calculate_area(&self) -> Result<usize, String> {
    let mut total_row = 0;
    let mut perimeter = 0;
    let mut area = 0;


    for traversal in &self.trench {
      let (dy, dx) = traversal.direction.coordinate_rotation();
      let dy = dy * traversal.distance;
      let dx = dx * traversal.distance;

      total_row += dx;
      perimeter += traversal.distance;
      area += total_row * dy;
    }

    Ok((area + perimeter / 2 + 1) as usize)
  }
}

type Consequent = usize;


#[mry::mry]
fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}

#[allow(unused_variables, unused_assignments, unused_mut)]
fn extract() -> Result<ProblemDefinition, String> {
  let mut trench = Vec::new();

  for line in src_provider()?.lines() {
    let (direction_str, mut distance, color_str) =
      sscanf!(line, "{} {} (#{})", String, isize, String)
        .map_err(|e| format!("Error with line: {line}\n{:?}", e))?;

    let direction = ({
      #[cfg(not(feature = "part2"))]
      {
        Direction::from_str(&direction_str)
      }
      #[cfg(feature = "part2")]
      {
        Direction::from_str(
          &color_str
            .chars()
            .last()
            .ok_or("Empty color string")?
            .to_string(),
        )
      }
    })
    .map_err(|e| format!("Error (in direction) with line: {line}\n{:?}", e))?;

    #[cfg(feature = "part2")]
    {
      distance = u64::from_str_radix(&color_str[..color_str.len() - 1], 16)
        .map_err(|e| {
          format!("Error (in distance) with line: {line}\n{:?}", e)
        })? as isize;
    }

    trench.push(Traversal { direction, distance });
  }

  Ok(ProblemDefinition { trench })
}

fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
  data.calculate_area()
}

fn load(result: Result<Consequent, String>) -> Result<(), String> {
  match result {
    Ok(area) => println!("area: {area}"),
    Err(e) => println!("{}", e),
  }

  Ok(())
}

fn main() -> Result<(), String> {
  let data = extract()?;
  let result = transform(data);

  load(result)
}
