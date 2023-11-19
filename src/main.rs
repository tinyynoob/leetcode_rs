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
        let n = nums.len();
        let half = nums.len() / 2;
        let mut nums = nums;
        nums.sort();
        let mut max = 0;
        for i in 0..half {
            let pair_sum = nums.get(i).unwrap_or_else(|| &0)
                             + nums.get(n - i - 1).unwrap_or_else(|| &0);
            if pair_sum > max {
                max = pair_sum;
            }
        }
        max
    }
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut max = *nums.iter().max().unwrap();  // must exist
        let mut count = 0;
        loop {
            let second_max = nums.iter().filter(|&&x| x != max).max();
            if second_max == None {
                return count;
            }
            let second_max = *second_max.unwrap();
            for it in nums.iter_mut() {
                if *it == max {
                    count += 1;
                    *it = second_max;
                }
            }
            max = second_max;
        }
    }
}