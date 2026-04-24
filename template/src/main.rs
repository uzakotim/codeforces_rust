use std::io::{self, Read, Write};

/// =========================
/// CORE SOLVER
/// =========================
/// Takes full input as &str, returns output as String
fn solve(input: &str) -> String {
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut groups: Vec<i32> = (0..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();

    groups.sort_by(|a, b| b.cmp(a));

    let mut taxis = 0;

    while !groups.is_empty() {
        let mut sum = groups.remove(0);

        while sum < 4 {
            if let Some(v) = groups.pop() {
                if sum + v > 4 {
                    break;
                }
                sum += v;
            } else {
                break;
            }
        }

        taxis += 1;
    }

    taxis.to_string()
}

/// =========================
/// MAIN (Codeforces mode)
/// =========================
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let output = solve(&input);

    let mut out = io::BufWriter::new(io::stdout());
    writeln!(out, "{}", output).unwrap();
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
8
2 3 4 4 2 1 3 1
"#,
            r#"
5
"#,
        );
    }

    #[test]
    fn test_example_2() {
        check(
            r#"
12
1 1 1 1 1 1 1 1 1 1 1 1
"#,
            r#"
3
"#,
        );
    }
}