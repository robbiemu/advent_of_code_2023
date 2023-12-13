use std::collections::HashMap;


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

type ProblemDefinition = Vec<(Vec<u8>, Vec<usize>)>;
type Consequent = Vec<usize>;

fn main() -> Result<(), String> {
  let data = extract()?;
  let result = transform(data);

  load(result)
}

#[mry::mry]
fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}

fn extract() -> Result<ProblemDefinition, String> {
  let mut problems = Vec::new();
  for input_string in src_provider()?.lines() {
    let (prefix, suffix) = input_string
      .split_once(' ')
      .ok_or(format!("[extract] invalid input (no space) {input_string}"))?;
    let points = prefix.as_bytes().to_vec();
    let mut constraints = Vec::new();
    for n in suffix.split(',') {
      let constraint = n.parse::<usize>().map_err(|e| {
        format!(
          "[extract] invalid input ('{n}' not a natural number) in: \
           {input_string}\nError:\n{e}"
        )
      })?;
      constraints.push(constraint);
    }
    problems.push((points, constraints));
  }

  Ok(problems)
}

/* solve - create a cache and prefix sum of elements up to known counts of
broken springs and start a recursive search */
fn solve(points: &[u8], constraints: &[usize]) -> usize {
  let mut springs = Vec::new();
  let mut consecutive_broken = Vec::new();
  #[cfg(feature = "part2")]
  for _ in 0..4 {
    springs.extend_from_slice(points);
    springs.push(b'?');
    consecutive_broken.extend_from_slice(constraints);
  }

  springs.extend(points);
  springs.push(b'.');

  consecutive_broken.extend(constraints);

  /* calculate the sum of all elements from index `0` up to `i` in
  consecutive_broken. */
  let mut sum = 0;
  let mut ps = vec![0; consecutive_broken.len()];
  for i in (1..consecutive_broken.len()).rev() {
    sum += consecutive_broken[i] + 1;
    ps[i - 1] = sum;
  }

  let mut cache = HashMap::new();

  recurse(&springs, &consecutive_broken, &ps, &mut cache)
}

/* recurse - checks all possible positions from which a new segment can start.
If this position does not have any broken elements or the next element is
marked as working ('.'), then it makes a recursive call for the remaining string
and counts (skipping the current count). */
fn recurse(
  springs: &[u8],
  consecutive_broken: &[usize],
  ps: &[usize],
  cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
  /* Create a key for this specific slice and counts. This will be used to store
  and retrieve computed results in the cache map. */
  let key = (springs.len(), consecutive_broken.len());
  // Check if we've already calculated the result for this combination of
  // springs and consecutive_broken, i.e., return it from cache if available.
  if let Some(prev) = cache.get(&key) {
    return *prev;
  }

  /* If there are no counts left in 'consecutive_broken', check if all elements
  in slice are working. If so it was satisfied. Cache success or failure and
  return result */
  if consecutive_broken.is_empty() {
    let result = springs.iter().all(|&b| b == b'.' || b == b'?') as usize;
    cache.insert(key, result);
    return result;
  }

  // 'size' is the current count of consecutive broken elements to look for.
  let size = consecutive_broken[0];
  /* The wiggle room is the maximum position we can slide to next in 'slice',
  considering the remaining counts in 'consecutive_broken'. */
  let wiggle = springs.len() - ps[0] - size;
  let mut result = 0;

  for offset in 0..wiggle {
    // if we have reached a '#' then we break because the next segment covers it
    if offset > 0 && springs[offset - 1] == b'#' {
      break;
    }
    /* If the next 'size' elements after 'offset' are broken or '?' and the
    following element is not, then we have found our match */
    if springs[offset + size] != b'#'
      && springs[offset..offset + size]
        .iter()
        .all(|&b| b == b'#' || b == b'?')
    {
      result += recurse(
        &springs[offset + size + 1..],
        &consecutive_broken[1..],
        &ps[1..],
        cache,
      );
    }
  }

  cache.insert(key, result);
  result
}

fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
  Ok(
    data
      .iter()
      .map(|(points, constraints)| solve(points, constraints))
      .collect(),
  )
}

fn load(result: Result<Consequent, String>) -> Result<(), String> {
  match result {
    Ok(items) => {
      println!(
        "{} sum of possible satsifying allocations",
        items.iter().sum::<usize>()
      );
    }
    Err(e) => eprintln!("{e}"),
  }

  Ok(())
}
