use std::io::{self, Read, Write};
/// =========================
/// CORE SOLVER
/// =========================
/// Takes full input as &str, returns output as String
fn solve(input: &str) -> String {
    let mut iter = input.split_whitespace();
    let test_n = iter.next().unwrap().parse::<i32>().unwrap();
    let mut result = String::new();

    for _ in 0..test_n {
        let n = iter.next().unwrap().parse::<i64>().unwrap();
        let k = iter.next().unwrap().parse::<i64>().unwrap();
        if n % k == 0{
            result.push_str(format!("{}\n", 1).as_str());
            continue;
        }
        if n == 1 {
            result.push_str(format!("{}\n", k).as_str());
            continue;
        }
        let sum_answer : i64 = (n/k as i64 + 1) * k;
        if sum_answer % n == 0 {
            result.push_str(format!("{}\n", sum_answer/n).as_str());
            continue;
        }
        let max_element: i64 = (sum_answer/n) as i64 + 1;
        result.push_str(format!("{}\n", max_element).as_str());
    }
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
    fn test_example_all_ones() {
        check(
            r#"
            1
            8 8
            "#,
                        r#"
            1
            "#,
        );
    }

    #[test]
    fn test_example_one_two_one_two() {
        check(
            r#"
            1
            4 3
            "#,
            r#"
            2
            "#,
        );
    }
    #[test]
    fn test_single_number() {
        check(
            r#"
            1
            1 5
            "#,
            r#"
            5
            "#,
        );
    }
    #[test]
    fn test_last() {
        check(
            r#"
            1
            8 17
            "#,
            r#"
            3
            "#,
        );
    }
    #[test]
    fn test_divisible_by_one(){
        check(
            r#"
            1
            2 1
            "#,
            r#"
            1
            "#,
        );
    }
    #[test]
    fn test_two_four(){
        check(
            r#"
            1
            2 4
            "#,
            r#"
            2
            "#,
        )
    }

}
