use regex::Regex;
use std::collections::HashMap;


#[cfg(feature = "sample")]
const DATA: &str = include_str!("../sample.txt");
#[cfg(not(feature = "sample"))]
const DATA: &str = include_str!("../input.txt");

#[cfg(not(feature = "part2"))]
type ProblemDefinition = (HashMap<String, Workflow>, Vec<Parts>);
#[cfg(feature = "part2")]
type ProblemDefinition = HashMap<String, Node>;

#[cfg(feature = "part2")]
struct Node {
  rules: Vec<BooleanEvaluation>,
  default: String,
}

#[cfg(feature = "part2")]
struct BooleanEvaluation {
  closure_name: char,
  operand: bool,
  value: usize,
  goal: String,
}

#[cfg(feature = "part2")]
#[derive(Debug, Clone, Copy)]
struct XMASBounds {
  x: (usize, usize),
  m: (usize, usize),
  a: (usize, usize),
  s: (usize, usize),
}

#[cfg(not(feature = "part2"))]
struct Workflow {
  rules: Vec<Box<Rule>>,
  default: String,
}

#[cfg(not(feature = "part2"))]
impl Workflow {
  fn create_rule(
    closure_name: char,
    is_greater_than: bool,
    value: usize,
    result: String,
  ) -> impl Fn(&Parts) -> Option<String> {
    move |x| {
      let val = x.get(closure_name);
      let is_evaluated_true = if is_greater_than {
        val > value
      } else {
        val < value
      };
      if is_evaluated_true {
        Some(result.to_owned())
      } else {
        None
      }
    }
  }
}

#[cfg(not(feature = "part2"))]
type Rule = dyn Fn(&Parts) -> Option<String>;

#[cfg(not(feature = "part2"))]
#[derive(Debug)]
struct Parts {
  x: usize,
  m: usize,
  a: usize,
  s: usize,
}

#[cfg(not(feature = "part2"))]
impl Parts {
  fn get(&self, elem: char) -> usize {
    match elem {
      'x' => self.x,
      'm' => self.m,
      'a' => self.a,
      's' => self.s,
      _ => unreachable!(),
    }
  }
}

fn src_provider() -> Result<String, String> {
  Ok(DATA.to_string())
}

fn extract() -> Result<ProblemDefinition, String> {
  let reg_rule = Regex::new(r"([xmas])([<>])(\d+):(\w+)|(\w+)").unwrap();

  let input = src_provider()?;

  #[allow(unused_variables)]
  let (workflows_input, parts_input) = input
    .split_once("\n\n")
    .ok_or("invalid workflow and parts definition")?;

  #[cfg(not(feature = "part2"))]
  {
    let reg_part = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)}").unwrap();
    let rules = workflows_input
      .trim()
      .lines()
      .map(|line| {
        let mut key = String::new();
        let mut workflow = Workflow { rules: vec![], default: String::new() };
        for (index, spec) in reg_rule.captures_iter(line).enumerate() {
          if index == 0 {
            key = spec[5].to_string();
            continue;
          }
          if spec.get(1).is_some() {
            let closure_name = spec[1].chars().next().unwrap();
            let operand = &spec[2];
            let value = spec[3].parse::<usize>().unwrap();
            let result = spec[4].to_string();
            let rule = Workflow::create_rule(
              closure_name,
              operand == ">",
              value,
              result,
            );
            workflow.rules.push(Box::new(rule));
          } else {
            workflow.default = spec[5].to_string();
          }
        }
        (key, workflow)
      })
      .collect();

    let parts: Vec<Parts> = reg_part
      .captures_iter(parts_input)
      .map(|cap| Parts {
        x: cap[1].parse().unwrap(),
        m: cap[2].parse().unwrap(),
        a: cap[3].parse().unwrap(),
        s: cap[4].parse().unwrap(),
      })
      .collect();

    Ok((rules, parts))
  }
  #[cfg(feature = "part2")]
  {
    let map = workflows_input
      .lines()
      .map(|line| {
        let mut node = Node { rules: vec![], default: String::new() };
        let mut name = String::new();
        for (index, spec) in reg_rule.captures_iter(line).enumerate() {
          if index == 0 {
            name = spec[5].to_string();
            continue;
          }
          if spec.get(1).is_some() {
            let closure_name = spec[1].chars().next().unwrap();
            let operand = &spec[2] == ">";
            let value = spec[3].parse::<usize>().unwrap();
            let goal = spec[4].to_string();
            node.rules.push(BooleanEvaluation {
              closure_name,
              operand,
              value,
              goal,
            });
          } else {
            node.default = spec[5].to_string();
          }
        }
        (name, node)
      })
      .collect();

    Ok(map)
  }
}

