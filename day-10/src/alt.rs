use std::collections::HashSet;


fn main() {
  let contents = include_str!("../sample_2.txt");
  let lines: Vec<Vec<char>> = contents
    .lines()
    .map(|line| line.chars().collect())
    .collect();

  let mut starting_row = 0;
  let mut starting_column = 0;
  for (y, row) in lines.iter().enumerate() {
    for (x, column) in row.iter().enumerate() {
      if column == &'S' {
        starting_row = y;
        starting_column = x;
        break;
      }
    }
  }

  let mut check_pipes: Vec<(usize, usize)> = vec![];
  let mut seen_pipes: HashSet<(usize, usize)> = HashSet::new();
  let mut potential_s = vec!['|', '-', 'L', 'J', '7', 'F'];

  check_pipes.push((starting_row, starting_column));
  seen_pipes.insert((starting_row, starting_column));

  while !check_pipes.is_empty() {
    let (row, column) = check_pipes.remove(0);
    let current_pipe = lines[row][column];

    if row > 0
      && "S|LJ".contains(current_pipe)
      && "|7F".contains(lines[row - 1][column])
      && !seen_pipes.contains(&(row - 1, column))
    {
      seen_pipes.insert((row - 1, column));
      check_pipes.push((row - 1, column));
      if current_pipe == 'S' {
        potential_s.retain(|x| "|LJ".contains(*x));
      }
    }

    if row < lines.len() - 1
      && "S|7F".contains(current_pipe)
      && "|LJ".contains(lines[row + 1][column])
      && !seen_pipes.contains(&(row + 1, column))
    {
      seen_pipes.insert((row + 1, column));
      check_pipes.push((row + 1, column));
      if current_pipe == 'S' {
        potential_s.retain(|x| "|7F".contains(*x));
      }
    }

    if column > 0
      && "S-7J".contains(current_pipe)
      && "-LF".contains(lines[row][column - 1])
      && !seen_pipes.contains(&(row, column - 1))
    {
      seen_pipes.insert((row, column - 1));
      check_pipes.push((row, column - 1));
      if current_pipe == 'S' {
        potential_s = potential_s.into_iter().filter(|x| "-7J".contains(*x));
        potential_s.retain(|x| "-7J".contains(*x));
      }
    }
    if column < lines[row].len() - 1
      && "S-LF".contains(current_pipe)
      && "-J7".contains(lines[row][column + 1])
      && !seen_pipes.contains(&(row, column + 1))
    {
      seen_pipes.insert((row, column + 1));
      check_pipes.push((row, column + 1));
      if current_pipe == 'S' {
        potential_s = potential_s
          .into_iter()
          .filter(|x| "-LF".contains(*x))
          .collect();
      }
    }
  }

  let mut interior = 0;
  for row in &lines {
    let mut corner_pipes: Vec<char> = vec![];
    let mut intersect = 0;
    for column in 1..row.len() {
      if row[column] != '.' {
        continue;
      };

      if row[column] == '|' {
        intersect += 1
      }

      let mut corner_pipes = Vec::<char>::new();
      let mut intersect = 0;
      for column in row.iter().skip(1) {
        if column != &'.' {
          continue;
        };

        if column == &'|' {
          intersect += 1
        }

        if corner_pipes.is_empty() {
          // If the current character is a corner pipe, add it to corner_pipes vector
          if column == &'F' || column == &'L' {
            corner_pipes.push(*column);
          } else if (column == &'7' && corner_pipes.last().unwrap() == &'L')
            || (column == &'J' && corner_pipes.last().unwrap() == &'F')
          {
            // If the current character is a valid pipe and it matches with the last one in corner_pipes,
            // increment intersect and pop the last item from corner_pipes
            intersect += 1;
            corner_pipes.pop();
          } else if column != &'|' {
            // If the current character is not a vertical pipe or a valid corner pipe, add it to corner_pipes
            corner_pipes.push(*column);
          }
        }
      }
      if intersect % 2 == 1 {
        interior += 1;
      }
    }
  }

  println!("{}", interior);
}
