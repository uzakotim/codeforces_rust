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
        
        // first combination of upto k odd numbers
        let combination_sum = k * k;
        // determined next_combination_sum as upper bound from n to next 2**i
        let next_combination_k = (n as f64).sqrt() as i64+1;
        let next_combination_sum = next_combination_k * next_combination_k;
        
        if k % 2 == 0{
            if n>=combination_sum && n< next_combination_sum{
                if n % 2 == 0 {
                    result.push_str("YES\n");
                }else{
                    result.push_str("NO\n");
                }
            }else{
                result.push_str("NO\n");
            }
        }
        else {
            if n>=combination_sum && n< next_combination_sum{

                if n % 2 == 1 {
                    result.push_str("YES\n");
                }else{
                    result.push_str("NO\n");
                }
            }else{
                result.push_str("NO\n");
            }
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
6
3 1
4 2
10 3
10 2
16 4
16 5
"#,
            r#"
YES
YES
NO
YES
YES
NO
"#,
        );
    }
}
