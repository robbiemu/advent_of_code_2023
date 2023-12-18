use petgraph::prelude::NodeIndex;
use std::collections::HashMap;

use super::{
  prelude::{GraphMap, Weight},
  record::Record,
};


pub fn expand_path(
  path: &HashMap<NodeIndex, (Weight, Option<NodeIndex>)>,
  graph: &GraphMap,
  end_node: NodeIndex,
) -> Vec<(usize, usize)> {
  let mut expanded_path = vec![*graph.node_weight(end_node).unwrap()];
  let mut current_node = end_node;

  while let Some(&(_, prev_node)) = path.get(&current_node) {
    if let Some(node) = prev_node {
      if let Some(weight) = graph.node_weight(node) {
        expanded_path.push(*weight);
        current_node = node;
      } else {
        break; // Node not found in the graph
      }
    } else {
      break; // Reached the start node
    }
  }
  expanded_path.reverse(); // Reverse the path to start from the beginning

  expanded_path
}

pub fn update_record_with_path(
  data: &Record,
  path: &[(usize, usize)],
) -> Vec<Vec<char>> {
  let mut record: Vec<Vec<char>> = data
    .grid
    .iter()
    .map(|row| {
      row
        .iter()
        .map(|&val| char::from_digit(val, 10).unwrap())
        .collect()
    })
    .collect();

  dbg!(path);

  for node_weight in path {
    let (y, x) = *node_weight; // Destructure the NodeWeight tuple
    record[y][x] = '#';
  }

  record
}
