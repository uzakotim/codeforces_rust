fn main() {
    // read two numbers (N and M) as input and print their sum   
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (n, m) = (nums[0], nums[1]);
    // read lines of input into a matrix of size N x M
    let mut matrix = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let row: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        matrix.push(row);   
    } 

    
    let maxes: Vec<i32> = (0..m).map(|j| {
        matrix.iter().map(|row| row[j as usize]).fold(0, |max, x| max.max(x))
    }).collect();
    let sum = maxes.iter().sum::<i32>();
    println!("{}", sum);

}
