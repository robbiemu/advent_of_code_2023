use indexmap::IndexMap;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};

use crate::lib::machine::{Machine, MachineType};
use crate::lib::prelude::*;


#[derive(Default)]
pub struct Network {
  signal_queue: VecDeque<Signal>,
  memory: IndexMap<(Signal, u64), (usize, (usize, usize))>,
  cycle_memory: Vec<(usize, (usize, usize))>,
  #[cfg(feature = "part2")]
  input_memory: HashMap<Address, usize>,
  nodes: HashMap<Address, Box<dyn Machine>>,
  broadcaster: Option<Address>,
  #[cfg(feature = "part2")]
  outputs: Vec<Address>,
  log: SignalLog,
  current_step: usize,
}

impl From<Vec<Box<dyn Machine>>> for Network {
  fn from(machines: Vec<Box<dyn Machine>>) -> Self {
    let mut broadcaster = None;
    let nodes = machines.into_iter().fold(HashMap::new(), |mut acc, v| {
      let k = v.get_address();
      if v.get_type() == MachineType::Broadcaster {
        broadcaster = Some(v.get_address());
      }
      acc.insert(k, v);

      acc
    });

    #[cfg(not(feature = "part2"))]
    {
      Network { nodes, broadcaster, ..Network::default() }
    }
    #[cfg(feature = "part2")]
    {
      let mut outputs: Vec<Address> = Vec::new();
      let addresses: Vec<Address> = nodes.keys().cloned().collect();
      let output = nodes.values().fold(None, |mut acc, node| {
        let node_outputs = node.get_unique_outputs(&addresses);
        match node_outputs.len() {
          1 => acc = Some(node_outputs.first().unwrap().clone()),
          0 => (),
          _ => unimplemented!(),
        }

        acc
      });
      if let Some(o) = output {
        if let Some(interim) = nodes
          .iter()
          .filter_map(|(address, node)| {
            if node.reports_to(&o) {
              Some(address)
            } else {
              None
            }
          })
          .next()
        {
          outputs = nodes
            .iter()
            .filter_map(|(address, node)| {
              if node.reports_to(interim) {
                Some(address)
              } else {
                None
              }
            })
            .cloned()
            .collect();
        }
      }

      Network { nodes, broadcaster, outputs, ..Network::default() }
    }
  }
}

impl Network {
  #[cfg(not(feature = "part2"))]
  pub fn run(&mut self, n: usize) -> Result<(usize, usize), String> {
    if self.broadcaster.is_none() {
      return Err("no broadcaster in network".to_string());
    };

    for i in 0..n {
      self.current_step = i + 1;
      if let Some(memoized_signal) = self.on_button_press()? {
        eprintln!("cycle detected!");
        self.adjust_count_to_cycle(memoized_signal, i + 1, n);

        // Return early with adjusted log values
        return Ok((self.log.low, self.log.high));
      }
    }

    Ok((self.log.low, self.log.high))
  }

  #[cfg(feature = "part2")]
  pub fn run(&mut self) -> Result<(usize, usize), String> {
    if self.broadcaster.is_none() {
      return Err("no broadcaster in network".to_string());
    };

    for i in 0..10000 {
      self.current_step = i + 1;
      if self.on_button_press()?.is_some() {
        let lcm =
          get_lcm(self.input_memory.values().copied().collect::<Vec<_>>())
            .unwrap();

        return Ok((1, lcm));
      }
    }

    Err("no cycle found".to_string())
  }

  #[cfg(not(feature = "part2"))]
  fn adjust_count_to_cycle(
    &mut self,
    memoized_signal: Signal,
    mut current_step: usize,
    max_steps: usize,
  ) {
    let state = self.get_network_state();
    // let (j, _, (step, (prev_low, prev_high))) =
    //   self.memory.get_full(&(memoized_signal, state)).unwrap();
    let (_, _, (step, (prev_low, prev_high))) =
      self.memory.get_full(&(memoized_signal, state)).unwrap();

    let cycle_lows = self.log.low - prev_low;
    let cycle_highs = self.log.high - prev_high;
    let step_cycle_length = current_step - step;
    if self.cycle_memory.len() < current_step {
      current_step -= 1;
    }
    let steps_remaining = max_steps - current_step;
    let complete_cycle_count = steps_remaining / step_cycle_length;
    let remaining_after_cycles = steps_remaining % step_cycle_length;

    // let cycle_length = self.memory.len() - j;
    // dbg!(
    //   max_steps,
    //   current_step,
    //   step_cycle_length,
    //   cycle_highs,
    //   cycle_lows,
    //   complete_cycle_count,
    //   cycle_length,
    //   remaining_after_cycles
    // );

    self.log.low += complete_cycle_count * cycle_lows + remaining_after_cycles;
    self.log.high +=
      complete_cycle_count * cycle_highs + remaining_after_cycles;
  }

  fn on_button_press(&mut self) -> Result<Option<Signal>, String> {
    let Some(broadcaster) = self.broadcaster.to_owned() else {
      return Err("no broadcaster in network".to_string());
    };
    self.signal_queue.push_back(Signal {
      from: None,
      pulse: Pulse::Low,
      to: broadcaster.to_owned(),
    });
    // eprintln!("button -Low-> {broadcaster}");

    Ok(self.cycle())
  }

  fn cycle(&mut self) -> Option<Signal> {
    let mut is_first = true;
    while let Some(signal) = self.signal_queue.pop_front() {
      if self.is_memoized(&signal) {
        return Some(signal);
      }
      #[cfg(feature = "part2")]
      {
        if self.outputs.contains(&signal.to) && signal.pulse == Pulse::Low {
          self
            .input_memory
            .insert(signal.to.to_owned(), self.current_step);
          if self.input_memory.len() == self.outputs.len() {
            return Some(signal);
          }
        }
      }

      self.log.register(&signal.pulse);
      if !is_first {
        if self.cycle_memory.len() < self.current_step {
          self.cycle_memory.push((0, (0, 0)));
        }
        self.cycle_memory[self.current_step - 1] =
          (0, (self.log.low, self.log.high));
      }
      self.send_signal(signal);
      is_first = false;
    }

    None
  }

  fn send_signal(&mut self, signal: Signal) {
    let to_address = signal.to.clone();
    if let Some(node) = self.nodes.get_mut(&to_address) {
      if let Some(next) = node.on_signal(signal) {
        self.signal_queue.extend(next.into_iter().map(|mut s| {
          s.from = Some(to_address.clone());
          s
        }));
      }
    }
  }

  fn is_memoized(&mut self, signal: &Signal) -> bool {
    let state = self.get_network_state();
    if self.memory.contains_key(&(signal.clone(), state)) {
      return true;
    }

    self.memory.insert(
      (signal.to_owned(), state),
      (self.current_step, (self.log.low, self.log.high)),
    );

    false
  }

  fn get_network_state(&self) -> u64 {
    let mut hasher = DefaultHasher::new();
    for node in self.nodes.values() {
      let value = node.get_state();
      value.hash(&mut hasher);
    }
    for signal in &self.signal_queue {
      signal.hash(&mut hasher);
    }

    hasher.finish()
  }
}

#[derive(Default)]
struct SignalLog {
  low: usize,
  high: usize,
}

impl SignalLog {
  fn register(&mut self, pulse: &Pulse) {
    match pulse {
      Pulse::Low => self.low += 1,
      Pulse::High => self.high += 1,
      _ => unimplemented!(),
    }
  }
}
