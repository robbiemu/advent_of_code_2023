use std::str::FromStr;

mod lib {
  pub mod path;
  pub mod petgraph {
    pub mod etc;
  }
  pub mod prelude;
  pub mod record;
}
use lib::path::expand_path;
use lib::petgraph::etc::{dijkstra, find_node_by_weight};
use lib::prelude::*;
use lib::record::{get_map_from_record, Record};

use crate::lib::path::update_record_with_path;


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample3.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

struct ProblemDefinition {
  map: GraphMap,
  record: Record,
  start: (usize, usize),
  end: (usize, usize),
}

impl FromStr for ProblemDefinition {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let record: Record = s.parse().expect("non-digit in map!");

    let start = (0, 0);
    let end = (record.rows - 1, record.cols - 1);
    let map = get_map_from_record(&record);

    dbg!(map.node_indices().len(), map.edge_indices().len());

    Ok(ProblemDefinition { map, record, start, end })
  }
}

#[mry::mry]
fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}

fn extract() -> Result<ProblemDefinition, String> {
  src_provider()?.parse()
}

fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
  let start_node = find_node_by_weight(&data.map, data.start).unwrap();
  let end_node = find_node_by_weight(&data.map, data.end);

  let path = dijkstra(&data.map, start_node, end_node, |c, p| {
    let Weight(weight, current) = c.weight();
    if p.is_none() {
      return Weight(*weight, *current);
    }

    let Weight(_, previous) = p.unwrap().weight();
    if current != previous && Direction::invert(current) != *previous {
      Weight(*weight, *current)
    } else {
      Weight(u32::MAX, Direction::NoDirection)
    }
  });

  if !path.is_empty() && end_node.is_some() {
    let end = end_node.unwrap();

    let char_map = update_record_with_path(
      &data.record,
      &expand_path(&path, &data.map, end),
    )
    .iter()
    .map(|row| row.iter().collect::<String>())
    .collect::<Vec<_>>()
    .join("\n");
    eprintln!("{char_map}");

    let (Weight(cost, _), _) = path[&end];

    Ok(cost)
  } else {
    Err("No path found".to_string())
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


#[cfg(test)]
mod tests {
  // use super::*;

  // MARK extract

  // MARK transform

  // MARK load
}
