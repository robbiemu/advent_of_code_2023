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

fn extract() -> Result<Vec<Vec<char>>, String> {
  Ok(
    src_provider()?
      .split('\n')
      .map(|s: &str| s.chars().collect::<Vec<_>>())
      .collect::<Vec<_>>(),
  )
}

fn transform(data: Vec<Vec<char>>) -> Result<usize, String> {
  let mut sum = 0;

  for line in data {
    let mut left_index = 0;
    let mut right_index = line.len().saturating_sub(1);

    while left_index <= right_index {
      if let (Some(left), Some(right)) =
        (line.get(left_index), line.get(right_index))
      {
        if left.is_numeric() && right.is_numeric() {
          if let Ok(num) = format!("{}{}", left, right).parse::<usize>() {
            sum += num;
            break;
          } else {
            unreachable!()
          }
        }
      }

      if !line.get(left_index).map_or(false, |c| c.is_numeric()) {
        left_index += 1;
      }
      if !line.get(right_index).map_or(false, |c| c.is_numeric()) {
        right_index -= 1;
      }
    }

    if left_index > right_index {
      return Err(format!(
        "Could not produce a string from '{}'",
        line.iter().collect::<String>()
      ));
    }
  }

  Ok(sum)
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
