#[cfg(feature = "sample")]
#[cfg(not(feature = "part2"))]
const DATA: &str = include_str!("../sample.txt");
#[cfg(all(feature = "sample", feature = "part2"))]
const DATA: &str = include_str!("../sample-part2.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

#[allow(dead_code)]
const WORDS: &[&str; 9] = &[
  "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() -> Result<(), String> {
  let data = extract()?;
  let result = transform(data);

  load(result)
}

#[mry::mry]
fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}

fn extract() -> Result<Vec<Vec<char>>, String> {
  Ok(
    src_provider()?
      .lines()
      .map(|line: &str| line.chars().collect::<Vec<_>>())
      .collect::<Vec<_>>(),
  )
}

#[allow(unused_mut)]
fn transform(mut data: Vec<Vec<char>>) -> Result<usize, String> {
  #[cfg(feature = "part2")]
  {
    data = part2(&data)?;
  }

  let mut sum = 0;

  for line in data {
    let Some(num) = process_line(&line) else {
      return Err(format!(
        "Could not produce a string from '{}'",
        line.iter().collect::<String>()
      ));
    };
    sum += num;
  }

  Ok(sum)
}

#[cfg(feature = "part2")]
fn part2(data: &[Vec<char>]) -> Result<Vec<Vec<char>>, String> {
  let recomposed: Vec<String> = data
    .iter()
    .map(|chars| {
      let mut line = chars.iter().collect::<String>();
      WORDS.iter().enumerate().for_each(|(i, word)| {
        line = line.replace(word, &format!("{word}{}{word}", (i + 1)))
      });

      line
    })
    .collect();

  let new_data: Vec<Vec<char>> =
    recomposed.iter().map(|s| s.chars().collect()).collect();

  Ok(new_data)
}

fn process_line(chars: &[char]) -> Option<usize> {
  let mut left_index = 0;
  let mut right_index = chars.len().saturating_sub(1);

  while left_index <= right_index {
    if let (Some(left), Some(right)) =
      (chars.get(left_index), chars.get(right_index))
    {
      if left.is_numeric() && right.is_numeric() {
        if let Ok(num) = format!("{}{}", left, right).parse::<usize>() {
          return Some(num);
        } else {
          unreachable!()
        }
      }

      if !left.is_numeric() {
        left_index += 1;
      }
      if !right.is_numeric() {
        right_index -= 1;
      }
    }
  }

  None
}

fn load(result: Result<usize, String>) -> Result<(), String> {
  match result {
    Ok(result) => println!("result: {result}"),
    Err(msg) => println!("{msg}"),
  }

  Ok(())
}


#[cfg(test)]
mod tests {
  use super::*;

  // MARK extract
  #[test]
  #[mry::lock(src_provider)]
  fn it_should_extract_a_file_into_a_vector_of_strings() {
    mock_src_provider().returns(Ok("a test\nmultiline".to_string()));

    let contents = extract();
    assert!(contents.is_ok());
    let result = contents.ok().unwrap();
    assert_eq!(result.first(), Some(&"a test".chars().collect::<Vec<_>>()));
    assert_eq!(
      result.last(),
      Some(&"multiline".chars().collect::<Vec<_>>())
    );
  }

  // MARK transform
  #[test]
  fn it_should_sum_encountered_numbers_in_input() {
    let data = "a 1test\nmu2ltil3ine"
      .to_string()
      .split('\n')
      .map(|s: &str| s.chars().collect::<Vec<_>>())
      .collect::<Vec<_>>();


    let result = transform(data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 34);
  }

  #[test]
  fn it_should_return_an_error_when_no_number_encountered_in_input() {
    let data = "a 1test\nno number\nmu2ltil3ine"
      .to_string()
      .split('\n')
      .map(|s: &str| s.chars().collect::<Vec<_>>())
      .collect::<Vec<_>>();


    let result = transform(data);
    assert!(result.is_err());
  }

  // MARK load
}
