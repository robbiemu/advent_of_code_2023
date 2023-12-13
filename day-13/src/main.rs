#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

struct ProblemDefinition {
  patterns: Vec<Reflection>,
}
type Reflection = Vec<Vec<u8>>;
type Consequent = Vec<(usize, Symmetry)>;

#[derive(Debug)]
enum Symmetry {
  Horizontal,
  Vertical,
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

fn extract() -> Result<ProblemDefinition, String> {
  let patterns = src_provider()?
    .split("\n\n")
    .map(|reflection| {
      reflection
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect()
    })
    .collect();

  Ok(ProblemDefinition { patterns })
}

fn get_vertical_inflection_point(matrix: &[Vec<u8>]) -> Option<usize> {
  let ncols = matrix[0].len();

  (0..=ncols - 2).find(|&i| {
    let j = i + 1;
    let mut symmetry_found = true;

    for k in 0..=i {
      if j + k >= ncols {
        break;
      }
      if matrix.iter().any(|row| row[i - k] != row[j + k]) {
        symmetry_found = false;
        break;
      }
    }

    symmetry_found
  })
}

fn get_horizontal_inflection_point(matrix: &[Vec<u8>]) -> Option<usize> {
  let nrows = matrix.len();

  (0..=nrows - 2).find(|&i| {
    let j = i + 1;
    let mut symmetry_found = true;

    for k in 0..=i {
      if j + k >= nrows {
        break;
      }
      if matrix[i - k]
        .iter()
        .zip(&matrix[j + k])
        .any(|(a, b)| a != b)
      {
        symmetry_found = false;
        break;
      }
    }

    symmetry_found
  })
}


fn get_inflection(matrix: &[Vec<u8>]) -> Option<(usize, Symmetry)> {
  if let Some(point) = get_horizontal_inflection_point(matrix) {
    return Some((point, Symmetry::Horizontal));
  }
  if let Some(point) = get_vertical_inflection_point(matrix) {
    return Some((point, Symmetry::Vertical));
  }

  None
}

fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
  let mut results = Vec::new();
  for reflection in data.patterns {
    let Some((inflection_point, symmetry)) = get_inflection(&reflection) else {
      return Err(format!("[transform] no symmetry found in {:?}", reflection));
    };
    results.push((inflection_point, symmetry));
  }


  dbg!(&results);

  Ok(results)
}

fn summarize(items: Consequent) -> usize {
  items.iter().fold(0, |acc, (p, symmetry)| match symmetry {
    Symmetry::Horizontal => acc + 100 * (p + 1),
    Symmetry::Vertical => acc + p + 1,
  })
}

fn load(result: Result<Consequent, String>) -> Result<(), String> {
  match result {
    Ok(items) => {
      println!("summary: {}", summarize(items));
    }
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
