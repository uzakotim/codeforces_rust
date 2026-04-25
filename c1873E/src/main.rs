use std::io::{self, Read, Write};
/// =========================
/// CORE SOLVER
/// =========================
/// Takes full input as &str, returns output as String
fn solve(input: &str) -> String {
    let mut iter = input.split_whitespace();

    let test_n = match iter.next() {
        Some(s) => s.parse::<i32>().unwrap(),
        None => return String::new(),
    };
    let mut result = String::new();
    for _ in 0..test_n {
        let n = iter.next().unwrap().parse::<usize>().unwrap();
        let x = iter.next().unwrap().parse::<i64>().unwrap();
        let a: Vec<i64> = (0..n)
            .map(|_| iter.next().unwrap().parse().unwrap())
            .collect();

        let mut low = 1i64;
        let mut high = 2_000_000_000 + 1_000_000_000 + 7; // Safe upper bound
        let mut ans = 1;

        while low <= high {
            let mid = low + (high - low) / 2;
            let mut total_water: i64 = 0;
            for &height in &a {
                if mid > height {
                    // summing the reverse of columns for a potential height
                    total_water += mid - height;
                    if total_water > x {
                        break;
                    }
                }
            }

            if total_water <= x {
                ans = mid;
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        result.push_str(&ans.to_string());
        result.push('\n');
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
2
7 9
3 1 2 4 6 2 5
3 10
1 1 1
"#,
            r#"
4
4
"#,
        );
    }

    #[test]
    fn test_example_2() {
        check(
            r#"
5
7 9
3 1 2 4 6 2 5
3 10
1 1 1
4 1
1 4 3 4
6 1984
2 6 5 9 1 8
1 1000000000
1
"#,
            r#"
4
4
2
335
1000000001
"#,
        );
    }
}
