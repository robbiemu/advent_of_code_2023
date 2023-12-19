use indexmap::IndexMap;
use num_complex::Complex;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::ops::RangeInclusive;


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

#[cfg(not(feature = "part2"))]
const LEGAL_CONSECUTIVE_MOVES: LegalMoves = LegalMoves { min: 1, max: 3 };
#[cfg(feature = "part2")]
const LEGAL_CONSECUTIVE_MOVES: LegalMoves = LegalMoves { min: 4, max: 10 };

pub struct LegalMoves {
  pub min: i32,
  pub max: i32,
}

impl LegalMoves {
  pub fn range(&self) -> RangeInclusive<i32> {
    self.min..=self.max
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct ComplexWrapper(Complex<i32>);

impl ComplexWrapper {
  fn magnitude(&self) -> i32 {
    (self.0.norm_sqr() as f64).sqrt() as i32
  }
}

impl PartialOrd for ComplexWrapper {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for ComplexWrapper {
  fn cmp(&self, other: &Self) -> Ordering {
    let self_mag = self.magnitude();
    let other_mag = other.magnitude();

    self_mag
      .partial_cmp(&other_mag)
      .unwrap_or(Ordering::Equal)
      .then_with(|| self.0.re.cmp(&other.0.re))
      .then_with(|| self.0.im.cmp(&other.0.im))
  }
}

struct ProblemDefinition {
  grid: IndexMap<ComplexWrapper, i32>,
}

type Consequent = i32;

fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}

fn extract(data: &str) -> Result<ProblemDefinition, String> {
  let grid: IndexMap<ComplexWrapper, i32> = data
    .lines()
    .enumerate()
    .flat_map(|(i, row)| {
      row.chars().enumerate().map(move |(j, c)| {
        (
          ComplexWrapper(Complex::new(i as i32, j as i32)),
          c.to_digit(10).unwrap() as i32,
        )
      })
    })
    .collect();

  Ok(ProblemDefinition { grid })
}

fn find_shortest_path(
  end_point: ComplexWrapper,
  grid: &IndexMap<ComplexWrapper, i32>,
) -> Option<i32> {
  let one_imaginary = Complex::new(0, 1);
  let mut todo: BinaryHeap<Reverse<(i32, ComplexWrapper, ComplexWrapper)>> =
    BinaryHeap::new();
  // Starting from position (0, 0) with an initial direction of moving right and down
  todo.push(Reverse((
    0,
    ComplexWrapper(Complex::new(0, 0)),
    ComplexWrapper(Complex::new(0, 1)),
  )));
  todo.push(Reverse((
    0,
    ComplexWrapper(Complex::new(0, 0)),
    ComplexWrapper(Complex::new(1, 0)),
  )));


  let mut visited = HashSet::new();

  while let Some(Reverse((value, position, direction))) = todo.pop() {
    if position == end_point {
      return Some(value);
    }

    if visited.contains(&(position, direction)) {
      continue;
    }

    visited.insert((position, direction));

    for delta in [one_imaginary / direction.0, -one_imaginary / direction.0] {
      for steps in LEGAL_CONSECUTIVE_MOVES.range() {
        let new_position = position.0 + delta * steps;
        // eprintln!(
        //   "Debug Info: Current position: (y{},x{}), direction: (y{},x{}), \
        //    Delta: {delta}, Steps: {steps}",
        //   position.0.re, position.0.im, direction.0.re, direction.0.im
        // );

        if grid.get(&ComplexWrapper(new_position)).is_some() {
          let step_values: i32 = (1..=steps)
            .map(|j| grid.get(&ComplexWrapper(position.0 + delta * j)).unwrap())
            .sum();
          todo.push(Reverse((
            value + step_values,
            ComplexWrapper(new_position),
            ComplexWrapper(delta),
          )));
        }
      }
    }
  }

  None
}

fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
  let end_point = data.grid.last().ok_or("Empty map")?.0;

  // Check if end_point is within the bounds of the grid
  if !data.grid.contains_key(end_point) {
    return Err("End point is not in the grid".to_string());
  }

  let heat_loss =
    find_shortest_path(*end_point, &mut &data.grid).ok_or("no path found!")?;

  Ok(heat_loss)
}

fn load(result: Result<Consequent, String>) -> Result<(), String> {
  match result {
    Ok(heat_loss) => println!("heat_loss {heat_loss}"),
    Err(e) => eprintln!("{:?}", e),
  }

  Ok(())
}

fn main() -> Result<(), String> {
  let data_str = src_provider()?;
  let data = extract(&data_str)?;
  let result = transform(data);

  load(result)
}
