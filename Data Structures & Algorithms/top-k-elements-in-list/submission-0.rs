impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut counts = HashMap::new();

    // First we insert everything into the hashmap
    for num in nums {
        *counts.entry(num).or_insert(0) += 1;
    }

    let mut v: Vec<_> = counts.into_iter().collect();

    v.sort_by(|k, v| v.1.cmp(&k.1));

    let values: Vec<_> = v.into_iter().map(|(key, _value)| key).collect();

    let kusize = k as usize;

    let result = if let Some(result) = values.get(..kusize) {
        result.to_vec()
    } else {
        Vec::new()
    };

    result

    }
}