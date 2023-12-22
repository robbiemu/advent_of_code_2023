#[cfg(feature = "part2")]
use num::integer::lcm;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Pulse {
  #[default]
  None,
  Low,
  High,
}

pub trait Signaler {
  fn send_signal(&self);
}

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Signal {
  pub from: Option<Address>,
  pub pulse: Pulse,
  pub to: Address,
}

pub type Address = String;

pub type State = u64;

#[cfg(feature = "part2")]
pub fn get_lcm<T: AsRef<[usize]>>(numbers: T) -> Option<usize> {
  let slice = numbers.as_ref();

  if slice.is_empty() {
    return None;
  }

  let mut result = slice[0];

  for &number in slice[1..].iter() {
    result = lcm(result, number);
  }

  Some(result)
}
