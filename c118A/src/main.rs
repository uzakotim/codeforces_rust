use std::io::{self, Read, Write};
/// =========================
/// CORE SOLVER
/// =========================
/// Takes full input as &str, returns output as String
fn solve(input: &str) -> String {
    let mut result = String::new();
    let consonants = [
        'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'z'
    ];
    for c in input.to_lowercase().chars() {
        if consonants.contains(&c) {
            result.push('.');
            result.push(c);
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
                "tour",
                ".t.r"
            )
        }
        #[test]
        fn test_example_2() {
            check(
                "Codeforces",
                ".c.d.f.r.c.s"
            )
        }
}
