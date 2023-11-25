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
        let mut ans = 0;
        let mut nums = nums;
        nums.sort();
        let mut cn = nums[0];   // current num
        let mut count = 1;
        let mut repeat_total = 0;   // only increase
        for (i, val) in nums.iter().enumerate().skip(1) {
            if *val == cn {
                count += 1;
                continue;
            }
            repeat_total += count - 1;
            ans += count * ((i as i32) - 1 - repeat_total);  // #cn * operator need for cn
            cn = *val;
            count = 1;
        }
        repeat_total += count - 1;
        ans += count * ((nums.len() as i32) - 1 - repeat_total);
        ans
    }
}

/* need to learn how test the trait
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_reduction_operations() {
        assert_eq!(reduction_operations(vec![5, 1, 3]), 3);
    }
}
*/