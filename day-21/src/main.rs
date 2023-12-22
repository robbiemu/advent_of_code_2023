use nalgebra::Vector2;
use std::collections::{HashMap, VecDeque};


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(feature = "sample")]
#[cfg(not(feature = "part2"))]
const STEPS: usize = 6;
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");
#[cfg(not(feature = "sample"))]
#[cfg(not(feature = "part2"))]
const STEPS: usize = 64;
#[cfg(feature = "part2")]
const STEPS: usize = 26501365;

struct ProblemDefinition {
  map: Vec<Vec<char>>,
  start: Vector2<usize>,
}
type Consequent = usize;


#[mry::mry]
fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}

fn locate_start(map: &[Vec<char>]) -> Result<Vector2<usize>, String> {
  for (y, row) in map.iter().enumerate() {
    for (x, &ch) in row.iter().enumerate() {
      if ch == 'S' {
        return Ok(Vector2::new(x, y));
      }
    }
  }

  Err("Start 'S' not found.".to_string())
}

fn extract() -> Result<ProblemDefinition, String> {
  let map: Vec<Vec<char>> = src_provider()?
    .lines()
    .map(|line| line.chars().collect())
    .collect();
  let start = locate_start(&map)?;

  Ok(ProblemDefinition { map, start })
}

fn get_location_distance(
  data: &ProblemDefinition,
) -> HashMap<Vector2<usize>, usize> {
  let mut visited: HashMap<Vector2<usize>, usize> = HashMap::new();
  let mut distances: HashMap<Vector2<usize>, usize> = HashMap::new();
  let mut to_visit: VecDeque<(Vector2<usize>, usize)> = VecDeque::new();
  to_visit.push_back((data.start, 0));
  let (rows, cols) = (data.map.len(), data.map[0].len());
  let neighbors = vec![
    Vector2::new(0, 1),
    Vector2::new(0, -1),
    Vector2::new(1, 0),
    Vector2::new(-1, 0),
  ];
  while let Some((position, steps)) = to_visit.pop_front() {
    if visited.contains_key(&position) {
      continue;
    }
    visited.insert(position, steps);
    distances.insert(position, steps);

    for neighbor in &neighbors {
      let new_pos_isize = Vector2::new(
        (position.x as isize) + neighbor.x,
        (position.y as isize) + neighbor.y,
      );

      if new_pos_isize.x >= 0
        && new_pos_isize.y >= 0
        && new_pos_isize.x < cols as isize
        && new_pos_isize.y < rows as isize
      {
        let new_pos =
          Vector2::new(new_pos_isize.x as usize, new_pos_isize.y as usize);

        let row = data.map.get(new_pos.y).unwrap();
        let &ch = row.get(new_pos.x).unwrap();
        if ch != '#' {
          to_visit.push_back((new_pos, steps + 1));
        }
      }
    }
  }

  distances
}


fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
  let location_data = get_location_distance(&data);
  #[cfg(not(feature = "part2"))]
  {
    let visitable: Vec<Vector2<usize>> = location_data
      .iter()
      .filter(|(&_, &distance)| {
        distance <= STEPS && (distance % 2) == (STEPS % 2)
      })
      .map(|(pos, _)| pos.to_owned())
      .collect();

    println!(
      "{}",
      data
        .map
        .iter()
        .enumerate()
        .map(|(y, row)| {
          let r = row
            .iter()
            .enumerate()
            .map(|(x, col)| {
              if visitable.contains(&Vector2::new(x, y)) {
                &'O'
              } else {
                col
              }
            })
            .collect::<String>();

          format!("{r}\n")
        })
        .collect::<String>()
    );

    match visitable.len() {
      0 => Err("no visitable locations".to_string()),
      _ => Ok(visitable.len()),
    }
  }
  #[cfg(feature = "part2")]
  {
    // (verified) rows and cols are same in input.
    let span = data.map.len();
    // (verified) STEPS in part 2 chosen to evenly compose this many repeticiones
    let n = (STEPS - (span / 2)) / span;
    /* we need n of these pq are an even distance and there are n repetitions of
    our map in a direction. these were excluded when they aught not to have been
    (the diamond is not jaggigty at the scale of our whole map, but at tiles) */
    let excluded_even_positions = n
      * location_data
        .values()
        .filter(|&distance| distance % 2 == 0 && distance > &span)
        .count();
    // but we need n+ 1 of these because these are an odd distance
    let excluded_odd_positions = (n + 1)
      * location_data
        .values()
        .filter(|&distance| distance % 2 == 1 && distance > &span)
        .count();

    // we need n^2 here because there are two dimensions of expansion in an area.
    let evens = n.pow(2)
      * location_data
        .values()
        .filter(|&distance| distance % 2 == 0)
        .count();
    // n + 1 because it is an odd distance
    let odds = (n + 1).pow(2)
      * location_data
        .values()
        .filter(|&distance| distance % 2 == 1)
        .count();

    let positions =
      evens + odds + excluded_even_positions - excluded_odd_positions;

    Ok(positions)
  }
}

fn load(result: Result<Consequent, String>) -> Result<(), String> {
  match result {
    Ok(positions) => println!("positions {positions}"),
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
  // use super::*;

  // MARK extract

  // MARK transform

  // MARK load
}
