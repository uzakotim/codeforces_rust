use std::io::{self, Read, Write};

/// =========================
/// CORE SOLVER
/// =========================
/// Takes full input as &str, returns output as String
fn solve(input: &str) -> String {
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();

    // create array of n items
    let angles: Vec<i32> = (0..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();

    let mut result = String::new();
    for angle in angles {
        let n = 360 / (180 - angle);
        // n must be integer and n >= 3
        if n >= 3 && 360 % (180 - angle) == 0 {
            result.push_str("YES\n");
        } else {
            result.push_str("NO\n");
        }
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
5
30
60
90
179
99
"#,
            r#"
NO
YES
YES
YES
NO
"#,
        );
    }
}