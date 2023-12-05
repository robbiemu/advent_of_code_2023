use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{space1, u32 as u32_parser};
use nom::multi::separated_list0;
use nom::sequence::{pair, tuple};
use nom::IResult;
#[allow(unused_imports)]
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

type RecordValue = (Vec<u32>, Vec<u32>);
type RecordEntry = (u32, RecordValue);

fn main() -> Result<(), String> {
  let data = extract()?;
  let result = transform(data);

  load(result)
}

#[mry::mry]
fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}

fn parse_line(input: &str) -> IResult<&str, RecordEntry> {
  let mut parser = pair(
    pair(tag("Card"), space1),
    tuple((
      u32_parser,
      alt((tag(":  "), tag(": "))),
      separated_list0(space1, u32_parser),
      alt((tag(" |  "), tag(" | "))),
      separated_list0(space1, u32_parser),
    )),
  );

  let (remaining, result) = parser(input)?;
  let (_, (id, _, left, _, right)) = result;
  Ok((remaining, (id, (left, right))))
}

fn extract() -> Result<HashMap<u32, RecordValue>, String> {
  let mut map: HashMap<u32, (Vec<u32>, Vec<u32>)> = HashMap::new();

  for line in src_provider()?.lines() {
    match parse_line(line) {
      Ok((_, (id, (left, right)))) => {
        map.insert(id, (left, right));
      }
      Err(err) => {
        return Err(format!(
          "Failed to parse line: {}. Error: {:?}",
          line, err
        ));
      }
    }
  }

  Ok(map)
}

fn transform(data: HashMap<u32, RecordValue>) -> Result<Vec<usize>, String> {
  #[cfg(not(feature = "part2"))]
  {
    Ok(
      data
        .values()
        .map(|(left, right)| {
          let cnt = left.iter().filter(|&n| right.contains(n)).count() as u32;
          if cnt == 0 {
            return 0;
          }

          2_usize.pow(cnt - 1)
        })
        .collect(),
    )
  }
  #[cfg(feature = "part2")]
  {
    let mut sorted_keys: Vec<u32> = data.keys().cloned().collect();
    sorted_keys.sort(); // Sort the keys by id

    Ok(
      sorted_keys
        .iter()
        .fold(HashMap::<u32, usize>::new(), |mut acc, id| {
          let (left, right) = data.get(id).unwrap();
          let cnt = left.iter().filter(|&n| right.contains(n)).count() as u32;
          if !acc.contains_key(id) {
            acc.insert(*id, 1);
          }
          let times = *acc.get(id).unwrap();
          (id + 1..id + cnt + 1).for_each(|k| {
            if let Vacant(e) = acc.entry(k) {
              e.insert(times + 1);
            } else {
              acc.entry(k).and_modify(|e| *e += times).or_insert(times);
            }
          });

          acc
        })
        .values()
        .cloned()
        .collect::<Vec<_>>(),
    )
  }
}

fn load(result: Result<Vec<usize>, String>) -> Result<(), String> {
  match result {
    Ok(values) => println!("{}", values.iter().sum::<usize>()),
    Err(e) => eprintln!("{e}"),
  }

  Ok(())
}


#[cfg(test)]
mod tests {
  // use super::*;

  // MARK extract

  // MARK transform

  // MARK load
}
