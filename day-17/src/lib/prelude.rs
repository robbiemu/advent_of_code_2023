use petgraph::{Directed, Graph};
use std::{cmp::Ordering, ops::AddAssign};

use crate::lib::petgraph::etc::Measure;


pub const PERSISTENCE: usize = 3;

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
    distance: usize,
    rows: usize,
    cols: usize,
  ) -> Vec<((usize, usize), Direction)> {
    let mut neighbors = Vec::new();

    for i in 1..=distance {
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

pub type GraphMap = Graph<(usize, usize), Weight, Directed>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Weight(pub u32, pub Direction);

impl Measure for Weight {
  fn default() -> Self {
    Weight(u32::default(), Direction::default())
  }
}

impl PartialOrd for Weight {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.0.cmp(&other.0))
  }
}

impl Ord for Weight {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap_or(Ordering::Equal)
  }
}

impl AddAssign for Weight {
  fn add_assign(&mut self, other: Self) {
    self.0 = self.0.saturating_add(other.0);
    self.1 = other.1;
  }
}

pub type Consequent = u32;
