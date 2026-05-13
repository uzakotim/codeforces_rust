use std::io::{self, Read, Write};
/// =========================
/// CORE SOLVER
/// =========================
/// Takes full input as &str, returns output as String
fn solve(input: &str) -> String {
    let mut iter = input.split_whitespace();
    let houses = iter.next().unwrap().parse::<i64>().unwrap();
    let things_to_do = iter.next().unwrap().parse::<i64>().unwrap();
    let tasks_in_houses: Vec<i64> = iter.map(|s| s.parse::<i64>().unwrap()).collect();

    let mut result = String::new();
    let mut total_time = 0;
    let mut current_house = 1;
    
    for i in 0..tasks_in_houses.len() {
        if tasks_in_houses[i] < current_house {
            total_time += houses - current_house +1; 
            current_house = 1;
            total_time+=tasks_in_houses[i] - current_house; 
        } else {
            total_time += tasks_in_houses[i] - current_house; 
        }
        current_house = tasks_in_houses[i];
    }
    result.push_str(&total_time.to_string());
    result
}

/// =========================
/// MAIN (Codeforces mode)
/// =========================
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let output = solve(&input);

    let mut out = io::BufWriter::new(io::stdout());
    write!(out, "{}", output).unwrap();
}

/// =========================
/// TESTS (TDD mode)
/// =========================
#[cfg(test)]
mod tests {
    use super::*;

    /// ✨ Paste input/output EXACTLY (no \n needed)
    fn check(input: &str, expected: &str) {
        let output = solve(input);
        assert_eq!(output.trim(), expected.trim());
    }

    #[test]
    fn test_example_1() {
        check(
            r#"
4 3
3 2 3
"#,
            r#"
6
"#,
        );
    }
    #[test]
    fn test_example_2() {
        check(
            r#"
4 3
2 3 3
"#,
            r#"
2
"#,
        );
    }
}
