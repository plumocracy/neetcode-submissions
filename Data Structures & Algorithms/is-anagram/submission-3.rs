impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
    // Anagrams must be the same length
    if s.len() != t.len() {
        return false;
    }

    // This will store the character and the amount of times
    // its been seen.
    let mut seen: HashMap<char, i32> = HashMap::new();

    // Store the count of all the chars in string s
    for c in s.chars() {
        *seen.entry(c).or_insert(0) += 1;
    }

    // Decrement the count of each char, if the overall count of each char
    // is 0 at the end we know we have an angram
    for c in t.chars() {
        let count = seen.entry(c).or_insert(0);
        *count -= 1;

        // If we didnt have this character before, we'll drop the count below 0
        if *count < 0 {
            return false;
        }
    }

    true


    }
}
