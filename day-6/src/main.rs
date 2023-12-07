#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");


fn main() -> Result<(), String> {
  let data = extract()?;
  let result = transform(data);

  load(result)
}

#[mry::mry]
fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}

fn parse_input(input: String) -> Result<Vec<(i64, i64)>, String> {
  let lines: Vec<&str> = input.lines().collect();

  if lines.len() != 2 {
    return Err("Invalid input".to_string());
  }

  let times: Vec<i64>;
  let distances: Vec<i64>;
  #[cfg(not(feature = "part2"))]
  {
    times = lines[0]
      .split_whitespace()
      .skip(1)
      .map(|s| s.parse().unwrap())
      .collect();
    distances = lines[1]
      .split_whitespace()
      .skip(1)
      .map(|s| s.parse().unwrap())
      .collect();
  }
  #[cfg(feature = "part2")]
  {
    let parts: Vec<&str> = lines[0].splitn(2, char::is_whitespace).collect();
    times = vec![parts[1]
      .chars()
      .filter(|&c| !c.is_whitespace())
      .collect::<String>()
      .parse::<i64>()
      .unwrap()];

    let parts: Vec<&str> = lines[1].splitn(2, char::is_whitespace).collect();
    distances = vec![parts[1]
      .chars()
      .filter(|&c| !c.is_whitespace())
      .collect::<String>()
      .parse::<i64>()
      .unwrap()];

    dbg!(&distances);
  }

  Ok(times.into_iter().zip(distances).collect())
}

fn extract() -> Result<Vec<(i64, i64)>, String> {
  parse_input(src_provider()?)
}

fn find_time_to_threshold(
  total_time: f64,
  threshold: f64,
) -> Result<Vec<isize>, String> {
  // find the open interval above threshold
  let p1 =
    total_time / 2.0 - ((total_time.powi(2) - 4.0 * threshold).sqrt()) / 2.0;
  let p2 =
    total_time / 2.0 + ((total_time.powi(2) - 4.0 * threshold).sqrt()) / 2.0;

  // bound to largest and smallest integer
  let lower_bound = (p1 + 1.0).floor() as isize;
  let upper_bound = (p2 - 1.0).ceil() as isize;

  // Populate solutions within the open interval
  Ok((lower_bound..=upper_bound).collect::<Vec<isize>>())
}

fn transform(data: Vec<(i64, i64)>) -> Result<Vec<Vec<isize>>, String> {
  let mut result = vec![];

  for (time, threshold) in data {
    match find_time_to_threshold(time as f64, threshold as f64) {
      Ok(solutions) => result.push(solutions),
      Err(e) => return Err(e),
    }
  }

  Ok(result)
}

fn parse_result(findings: Vec<Vec<isize>>) -> isize {
  findings
    .iter()
    .fold(1, |acc, solutions| solutions.len() as isize * acc)
}

fn load(result: Result<Vec<Vec<isize>>, String>) -> Result<(), String> {
  match result {
    Ok(findings) => println!("{}", parse_result(findings)),
    Err(err) => eprintln!("{err}"),
  }

  Ok(())
}
