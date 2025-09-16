fn main() {
    // read a line from stdin and convert that to a number
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: i64 = input.trim().parse().unwrap();

    // Read a line of N space-separated integers
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let start: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Read a line of N space-separated integers
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let goal: Vec<i64> = input
        .trim()
        .split_whitespace()         
        .map(|s| s.parse().unwrap())
        .collect(); 

    // Read a line of N space-separated integers
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let mapping: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let mut current = start.clone();
    let mut count: i64 = 0;
    while current != goal {
        // print current
        let mut next = vec![0; n as usize];
        for i in 0..n as usize {
            next[mapping[i] as usize -1] = current[i]; 
        }
        current = next;
        count += 1;
        if current == start {
            println!("IMPOSSIVEL");
            return;
        }
        if count > 1000000000 {
        println!("DEMAIS");
        return;
        }
    }
  
    println!("{}", count);
    
}
