fn main() {
    let t = read_i32();
    for _ in 0..t {
        test_case();
    }
}
fn test_case() {
    let n = read_i32();
    let arr = read_numbers_from_input();
    let mut bit = FenwickTree::new(1000000);
    let mut ans = 0;
    for i in (0..n as usize).rev() {
        ans += bit.query(arr[i] as usize - 1);
        bit.update(arr[i] as usize, 1);
    }
    println!("{}", ans);
}
fn read_numbers_from_input() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to get input");
    input
        .split_whitespace()
        .filter_map(|word| word.parse::<i32>().ok())
        .collect()
}

fn read_i32() -> i32 {
    use std::io::{self, Read, Write};
    let mut number = String::new();
    io::stdout().flush().unwrap();
    let mut stdin = io::stdin();
    let mut buffer = [0; 1];
    while let Ok(()) = stdin.read_exact(buffer.as_mut()) {
        let c = buffer[0] as char;
        if number.is_empty() && c == '-' {
            number.push(c);
            continue;
        }
        if c.is_numeric() {
            number.push(c);
        } else {
            break;
        }
    }
    number.parse::<i32>().unwrap_or(0)
}
struct FenwickTree {
    tree: Vec<i32>,
    size: usize,
}

impl FenwickTree {
    fn new(size: usize) -> Self {
        FenwickTree {
            tree: vec![0; size + 1],
            size,
        }
    }
    fn update(&mut self, mut index: usize, delta: i32) {
        while index <= self.size {
            self.tree[index] += delta;
            index += index & (!index + 1);
        }
    }

    fn query(&self, mut index: usize) -> i32 {
        let mut sum = 0;
        while index > 0 {
            sum += self.tree[index];
            index -= index & (!index + 1);
        }
        sum
    }
}
