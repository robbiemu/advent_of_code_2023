use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};

use crate::lib::prelude::*;


#[derive(PartialEq, Eq)]
pub enum MachineType {
  Broadcaster,
  FlipFlop,
  Conjunction,
  Output,
}

#[derive(Clone, Default)]
pub struct Node {
  pub address: Address,
  pub input: Vec<Address>,
  pub output: Vec<Address>,
}

pub trait FromNode {
  fn from_node(node: &Node) -> Self;
}

pub trait Machine: Stateful {
  fn get_address(&self) -> Address;
  fn get_type(&self) -> MachineType;
  fn on_signal(&mut self, signal: Signal) -> Option<Vec<Signal>>;
  #[cfg(feature = "part2")]
  fn reports_to(&self, address: &Address) -> bool;
  #[cfg(feature = "part2")]
  fn get_unique_outputs(&self, addresses: &[Address]) -> Vec<Address>;
}

pub trait Stateful {
  fn get_state(&self) -> State;
}

#[derive(Default)]
pub struct Broadcaster(Node);

impl FromNode for Broadcaster {
  fn from_node(node: &Node) -> Self {
    Self(node.to_owned())
  }
}

impl Machine for Broadcaster {
  #[cfg(feature = "part2")]
  fn reports_to(&self, address: &Address) -> bool {
    self.0.output.contains(address)
  }
  #[cfg(feature = "part2")]
  fn get_unique_outputs(&self, addresses: &[Address]) -> Vec<Address> {
    self
      .0
      .output
      .iter()
      .filter(|item| !addresses.contains(item))
      .cloned()
      .collect()
  }

  fn get_type(&self) -> MachineType {
    MachineType::Broadcaster
  }

  fn get_address(&self) -> Address {
    self.0.address.clone()
  }

  fn on_signal(&mut self, signal: Signal) -> Option<Vec<Signal>> {
    if self.0.output.is_empty() {
      return None;
    }

    Some(
      self
        .0
        .output
        .iter()
        .map(|address| {
          let to = address.clone();

          // eprintln!("{} -{:?}-> {to}", self.0.address, signal.pulse);

          Signal { pulse: signal.pulse, to, ..Signal::default() }
        })
        .collect(),
    )
  }
}

impl Stateful for Broadcaster {
  fn get_state(&self) -> State {
    0
  }
}

#[derive(Default)]
pub struct FlipFlop {
  node: Node,
  state: bool,
}

impl FromNode for FlipFlop {
  fn from_node(node: &Node) -> Self {
    Self { node: node.clone(), state: false }
  }
}

impl Machine for FlipFlop {
  #[cfg(feature = "part2")]
  fn reports_to(&self, address: &Address) -> bool {
    self.node.output.contains(address)
  }
  #[cfg(feature = "part2")]
  fn get_unique_outputs(&self, addresses: &[Address]) -> Vec<Address> {
    self
      .node
      .output
      .iter()
      .filter(|item| !addresses.contains(item))
      .cloned()
      .collect()
  }

  fn get_type(&self) -> MachineType {
    MachineType::FlipFlop
  }

  fn get_address(&self) -> Address {
    self.node.address.clone()
  }

  fn on_signal(&mut self, signal: Signal) -> Option<Vec<Signal>> {
    let mut pulse = Pulse::None;
    if signal.pulse == Pulse::Low {
      self.state = !self.state;
      pulse = match self.state {
        true => Pulse::High,
        false => Pulse::Low,
      };
    }
    if pulse == Pulse::None {
      return None;
    }

    Some(
      self
        .node
        .output
        .iter()
        .map(|address| {
          let to = address.clone();

          // eprintln!("%{} -{:?}-> {to}", self.node.address, pulse);

          Signal { pulse, to, ..Signal::default() }
        })
        .collect(),
    )
  }
}

impl Stateful for FlipFlop {
  fn get_state(&self) -> State {
    self.state as u64
  }
}

#[derive(Default)]
pub struct Conjunction {
  node: Node,
  state: HashMap<Address, Pulse>,
}

impl FromNode for Conjunction {
  fn from_node(node: &Node) -> Self {
    let state = node.input.iter().fold(HashMap::new(), |mut acc, address| {
      acc.insert(address.to_owned(), Pulse::Low);

      acc
    });

    Self { node: node.clone(), state }
  }
}

impl Machine for Conjunction {
  #[cfg(feature = "part2")]
  fn reports_to(&self, address: &Address) -> bool {
    self.node.output.contains(address)
  }
  #[cfg(feature = "part2")]
  fn get_unique_outputs(&self, addresses: &[Address]) -> Vec<Address> {
    self
      .node
      .output
      .iter()
      .filter(|item| !addresses.contains(item))
      .cloned()
      .collect()
  }

  fn get_type(&self) -> MachineType {
    MachineType::Conjunction
  }

  fn get_address(&self) -> Address {
    self.node.address.clone()
  }

  fn on_signal(&mut self, signal: Signal) -> Option<Vec<Signal>> {
    let Some(from) = signal.from else {
      return None;
    };
    if !self.node.input.contains(&from) {
      return None;
    }
    self.state.insert(from, signal.pulse);

    let pulse = if self.state.values().all(|v| v == &Pulse::High) {
      Pulse::Low
    } else {
      Pulse::High
    };

    Some(
      self
        .node
        .output
        .iter()
        .map(|address| {
          let to = address.clone();

          // eprintln!("&{} -{:?}-> {to}", self.node.address, pulse);

          Signal { pulse, to, ..Signal::default() }
        })
        .collect(),
    )
  }
}

impl Stateful for Conjunction {
  fn get_state(&self) -> State {
    let mut hasher = DefaultHasher::new();
    for (key, value) in &self.state {
      key.hash(&mut hasher);
      value.hash(&mut hasher);
    }

    hasher.finish()
  }
}

#[derive(Default)]
pub struct Output(Node);

impl FromNode for Output {
  fn from_node(node: &Node) -> Self {
    Self(node.to_owned())
  }
}

impl Machine for Output {
  #[cfg(feature = "part2")]
  fn reports_to(&self, address: &Address) -> bool {
    self.0.output.contains(address)
  }
  #[cfg(feature = "part2")]
  fn get_unique_outputs(&self, addresses: &[Address]) -> Vec<Address> {
    self
      .0
      .output
      .iter()
      .filter(|item| !addresses.contains(item))
      .cloned()
      .collect()
  }

  fn get_type(&self) -> MachineType {
    MachineType::Output
  }

  fn get_address(&self) -> Address {
    self.0.address.clone()
  }

  fn on_signal(&mut self, _signal: Signal) -> Option<Vec<Signal>> {
    None
  }
}

impl Stateful for Output {
  fn get_state(&self) -> State {
    0
  }
}