#[cfg(feature = "part2")]
fn dfs(
  map: &HashMap<String, Node>,
  current: String,
  range: XMASBounds,
) -> usize {
  if &current == "A" {
    let total = (range.x.1 - range.x.0 + 1)
      * (range.m.1 - range.m.0 + 1)
      * (range.a.1 - range.a.0 + 1)
      * (range.s.1 - range.s.0 + 1);

    return total;
  } else if &current == "R" {
    return 0;
  }
  let mut total = 0;
  let node = map.get(&current).unwrap();
  let mut range_no = range;

  for rule in node.rules.iter() {
    let mut range_yes = range_no;
    match rule.closure_name {
      'x' => {
        if rule.operand {
          range_yes.x.0 = rule.value + 1;
          range_no.x.1 = rule.value;
        } else {
          range_yes.x.1 = rule.value - 1;
          range_no.x.0 = rule.value;
        }
      }
      'm' => {
        if rule.operand {
          range_yes.m.0 = rule.value + 1;
          range_no.m.1 = rule.value;
        } else {
          range_yes.m.1 = rule.value - 1;
          range_no.m.0 = rule.value;
        }
      }
      'a' => {
        if rule.operand {
          range_yes.a.0 = rule.value + 1;
          range_no.a.1 = rule.value;
        } else {
          range_yes.a.1 = rule.value - 1;
          range_no.a.0 = rule.value;
        }
      }
      's' => {
        if rule.operand {
          range_yes.s.0 = rule.value + 1;
          range_no.s.1 = rule.value;
        } else {
          range_yes.s.1 = rule.value - 1;
          range_no.s.0 = rule.value;
        }
      }
      _ => unreachable!(),
    }
    total += dfs(map, rule.goal.clone(), range_yes);
  }
  total += dfs(map, node.default.clone(), range_no);

  total
}


fn transform(data: ProblemDefinition) -> Result<usize, String> {
  #[cfg(not(feature = "part2"))]
  {
    let (workflows, parts) = data;
    let mut total_rating_number = 0_usize;

    for part in parts {
      let mut workflow_id = String::from("in");
      loop {
        let workflow = workflows.get(&workflow_id).unwrap();
        for rule in workflow.rules.iter() {
          if let Some(result) = rule(&part) {
            workflow_id = result;
            break;
          } else {
            workflow_id = workflow.default.clone();
          }
        }
        if &workflow_id == "A" {
          total_rating_number += part.x + part.m + part.a + part.s;
          break;
        } else if &workflow_id == "R" {
          break;
        }
      }
    }

    Ok(total_rating_number)
  }
  #[cfg(feature = "part2")]
  {
    let total_combination: usize = dfs(
      &data,
      String::from("in"),
      XMASBounds { x: (1, 4000), m: (1, 4000), a: (1, 4000), s: (1, 4000) },
    );

    Ok(total_combination)
  }
}

fn load(result: Result<usize, String>) -> Result<(), String> {
  match result {
    Ok(value) => println!("{value}"),
    Err(e) => eprintln!("{e}"),
  }

  Ok(())
}

fn main() -> Result<(), String> {
  let data = extract()?;
  let result = transform(data);

  load(result)
}
