fn main() {
    println!("Hello, world!");
}

trait Solution {
    fn s(&self) -> String;
}

impl dyn Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut ans = String::new();
        for (i, s) in nums.iter().enumerate() {
            match s.as_bytes()[i] {
                b'0' => ans.push('1'),
                b'1' => ans.push('0'),
                _ => ()
            }
        }
        ans
    }
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let half = nums.len() / 2;
        nums.iter().take(half)
                    .zip(nums.iter().rev().take(half))
                    .map(|(&a, &b)| a + b)
                    .max().unwrap()
    }
}
