#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

fn main() -> Result<(), String> {
  let data = extract()?;
  let result = transform(&data);

  load(result)
}

#[mry::mry]
fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}

fn extract() -> Result<Vec<Vec<isize>>, String> {
  let mut result = Vec::new();

  for line in src_provider()?.lines() {
    let numbers: Result<_, _> =
      line.split_whitespace().map(|s| s.parse()).collect();

    match numbers {
      Ok(ns) => result.push(ns),
      Err(_e) => return Err("Parsing failed".to_string()),
    }
  }

  Ok(result)
}

fn transform(data: &[Vec<isize>]) -> Result<Vec<isize>, String> {
  Ok(
    data
      .iter()
      .map(|time_series| {
        #[cfg(not(feature = "part2"))]
        let mut stack: Vec<Vec<isize>> = vec![time_series.to_vec()];
        #[cfg(feature = "part2")]
        let mut stack: Vec<Vec<isize>> =
          vec![time_series.iter().rev().cloned().collect::<Vec<_>>()];

        while !stack.last().unwrap().iter().all(|&x| x == 0) {
          let series = stack.last().unwrap();
          let differences: Vec<isize> =
            series.windows(2).map(|w| w[1] - w[0]).collect();
          stack.push(differences);
        }
        // dbg!(stack
        //   .iter()
        //   .rev()
        //   .map(|l| l
        //     .iter()
        //     .map(|i| i.to_string())
        //     .collect::<Vec<_>>()
        //     .join(" "))
        //   .collect::<Vec<_>>());

        let mut prev_diff = 0;
        for i in (0..stack.len()).rev() {
          let line = stack.get(i).unwrap();
          let last = line.last().unwrap();
          prev_diff += last;
        }

        prev_diff
      })
      .collect(),
  )
}

fn load(result: Result<Vec<isize>, String>) -> Result<(), String> {
  match result {
    Ok(values) => println!("{}", values.iter().sum::<isize>()),
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
