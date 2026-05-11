use std::io::{self, Read, Write};
/// =========================
/// CORE SOLVER
/// =========================
/// Takes full input as &str, returns output as String
fn solve(input: &str) -> String {
    let mut result = String::new();
    let words: Vec<&str> = input.split_whitespace().collect();
    for (i, word) in words.iter().enumerate() {
        if i > 0 {
            result.push(' ');
        }
        let chars: Vec<char> = word.chars().collect();
        if chars.iter().skip(1).all(|c| c.is_uppercase()) {
            let flipped: String = chars.iter()
                .map(|&c| {
                    if c.is_lowercase() {
                        c.to_uppercase().to_string()
                    } else {
                        c.to_lowercase().to_string()
                    }
                })
                .collect();
            result.push_str(&flipped);
        } else {
            result.push_str(word);
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
                r#"
    cAPS
    "#,
                r#"
    Caps
    "#
            )
        }
    #[test]
    fn test_example_2() {
        check(
            r#"
Lock
"#,
            r#"
Lock
"#
        )
    }

    #[test]
    fn test_example_3() {
        check(
            r#"
HTTP
"#,
            r#"
http
"#
        )
    }
}
