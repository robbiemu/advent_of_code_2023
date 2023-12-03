use std::collections::HashSet;


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Range {
  start: Coord,
  end: Coord,
  number: usize,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Symbol {
  coord: Coord,
  symbol: char,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
struct Coord {
  y: usize,
  x: usize,
}

struct ProblemRepresentation {
  ranges: Vec<Range>,
  symbols: Vec<Symbol>,
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

fn parse_input(input: &str) -> Result<(Vec<Range>, Vec<Symbol>), String> {
  let mut symbols = Vec::new();
  let mut ranges = Vec::new();

  for (y, line) in input.lines().enumerate() {
    if line.trim().is_empty() {
      return Err("Empty input".to_string());
    }

    let chars: Vec<char> = line.chars().collect();
    let mut x = 0;
    while x < line.len() {
      match chars[x] {
        '.' => {} // Skip periods
        _ if chars[x].is_ascii_digit() => {
          let start = Coord { x, y };
          let num_str = chars
            .iter()
            .skip(x)
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>();

          let end_of_number = x + num_str.len() - 1;

          let end = Coord { x: end_of_number, y };
          let number = &line[x..=end_of_number];
          ranges.push(Range { start, end, number: number.parse().unwrap() });
          x = end_of_number; // Skip to the end of this number in the next iteration
        }
        c => symbols.push(Symbol { coord: Coord { x, y }, symbol: c }), // Symbols are everything else
      }
      x += 1;
    }
  }

  Ok((ranges, symbols))
}

fn extract() -> Result<ProblemRepresentation, String> {
  let schematic = src_provider()?;
  let (ranges, symbols) = parse_input(schematic.as_str())?;

  Ok(ProblemRepresentation { ranges, symbols })
}

fn transform(data: ProblemRepresentation) -> Result<Vec<usize>, String> {
  let symbol_coords: HashSet<Coord> =
    data.symbols.iter().map(|s| s.coord).collect();

  let mut filtered_ranges = Vec::new();
  for range in data.ranges {
    let is_before_in_line = range.start.x > 0
      && symbol_coords
        .contains(&Coord { x: range.start.x - 1, y: range.start.y });

    let is_after_in_line =
      symbol_coords.contains(&Coord { x: range.end.x + 1, y: range.start.y });

    let is_above_or_below = symbol_coords.iter().any(|s| {
      s.y.abs_diff(range.start.y) == 1
        && (s.x >= range.start.x.saturating_sub(1) && s.x <= range.end.x + 1)
    });
    if is_before_in_line || is_after_in_line || is_above_or_below {
      filtered_ranges.push(range);
    }
  }

  Ok(filtered_ranges.iter().map(|range| range.number).collect())
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
  use super::*;

  // MARK extract
  #[test]
  fn test_parse_input() {
    let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let expected_symbols = vec![
      Symbol { coord: Coord { x: 3, y: 1 }, symbol: '*' },
      Symbol { coord: Coord { x: 6, y: 3 }, symbol: '#' },
      Symbol { coord: Coord { x: 3, y: 4 }, symbol: '*' },
      Symbol { coord: Coord { x: 5, y: 5 }, symbol: '+' },
      Symbol { coord: Coord { x: 3, y: 8 }, symbol: '$' },
      Symbol { coord: Coord { x: 5, y: 8 }, symbol: '*' },
    ];

    let expected_ranges = vec![
      Range {
        start: Coord { x: 0, y: 0 },
        end: Coord { x: 2, y: 0 },
        number: 467,
      },
      Range {
        start: Coord { x: 5, y: 0 },
        end: Coord { x: 7, y: 0 },
        number: 114,
      },
      Range {
        start: Coord { x: 2, y: 2 },
        end: Coord { x: 3, y: 2 },
        number: 35,
      },
      Range {
        start: Coord { x: 6, y: 2 },
        end: Coord { x: 8, y: 2 },
        number: 633,
      },
      Range {
        start: Coord { x: 0, y: 4 },
        end: Coord { x: 2, y: 4 },
        number: 617,
      },
      Range {
        start: Coord { x: 7, y: 5 },
        end: Coord { x: 8, y: 5 },
        number: 58,
      },
      Range {
        start: Coord { x: 2, y: 6 },
        end: Coord { x: 4, y: 6 },
        number: 592,
      },
      Range {
        start: Coord { x: 6, y: 7 },
        end: Coord { x: 8, y: 7 },
        number: 755,
      },
      Range {
        start: Coord { x: 1, y: 9 },
        end: Coord { x: 3, y: 9 },
        number: 664,
      },
      Range {
        start: Coord { x: 5, y: 9 },
        end: Coord { x: 7, y: 9 },
        number: 598,
      },
    ];

    let result = parse_input(input);
    assert!(result.is_ok());
    let (mut ranges, mut symbols) = result.unwrap();
    ranges.sort_by_key(|r| r.start);
    symbols.sort_by_key(|s| s.coord);
    assert_eq![symbols, expected_symbols];
    assert_eq![ranges, expected_ranges];
  }

  // MARK transform

  // MARK load
}
