use petgraph::{
  prelude::{Graph, NodeIndex},
  visit::{EdgeRef, IntoEdges, IntoNodeIdentifiers, VisitMap, Visitable},
};
use std::collections::{
  hash_map::Entry::{Occupied, Vacant},
  BinaryHeap, HashMap,
};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::AddAssign;


pub trait Measure: PartialOrd + Copy {
  fn default() -> Self;
}

struct MinScored<T>(T, NodeIndex);

impl<K> Ord for MinScored<K>
where
  K: Ord,
{
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.0.cmp(&other.0).reverse() // reverse for min heap behavior
  }
}

impl<K> PartialOrd for MinScored<K>
where
  K: Ord,
{
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl<K> PartialEq for MinScored<K>
where
  K: Ord,
{
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl<K> Eq for MinScored<K> where K: Ord {}

pub fn dijkstra<G, F, K>(
  graph: G,
  start: G::NodeId,
  goal: Option<G::NodeId>,
  mut edge_cost: F,
) -> HashMap<G::NodeId, (K, Option<NodeIndex>)>
where
  G: IntoEdges + IntoNodeIdentifiers + Visitable,
  G::NodeId: Eq + Hash + Copy + Debug + Into<NodeIndex> + From<NodeIndex>,
  G::EdgeId: Copy,
  F: FnMut(G::EdgeRef, Option<G::EdgeRef>) -> K,
  K: Measure + Copy + AddAssign + Ord + Debug,
{
  let mut visited = graph.visit_map();
  let mut scores = HashMap::new();
  let mut predecessor = HashMap::new();
  let mut visit_next = BinaryHeap::new();
  let zero_score = K::default();
  scores.insert(start, (zero_score, None));
  visit_next.push(MinScored(zero_score, start.into()));
  while let Some(MinScored(node_score, node)) = visit_next.pop() {
    println!("Popped node {:?} with score {:?}", node, node_score);
    if goal.as_ref() == Some(&node.into()) {
      break;
    }
    for edge in graph.edges(node.into()) {
      let next = edge.target();
      let mut next_score: K = node_score;
      next_score += edge_cost(
        edge,
        scores
          .get(&node.into())
          .and_then(|(_, prev_edge)| *prev_edge),
      );

      match scores.entry(next) {
        Occupied(mut ent) => {
          let (existing_score, _) = ent.get_mut();
          if next_score < *existing_score {
            *existing_score = next_score;
            ent.insert((next_score, Some(edge)));
            println!("Pushing node {:?} with score {:?}", next, next_score);
            visit_next.push(MinScored(next_score, next.into()));
            predecessor.insert(next, node);
          }
        }
        Vacant(ent) => {
          ent.insert((next_score, Some(edge)));
          println!("Pushing node {:?} with score {:?}", next, next_score);
          visit_next.push(MinScored(next_score, next.into()));
          predecessor.insert(next, node);
        }
      }
    }
    visited.visit(node.into());
  }

  scores
    .into_iter()
    .map(|(node, (score, _))| {
      let p = predecessor.get(&node).copied();

      (node, (score, p))
    })
    .collect()
}

pub fn find_node_by_weight<T, U>(
  graph: &Graph<T, U>,
  weight: T,
) -> Option<NodeIndex>
where
  T: PartialEq,
  U: PartialEq,
{
  graph.node_indices().find(|&node| graph[node] == weight)
}
