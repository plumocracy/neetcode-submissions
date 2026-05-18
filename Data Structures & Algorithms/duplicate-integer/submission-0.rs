impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
            let mut seen: HashSet<i32> = HashSet::new();
            nums.into_iter().any(|n| !seen.insert(n))
    }
}