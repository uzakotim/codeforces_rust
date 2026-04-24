use std::io::{self, Read, Write};
/// =========================
/// CORE SOLVER
/// =========================
/// Takes full input as &str, returns output as String
fn solve(input: &str) -> String {
    use std::collections::HashSet;
    
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let a: Vec<usize> = (0..n).map(|_| iter.next().unwrap().parse().unwrap()).collect();
    let l: Vec<usize> = (0..m).map(|_| iter.next().unwrap().parse().unwrap()).collect();
    
    let mut result = String::new();
    
    let mut hash_set: HashSet<usize> = HashSet::new();
    let mut counters: Vec<usize> = vec![0; n];
    for i in 1..=n {
        // make an array of counters of unique elements 
        hash_set.insert(a[n - i]);
        counters[i-1] = hash_set.len() as usize;
    }

    for i in l {
        result.push_str(&counters[n-i].to_string());
        result.push_str("\n");
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
10 10
1 2 3 4 1 2 3 4 100000 99999
1
2
3
4
5
6
7
8
9
10
"#,
            r#"
6
6
6
6
6
5
4
3
2
1
"#,
        );
    }
}