fn main() {
    let t = read_numbers_from_input()[0];
    for _ in 0..t {
        test_case();
    }
}
fn test_case() {
    let n = read_numbers_from_input()[0];
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
