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
        const RANGE: usize = 50000 + 1;
        let mut freq = [0; RANGE];
        for i in nums.iter() {
            freq[*i as usize] += 1;
        }
        let mut ans = 0;
        let mut accum = 0;
        for f in freq.into_iter().rev() {
            if f > 0 {
                accum += f;
                ans += accum;
            }
        }
        ans -= accum;   // it should not be added for the min num
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