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
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![0; nums.len()];
        let sum: i32 = nums.iter().sum();
        let n = nums.len() as i32;
        for (i, val) in nums.iter().enumerate() {
            if i == 0 {
                ans[i] = sum - val * n;
            } else {
                let last = ans[i - 1];
                let d = nums[i] - nums[i - 1];  // val - nums[i-1]
                ans[i] = last + (2 * (i as i32) - n) * d;
            }
        }
        ans
    }
    pub fn max_vowels(s: String, k: i32) -> i32 {
        fn is_vowel(c: &u8) -> bool {
            match c {
                b'a' | b'e' | b'i' | b'o' | b'u' => true,
                _ => false
            }
        }
        let mut count = 0;
        for i in s.bytes().take(k as usize) {
            if is_vowel(&i) {
                count += 1;
            }
        }
        let mut ans = count;
        for i in (k as usize)..s.len() {
            let r = &s.as_bytes()[i];
            let l = &s.as_bytes()[i - (k as usize)];
            match (is_vowel(l), is_vowel(r)) {
                (true, false) => count -= 1,
                (false, true) => count += 1,
                _ => ()
            }
            if count > ans {
                ans = count;
            }
        }
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