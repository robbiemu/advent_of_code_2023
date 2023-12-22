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
