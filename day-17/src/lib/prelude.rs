use petgraph::{Directed, Graph};
use std::ops::AddAssign;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Weight(pub u32, pub Direction);

impl Measure for Weight {
  fn default() -> Self {
    Weight(u32::default(), Direction::default())
  }
}

impl AddAssign for Weight {
  fn add_assign(&mut self, other: Self) {
    self.0 = self.0.saturating_add(other.0);
  }
}

pub type Consequent = u32;
