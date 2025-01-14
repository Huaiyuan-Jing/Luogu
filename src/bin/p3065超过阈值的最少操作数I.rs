struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().filter(|&x| *x < k).count() as i32
    }
}
fn main() {
    println!("{}", Solution::min_operations(vec![1, 2, 3, 4], 5));
}
