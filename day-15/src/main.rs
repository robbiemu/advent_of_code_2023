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

    acc
  })
}

/*
The focusing power of a single lens is the result of multiplying together:
- One plus the box number of the lens in question.
- The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
- The focal length of the lens.
*/
fn get_focal_power(
  box_number: usize,
  slot_number: usize,
  focal_length: usize,
) -> usize {
  box_number * slot_number * focal_length
}

enum Operation {
  Add((String, u8)),
  Subtract(String),
}

/* The label will be immediately followed by a character that indicates the operation to perform: either an equals sign (=) or a dash (-). */
fn get_operation(word: &str) -> Result<Operation, String> {
  if word.contains('=') {
    let (label, f) = word.split_once('=').unwrap();
    let Ok(focal_length) = f.parse::<u8>() else {
      return Err(format!("Invalid focal length: {f}"));
    };

    Ok(Operation::Add((label.to_string(), focal_length)))
  } else if word.contains('-') {
    Ok(Operation::Subtract(
      word.split_once('-').unwrap().0.to_string(),
    ))
  } else {
    Err(format!("Invalid operation: {word}"))
  }
}


fn transform(data: ProblemDefinition) -> Result<Consequent, String> {
  #[cfg(not(feature = "part2"))]
  {
    Ok(data.iter().map(|word| get_hash(word)).collect())
  }
  #[cfg(feature = "part2")]
  {
    let mut boxes = vec![Vec::<(String, u8)>::new(); 256];
    for word in data {
      let result = get_operation(&word);
      match result {
        Ok(Operation::Add((label, focal_length))) => {
          let hash = get_hash(&label);
          if let Some(i) = boxes[hash].iter().position(|(l, _)| *l == label) {
            boxes[hash][i] = (label, focal_length);
          } else {
            boxes[hash].push((label, focal_length));
          }
        }
        Ok(Operation::Subtract(label)) => {
          let hash = get_hash(&label);
          if let Some(item) = boxes[hash].iter().position(|(l, _)| *l == label)
          {
            boxes[hash].remove(item);
          }
        }
        Err(e) => {
          return Err(e);
        }
      }
    }

    Ok(
      boxes
        .iter()
        .enumerate()
        .map(|(i, bx)| {
          bx.iter().enumerate().fold(0, |acc, (j, (_, fl))| {
            acc + get_focal_power(i + 1, j + 1, *fl as usize)
          })
        })
        .collect(),
    )
  }
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
