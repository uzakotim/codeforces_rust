use std::io::{self, Read, Write};
/// =========================
/// CORE SOLVER
/// =========================
/// Takes full input as &str, returns output as String


fn solve(input: &str) -> String {
    let mut iter = input.split_whitespace();
    let test_n = match iter.next() {
        Some(s) => s.parse::<i32>().unwrap_or(0),
        None => return String::new(),
    };
    let mut result = String::new();

    for _ in 0..test_n {
        let n = iter.next().unwrap().parse::<usize>().unwrap();
        let mut a: Vec<i64> = Vec::new();
        for _ in 0..n {
            a.push(iter.next().unwrap().parse::<i64>().unwrap());
        }
        let mut sum = a.iter().map(|x| x.abs()).sum::<i64>();
        // find how many are negative
        let mut neg_count = 0;
        for x in &a {
            if *x < 0 {
                neg_count += 1;
            }
        }
        if neg_count % 2 != 0 {
            //find the smallest in absolute value
            let min_abs = a.iter().map(|x| x.abs()).min().unwrap();
            sum -= 2 * min_abs;
        }
        result.push_str(&format!("{}\n", sum));
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
1
3
-1 -1 -1
    "#,
                r#"
1
    "#
            )
    }
    #[test]
    fn test_example_2() {
        check(
            r#"
1
5
1 5 -5 0 2
    "#,
                r#"
13
    "#
            )
    }
    #[test]
    fn test_example_3() {
        check(
            r#"
1
3
1 2 3
    "#,
                r#"
6
    "#
            )
    }
    #[test]
    fn test_example_4() {
        check(
            r#"
5
3
-1 -1 -1
5
1 5 -5 0 2
3
1 2 3
6
-1 10 9 8 7 6
2
-1 -1
    "#,
                r#"
1
13
6
39
2
    "#
            )
    }
    #[test]
    fn test_example_5() {
        check(
            r#"
1
3
5 2 -8
    "#,
                r#"
11
    "#
            )
    }
    #[test]
    fn test_example_6() {
        check(
            r#"
            1
            4
            2 -2 -7 -3
            "#,
                r#"
            10
            "#
            )
    }
    #[test]
    fn test_example_7() {
        check(
            r#"
            1
            5
            5 -2 -8 -6 -4
            "#,
            r#"
            25
            "#
        )
    }

}
