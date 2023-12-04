use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{space1, u32 as u32_parser};
use nom::multi::separated_list0;
use nom::sequence::{pair, tuple};
use nom::IResult;
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
        map.entry(id).or_insert((vec![], vec![])).0.extend(left);
        map.entry(id).or_insert((vec![], vec![])).1.extend(right);
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
