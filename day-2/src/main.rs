use std::{cmp, str::FromStr};


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

#[derive(PartialEq, PartialOrd, Clone, Debug, Default)]
struct Set {
  red: usize,
  green: usize,
  blue: usize,
}

impl Set {
  pub fn from_str(s: &str) -> Result<Set, String> {
    let parts = s.trim().splitn(3, ", ").collect::<Vec<_>>();

    if parts.is_empty() {
      return Err(format!(
        "[Set::from_str] Invalid set (not [only] distinct colors) in '{s}'"
      ));
    }

    let mut set = Set::default();

    for color_spec in parts {
      let (value, color) = Set::get_color_and_value(color_spec)
        .map_err(|e| format!("{e}\n source string for set: {color_spec}"))?;

      match color.as_str() {
        "red" => set.red = value,
        "green" => set.green = value,
        "blue" => set.blue = value,
        _ => {
          return Err(format!(
            "[Set::from_str] Invalid set (unknown color) in '{s}'"
          ))
        }
      }
    }

    Ok(set)
  }

  fn get_color_and_value(spec: &str) -> Result<(usize, String), String> {
    let mut iter = spec.split(' ');
    let Some(value_str) = iter.next() else {
      return Err(format!(
        "[Set::from_str] Invalid set (no color value) in '{spec}'"
      ));
    };
    let value: usize = match value_str.parse() {
      Ok(value) => value,
      Err(e) => {
        return Err(format![
          "[Set::from_str] Invalid set (no color value) in '{spec}'. Parse \
           error:\n{e}"
        ]);
      }
    };
    let Some(color) = iter.next() else {
      return Err(format!(
        "[Set::from_str] Invalid set (no colo term) in '{spec}'"
      ));
    };
    if iter.peekable().peek().is_some() {
      return Err(format!(
        "[Set::from_str] Invalid set (additional data in color) in '{spec}'"
      ));
    }

    Ok((value, color.to_string()))
  }

  pub fn is_ge_strict(&self, other: &Self) -> bool {
    self.red > other.red || self.green > other.green || self.blue > other.blue
  }
}

const CONDITIONS: Set = Set { red: 12, green: 13, blue: 14 };

#[derive(PartialEq, Debug)]
struct Game {
  id: usize,
  sets: Vec<Set>,
}

impl Clone for Game {
  fn clone(&self) -> Self {
    Self { id: self.id, sets: self.sets.clone() }
  }
}

impl Ord for Game {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.id.cmp(&other.id)
  }
}
impl PartialOrd for Game {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Eq for Game {}

impl Game {
  fn get_id_from_string(s: &str) -> Result<usize, String> {
    // Assuming the format is "Game n:" where n is a number
    let id_parts: Vec<&str> = s.split_whitespace().collect();
    if id_parts.len() != 2 {
      return Err("Invalid game ID".to_string());
    }

    match id_parts[1].parse::<usize>() {
      Ok(id) => Ok(id),
      Err(_) => Err("Invalid game ID".to_string()),
    }
  }

  fn get_sets_from_string(s: &str) -> Result<Vec<Set>, String> {
    s.split(';')
      .map(|set_str| {
        Set::from_str(set_str).map_err(|e| format!["Invalid set: {}", e])
      })
      .collect()
  }
}

impl FromStr for Game {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts: Vec<&str> = s.splitn(2, ':').collect();
    if parts.len() != 2 {
      return Err("Invalid game string".to_string());
    }

    let id = Game::get_id_from_string(parts[0])?;
    let sets: Vec<Set> = Game::get_sets_from_string(parts[1])?;

    Ok(Game { id, sets })
  }
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

fn extract() -> Result<Vec<Game>, String> {
  src_provider()?
    .lines()
    .map(Game::from_str)
    .collect::<Result<Vec<_>, _>>()
}

fn transform(data: Vec<Game>) -> Result<Vec<usize>, String> {
  Ok(
    data
      .iter()
      .filter_map(|game| {
        if game.sets.iter().any(|set| set.is_ge_strict(&CONDITIONS)) {
          return None;
        }

        Some(game.id)
      })
      .collect(),
  )
}

fn load(result: Result<Vec<usize>, String>) -> Result<(), String> {
  match result {
    Ok(indices) => println!("result: {}", get_index_sum(&indices)?),
    Err(msg) => println!("{msg}"),
  }

  Ok(())
}

fn get_index_sum(indices: &[usize]) -> Result<usize, String> {
  Ok(indices.iter().sum::<usize>())
}


#[cfg(test)]
mod tests {
  use super::*;

  // MARK extract
  #[test]
  #[mry::lock(src_provider)]
  fn it_should_extract() {
    // Test that the extract function correctly reads from a string and converts it to a vector of Game structs.

    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame \
                 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
    mock_src_provider().returns(Ok(input.to_string()));

    let mut expected = vec![
      Game {
        id: 1,
        sets: vec![
          Set { red: 4, green: 0, blue: 3 },
          Set { red: 1, green: 2, blue: 6 },
          Set { red: 0, green: 2, blue: 0 },
        ],
      },
      Game {
        id: 2,
        sets: vec![
          Set { red: 0, green: 2, blue: 1 },
          Set { red: 1, green: 3, blue: 4 },
          Set { red: 0, green: 1, blue: 1 },
        ],
      },
    ];

    let mut result = extract().unwrap();
    result.sort_unstable();
    expected.sort_unstable();
    assert_eq!(&result, &expected);
  }

  // MARK transform
  #[test]
  fn it_should_transform() {
    // Test that the transform function correctly filters a vector of Game structs based on a condition.

    let input = vec![
      Game {
        id: 1,
        sets: vec![
          Set { red: 4, green: 0, blue: 3 },
          Set { red: 1, green: 2, blue: 16 },
          Set { red: 0, green: 2, blue: 0 },
        ],
      },
      Game {
        id: 2,
        sets: vec![
          Set { red: 0, green: 2, blue: 1 },
          Set { red: 1, green: 3, blue: 4 },
          Set { red: 0, green: 1, blue: 1 },
        ],
      },
    ];

    let mut expected = vec![2]; // Game with id 2 is the only one that meets the condition.

    let mut result = transform(input).unwrap();
    result.sort_unstable();
    expected.sort_unstable();
    assert_eq!(&result, &expected);
  }

  // MARK load
  #[test]
  fn it_should_get_index_sum() {
    // Test that the load function correctly prints the sum of a vector of usize.

    let input = vec![2, 3, 4];
    let expected = 9;

    let result = get_index_sum(&input);
    assert!(result.is_ok_and(|suma| suma == expected));
  }
}
