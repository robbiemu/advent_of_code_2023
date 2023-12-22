mod lib {
  pub mod machine;
  pub mod network;
  pub mod prelude;
}
use lib::{
  machine::{
    Broadcaster, Conjunction, FlipFlop, FromNode, Machine, MachineType, Node,
    Output,
  },
  network::Network,
  prelude::Address,
};


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample1.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");


struct ProblemDefinition {
  network: Network,
}
type Consequent = (usize, usize);


#[mry::mry]
fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}

fn extract() -> Result<ProblemDefinition, String> {
  let mut specifications: Vec<(MachineType, Node)> = Vec::new();
  for line in src_provider()?.lines() {
    let parts: Vec<&str> = line.split(" -> ").collect();
    if parts.len() != 2 {
      return Err("Invalid input format".to_string());
    }

    let machine_type = match parts[0].chars().next() {
      Some('%') => MachineType::FlipFlop,
      Some('&') => MachineType::Conjunction,
      Some('b') => MachineType::Broadcaster,
      _ => MachineType::Output,
    };

    let address = parts[0]
      .trim_start_matches('%')
      .trim_start_matches('&')
      .to_string();
    let output: Vec<Address> = parts[1]
      .split(',')
      .map(|output| output.trim().to_string())
      .collect();

    let node = Node {
      address,
      input: Vec::new(), // we fill this in later
      output,
    };

    specifications.push((machine_type, node));
  }

  let mut machines: Vec<Box<dyn Machine>> = Vec::new();
  for (machine_type, node) in specifications.iter() {
    let mut inputs: Vec<Address> = Vec::new();
    // fill the input
    for (_, other_node) in specifications.iter() {
      if other_node.address == node.address {
        continue;
      }
      if other_node.output.contains(&node.address) {
        inputs.push(other_node.address.clone());
      }
    }
    let mut updated_node = node.clone();
    updated_node.input = inputs;

    let machine: Box<dyn Machine> = match machine_type {
      MachineType::FlipFlop => Box::new(FlipFlop::from_node(&updated_node)),
      MachineType::Conjunction => {
        Box::new(Conjunction::from_node(&updated_node))
      }
      MachineType::Broadcaster => {
        Box::new(Broadcaster::from_node(&updated_node))
      }
      MachineType::Output => Box::new(Output::from_node(&updated_node)),
    };
    machines.push(machine);
  }

  // Create the network from the machines
  let network = Network::from(machines);

  Ok(ProblemDefinition { network })
}

fn transform(mut data: ProblemDefinition) -> Result<Consequent, String> {
  #[cfg(not(feature = "part2"))]
  {
    data.network.run(1000)
  }
  #[cfg(feature = "part2")]
  {
    data.network.run()
  }
}

fn load(result: Result<Consequent, String>) -> Result<(), String> {
  match result {
    Ok((low, high)) => println!("low_{low} × high_{high} = {}", low * high),
    Err(e) => eprintln!("{e}"),
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
