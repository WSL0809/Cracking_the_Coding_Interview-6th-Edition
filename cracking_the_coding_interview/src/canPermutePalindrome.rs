use std::collections::HashMap;

pub fn can_permute_palindrome(s: String) -> bool {
    let mut odd = 0;
    for value in count(s.as_str()).values() {
        if value % 2 == 1 {
            odd += 1;
            if odd > 1 {
                return false
            }
        }
    }
    return true
}
fn count(s: &str) -> HashMap<char, i32>{
    let mut  count_chars = HashMap::new();
    for ch in s.chars() {
        let count = count_chars.entry(ch).or_insert(0);
        *count += 1;
    }
    count_chars
}

#[test]
fn test(){
    let s = "code".to_string();
    println!("{:?}", can_permute_palindrome(s));
}