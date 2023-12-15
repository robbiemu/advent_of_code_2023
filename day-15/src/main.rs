#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

type ProblemDefinition = Vec<String>;
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
  Ok(
    src_provider()?
      .split(',')
      .map(|word| word.chars().filter(|c| c != &'\n').collect::<String>())
      .collect::<Vec<String>>(),
  )
}

/*
To run the HASH algorithm on a string, start with a current value of 0. Then, for each character in the string starting from the beginning:
- Determine the ASCII code for the current character of the string.
- Increase the current value by the ASCII code you just determined.
- Set the current value to itself multiplied by 17.
- Set the current value to the remainder of dividing itself by 256.
*/
fn get_hash(string: &str) -> usize {
  string.chars().fold(0, |mut acc, c| {
    acc += c as usize;
    acc *= 17;
    acc %= 256;
    dbg!((c, c as usize, acc));

    acc
  })
}

fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
  Ok(data.iter().map(|word| get_hash(word)).collect())
}

fn load(result: Result<Consequent, String>) -> Result<(), String> {
  match result {
    Ok(values) => {
      dbg!(&values);
      println!("sum {}", values.iter().sum::<usize>());
    }
    Err(err) => eprintln!("{err}"),
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
