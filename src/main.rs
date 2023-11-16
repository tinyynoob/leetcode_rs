fn main() {
    println!("Hello, world!");
}

trait Solution {
    fn s(&self) -> String;
}

use std::collections::HashSet;
impl dyn Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut h = HashSet::new();
        for s in nums.iter() {
            h.insert(usize::from_str_radix(s, 2).expect("Not binary"));
        }
        for i in 0..=nums.len() {
            if h.get(&i) == None {
                return format!("{:0w$b}", i, w = nums.len());
            }
        }
        String::new()
    }
}
