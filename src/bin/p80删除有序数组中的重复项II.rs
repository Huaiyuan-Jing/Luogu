struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut ed = 0 as usize;
        for i in 0..nums.len() {
            if i + 2 < nums.len() && nums[i] == nums[i + 1] && nums[i] == nums[i + 2] {
                continue;
            }
            nums[ed] = nums[i];
            ed += 1;
        }
        ed as i32
    }
}
fn main() {}
