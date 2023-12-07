use std::{cmp::Ordering, collections::BTreeMap};


#[cfg(feature = "sample")]
#[cfg(not(feature = "part2"))]
const DATA: &str = include_str!("../sample.txt");
#[cfg(all(feature = "sample", feature = "part2"))]
const DATA: &str = include_str!("../sample-part2.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Card {
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Jack,
  Queen,
  King,
  Ace,
}

impl Card {
  fn from_char(card: char) -> Option<Card> {
    match card {
      '2' => Some(Card::Two),
      '3' => Some(Card::Three),
      '4' => Some(Card::Four),
      '5' => Some(Card::Five),
      '6' => Some(Card::Six),
      '7' => Some(Card::Seven),
      '8' => Some(Card::Eight),
      '9' => Some(Card::Nine),
      'T' => Some(Card::Ten),
      'J' => Some(Card::Jack),
      'Q' => Some(Card::Queen),
      'K' => Some(Card::King),
      'A' => Some(Card::Ace),
      _ => None,
    }
  }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
  HighCard,
  OnePair,
  TwoPair,
  ThreeOfAKind,
  FullHouse,
  FourOfAKind,
  FiveOfAKind,
}

impl HandType {
  fn from(hand: &Hand) -> Option<HandType> {
    let count = count_elements(hand);
    match count.len() {
      1 => Some(HandType::FiveOfAKind),
      2 => {
        if *count.values().max().unwrap_or(&0) == 4 {
          Some(HandType::FourOfAKind)
        } else {
          Some(HandType::FullHouse)
        }
      }
      3 => {
        if *count.values().max().unwrap_or(&0) == 3 {
          Some(HandType::ThreeOfAKind)
        } else {
          Some(HandType::TwoPair)
        }
      }
      4 => Some(HandType::OnePair),
      5 => Some(HandType::HighCard),
      _ => unreachable!(),
    }
  }
}

type Hand = [Card; 5];

#[derive(Debug, PartialEq, Eq)]
struct Seat {
  hand: Hand,
  bid: usize,
}

impl Seat {
  pub fn from(hand_str: &str, bid_str: &str) -> Result<Self, String> {
    let hand: Result<Vec<_>, _> = hand_str
      .chars()
      .map(|c| Card::from_char(c).ok_or(format!("Invalid card: {}", c)))
      .collect();

    let hand = hand.and_then(|cards| {
      cards
        .try_into()
        .map_err(|_| "Invalid number of cards in hand".to_string())
    })?;

    let bid: usize = bid_str
      .parse()
      .map_err(|_| "Failed to parse bid".to_string())?;

    Ok(Self { hand, bid })
  }
}

impl PartialOrd for Seat {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Seat {
  fn cmp(&self, other: &Self) -> Ordering {
    // First, compare by hand type
    let left = HandType::from(&self.hand).unwrap();
    let right = HandType::from(&other.hand).unwrap();
    let type_ordering = left.cmp(&right);

    if type_ordering != Ordering::Equal {
      return type_ordering;
    }

    // Then, by cards in left-to-right-order
    for (self_card, other_card) in self.hand.iter().zip(other.hand.iter()) {
      let card_ordering = self_card.cmp(other_card);
      if card_ordering != Ordering::Equal {
        return card_ordering;
      }
    }

    Ordering::Equal
  }
}

fn count_elements<T: Eq + Ord + std::hash::Hash>(
  iter: impl IntoIterator<Item = T>,
) -> BTreeMap<T, usize> {
  let mut counts = BTreeMap::new();
  for item in iter.into_iter() {
    *counts.entry(item).or_insert(0) += 1;
  }
  counts
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

fn extract() -> Result<Vec<Seat>, String> {
  let result: Result<Vec<_>, _> = src_provider()?
    .lines()
    .map(|line| {
      line
        .split_once(' ')
        .ok_or("Invalid line format".to_string())
        .and_then(|(hand_str, bid_str)| {
          Seat::from(hand_str, bid_str).map_err(|e| {
            format!(
              "Failed to create seat from card: '{hand_str} {bid_str}': {e}"
            )
          })
        })
    })
    .collect();

  result
}

fn transform(mut data: Vec<Seat>) -> Result<Vec<usize>, String> {
  data.sort_unstable();
  Ok(
    data
      .iter()
      .enumerate()
      .map(|(rank, seat)| seat.bid * (rank + 1))
      .collect(),
  )
}


fn load(result: Result<Vec<usize>, String>) -> Result<(), String> {
  match result {
    Ok(winnings) => println!("{}", winnings.iter().sum::<usize>()),
    Err(e) => eprintln!("{:?}", e),
  };

  Ok(())
}
