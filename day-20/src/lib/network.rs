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
  nodes: HashMap<Address, Box<dyn Machine>>,
  broadcaster: Option<Address>,
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

    Network { nodes, broadcaster, ..Network::default() }
  }
}

impl Network {
  pub fn run(&mut self, n: usize) -> Result<(usize, usize), String> {
    if self.broadcaster.is_none() {
      return Err("no broadcaster in network".to_string());
    };
    for i in 0..n {
      self.current_step = i + 1;
      if let Some(memoized_signal) = self.on_button_press()? {
        self.adjust_count_to_cycle(memoized_signal, i + 1, n);

        // Return early with adjusted log values
        return Ok((self.log.low, self.log.high));
      }
    }

    Ok((self.log.low, self.log.high))
  }

  fn adjust_count_to_cycle(
    &mut self,
    memoized_signal: Signal,
    mut current_step: usize,
    max_steps: usize,
  ) {
    eprintln!("cycle detected!");

    let state = self.get_network_state();
    let (j, _, (step, (prev_low, prev_high))) =
      self.memory.get_full(&(memoized_signal, state)).unwrap();
    let cycle_length = self.memory.len() - j;

    let cycle_lows = self.log.low - prev_low;
    let cycle_highs = self.log.high - prev_high;

    let step_cycle_length = current_step - step;
    if self.cycle_memory.len() < current_step {
      current_step -= 1;
    }
    let steps_remaining = max_steps - current_step;

    dbg!(
      max_steps,
      current_step,
      step_cycle_length,
      cycle_highs,
      cycle_lows,
      cycle_length,
    );

    let complete_cycle_count = steps_remaining / step_cycle_length;

    let remaining_after_cycles = steps_remaining % step_cycle_length;

    dbg!(
      max_steps,
      current_step,
      step_cycle_length,
      cycle_highs,
      cycle_lows,
      complete_cycle_count,
      cycle_length,
      remaining_after_cycles
    );

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
    eprintln!("button -Low-> {broadcaster}");

    Ok(self.cycle())
  }

  fn cycle(&mut self) -> Option<Signal> {
    let mut is_first = true;
    while let Some(signal) = self.signal_queue.pop_front() {
      if self.is_memoized(&signal) {
        return Some(signal);
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
