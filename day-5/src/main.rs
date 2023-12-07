use nom::{
  bytes::complete::{tag, take_while1},
  character::complete::{self, space1},
  combinator::{map, opt},
  multi::{many1, separated_list0, separated_list1},
  sequence::{separated_pair, terminated, tuple},
  IResult,
};
use std::{
  collections::{HashMap, HashSet},
  ops::Range,
};


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

const SEED: &str = "seed";
const LOCATION: &str = "location";

type Record = ((String, String), Vec<(usize, usize, usize)>);

type Destination = Range<usize>;
type Source = Range<usize>;
type Transformation = (Destination, Source);

#[cfg(not(feature = "part2"))]
type Seeds = HashSet<usize>;
#[cfg(feature = "part2")]
type Seeds = HashSet<(usize, usize)>;

struct ProblemDefinition {
  seeds: Seeds,
  transformations: HashMap<(String, String), Vec<Transformation>>,
}

impl ProblemDefinition {
  fn recurse_transformations(
    &self,
    from_value: usize,
    stop_at: &str,
    from: &str,
  ) -> usize {
    eprintln!("{from_value}: {from} -> {stop_at}");
    if from == stop_at {
      return from_value;
    }

    for (mapping, transformations) in &self.transformations {
      if mapping.0 == from {
        if let Some(transformation) =
          transformations.iter().find(|t| t.1.contains(&from_value))
        {
          let new_seed =
            transformation.0.start + (from_value - transformation.1.start);
          return self.recurse_transformations(new_seed, stop_at, &mapping.1);
        }

        return self.recurse_transformations(from_value, stop_at, &mapping.1);
      }
    }

    unreachable!()
  }

  fn get_transformation(
    dest: usize,
    src: usize,
    range_length: usize,
  ) -> Transformation {
    let destination = dest..dest + range_length;
    let source = src..src + range_length;

    (destination, source)
  }

  fn from(seeds: Seeds, records: &[Record]) -> Self {
    let transformations =
      records
        .iter()
        .fold(HashMap::new(), |mut acc, (k, records)| {
          let v: Vec<Transformation> = records
            .iter()
            .map(|(dest, src, range_length)| {
              ProblemDefinition::get_transformation(*dest, *src, *range_length)
            })
            .collect();
          acc.insert(k.to_owned(), v);

          acc
        });

    Self { seeds, transformations }
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

fn parse_usize(input: &str) -> IResult<&str, usize> {
  map(complete::u32, |n| n as usize)(input)
}

pub fn parse_seeds(input: &str) -> IResult<&str, Seeds> {
  let (input, (_tag, numbers)) = terminated(
    separated_pair(
      tag("seeds:"),
      space1,
      map(separated_list0(space1, parse_usize), |n| {
        n.into_iter().collect::<HashSet<_>>()
      }),
    ),
    tuple((tag("\n"), tag("\n"))),
  )(input)?;


  Ok((input, numbers))
}

pub fn parse_record(input: &str) -> IResult<&str, Record> {
  let (input, ((mapping, _tag), numbers)) = terminated(
    separated_pair(
      separated_pair(
        map(
          tuple((
            take_while1(|c| c != '-'),
            tag("-to-"),
            take_while1(|c: char| !c.is_whitespace()),
          )),
          |(src, _tag, dest): (&str, &str, &str)| {
            (src.to_string(), dest.to_string())
          },
        ),
        space1,
        tag("map:"),
      ),
      tag("\n"),
      separated_list1(
        tag("\n"),
        map(
          tuple((parse_usize, space1, parse_usize, space1, parse_usize)),
          |(dest, _, src, _, range_length)| (dest, src, range_length),
        ),
      ),
    ),
    tuple((opt(tag("\n")), opt(tag("\n")))), // Expecting two newline sequences after seeds
  )(input)?;

  Ok((input, (mapping, numbers)))
}

pub fn parse_records(input: &str) -> IResult<&str, Vec<Record>> {
  many1(parse_record)(input)
}

pub fn parse_data(input: &str) -> IResult<&str, (Seeds, Vec<Record>)> {
  // Parses seeds followed by multiple records
  let (input, seeds) = parse_seeds(input)?;
  let (input, records) = parse_records(input)?;

  Ok((input, (seeds, records)))
}

fn extract() -> Result<ProblemDefinition, String> {
  let (_, (seeds, records)) = parse_data(&src_provider()?)
    .map_err(|e| format!("Failed to parse input. Error: {:?}", e))?;

  // dbg!(&seeds);

  Ok(ProblemDefinition::from(seeds, &records))
}

fn transform(problem: ProblemDefinition) -> Result<Vec<usize>, String> {
  let mut locations: HashMap<usize, usize> = HashMap::new();
  #[cfg(not(feature = "part2"))]
  {
    for seed in &problem.seeds {
      let location = problem.recurse_transformations(*seed, LOCATION, SEED);
      locations.insert(*seed, location);
    }

    return Ok(locations.values().cloned().collect::<Vec<_>>());
  }
  #[cfg(feature = "part2")]
  {
    for seed_range in &problem.seeds {
      for seed in seed_range.1..seed_range.0 {
        let location = problem.recurse_transformations(seed, LOCATION, SEED);
        locations.insert(seed, location);
      }
    }

    return Ok(locations.values().cloned().collect::<Vec<_>>());
  }
}


fn load(result: Result<Vec<usize>, String>) -> Result<(), String> {
  match result {
    Ok(values) => println!("{}", values.iter().min().unwrap()),
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
