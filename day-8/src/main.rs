use petgraph::{
  graph::{DiGraph, NodeIndex},
  visit::EdgeRef,
  Direction,
};
use sscanf::sscanf;
use std::collections::{BTreeMap, HashSet};


const DATA: &str = include_str!("../input.txt");
const START_NODE: &str = "AAA";
const TARGET_NODE: &str = "ZZZ";

#[derive(Debug)]
struct ProblemDefinition {
  instructions: Vec<char>,
  root_index: NodeIndex,
  graph: DiGraph<String, EdgeLabel>,
}

#[derive(Debug, PartialEq)]
enum EdgeLabel {
  Left,
  Right,
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

fn extract() -> Result<ProblemDefinition, String> {
  let src = src_provider()?;
  let mut lines = src.lines();
  let instructions: Vec<char> = lines
    .next()
    .map(|line| line.chars())
    .ok_or("empty input".to_string())?
    .collect();

  if let Some(_pos) = instructions.iter().position(|c| c != &'R' && c != &'L') {
    return Err(format!(
      "invalid line {}",
      instructions.iter().collect::<String>()
    ));
  }

  let mut graph = DiGraph::new();
  let mut node_indices = BTreeMap::new();
  let mut root_node_index = None;
  for line in lines.skip(1) {
    if let Ok((node, left, right)) = sscanf!(line, "{str} = ({str}, {str})") {
      // Add nodes if they don't exist
      let node_index = *node_indices
        .entry(node.to_string())
        .or_insert_with(|| graph.add_node(node.to_string()));

      let left_index = *node_indices
        .entry(left.to_string())
        .or_insert_with(|| graph.add_node(left.to_string()));

      let right_index = *node_indices
        .entry(right.to_string())
        .or_insert_with(|| graph.add_node(right.to_string()));

      // Add edges with the corresponding edge labels
      let edge_label_left = EdgeLabel::Left;
      let edge_label_right = EdgeLabel::Right;

      graph.add_edge(node_index, left_index, edge_label_left);
      graph.add_edge(node_index, right_index, edge_label_right);
      if node == "AAA" {
        root_node_index = Some(node_index);
      }
    } else {
      return Err(format!("Failed to parse line\n{line}"));
    }
  }

  if let Some(root_index) = root_node_index {
    Ok(ProblemDefinition { instructions, root_index, graph })
  } else {
    Err("No root node found".to_string())
  }
}

fn transform(data: ProblemDefinition) -> Result<usize, String> {
  let instruction_stream = data.instructions.iter().cycle();
  let mut node_index = data.root_index;
  let mut visited = HashSet::new();
  for (hops, instruction) in instruction_stream.enumerate() {
    if data.graph[node_index] == TARGET_NODE {
      return Ok(hops);
    }

    let key = (hops % data.instructions.len(), node_index);
    if visited.contains(&key) {
      return Err("infinite loop found".to_string());
    }
    visited.insert(key);

    let label = match instruction {
      'L' => EdgeLabel::Left,
      'R' => EdgeLabel::Right,
      _ => return Err(format!("invalid instruction in stream: {instruction}")),
    };

    let edge_references =
      data.graph.edges_directed(node_index, Direction::Outgoing);
    for edge in edge_references {
      let edge_label = edge.weight();
      if *edge_label == label {
        let neighbor = edge.target();

        node_index = neighbor;
      }
    }
  }

  unreachable!()
}

fn load(result: Result<usize, String>) -> Result<(), String> {
  match result {
    Ok(steps) => println!("{steps} steps"),
    Err(e) => eprintln!("{e}"),
  }

  Ok(())
}


#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE_DATA_1: &str = include_str!("../sample_1.txt");
  const SAMPLE_DATA_2: &str = include_str!("../sample_2.txt");

  // MARK extract
  #[test]
  #[mry::lock(src_provider)]
  fn it_should_use_data_from_tests() {
    mock_src_provider().returns(Ok(SAMPLE_DATA_1.to_string()));

    let data = src_provider();
    assert!(data.is_ok());

    assert!(!data.unwrap().is_empty());
  }

  // MARK transform
  #[test]
  #[mry::lock(src_provider)]
  fn it_should_find_route_to_target_node() {
    mock_src_provider().returns(Ok(SAMPLE_DATA_2.to_string()));

    let data = extract();
    assert!(data.is_ok());
    let result = transform(data.unwrap());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 6);
  }

  // MARK load
}
