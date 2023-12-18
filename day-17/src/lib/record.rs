use petgraph::{prelude::NodeIndex, Graph};
use std::{collections::HashMap, str::FromStr};

use crate::lib::prelude::*;


pub struct Record {
  pub grid: Vec<Vec<u32>>,
  pub rows: usize,
  pub cols: usize,
}

impl FromStr for Record {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
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
    let cols = grid[0].len();

    Ok(Record { grid, rows, cols })
  }
}

pub fn get_map_from_record(record: &Record) -> GraphMap {
  let mut map = Graph::<_, Weight>::new();
  let mut nodes: HashMap<(usize, usize), NodeIndex> = HashMap::new();
  for (y, line) in record.grid.iter().enumerate() {
    for (x, heat_loss) in line.iter().enumerate() {
      let destination =
        *nodes.entry((y, x)).or_insert_with(|| map.add_node((y, x)));
      nodes.insert((y, x), destination);

      for (ncoord, direction) in Direction::get_directed_neighbors(
        (y, x),
        PERSISTENCE,
        record.rows,
        record.cols,
      ) {
        let mut current_coord = ncoord;
        let inital_loss = record.grid[current_coord.0][current_coord.1];
        let mut accumulated_loss = 0;

        while current_coord != (y, x) {
          accumulated_loss += record.grid[current_coord.0][current_coord.1];

          current_coord = match direction {
            Direction::Left => (current_coord.0, current_coord.1 + 1),
            Direction::Right => (current_coord.0, current_coord.1 - 1),
            Direction::Up => (current_coord.0 + 1, current_coord.1),
            Direction::Down => (current_coord.0 - 1, current_coord.1),
            Direction::NoDirection => unreachable!(), // Shouldn't happen
          };
        }
        accumulated_loss -= inital_loss;

        let origin =
          *nodes.entry(ncoord).or_insert_with(|| map.add_node(ncoord));
        map.add_edge(
          origin,
          destination,
          Weight(heat_loss + accumulated_loss, Direction::invert(&direction)),
        );
      }
    }
  }

  map
}
