use std::collections::HashSet;

pub fn is_unique(astr: String) -> bool {
    let mut _astr: Vec<char> = astr.chars().collect();
    let set: HashSet<_> = _astr.drain(..).collect();
    _astr.extend(set);
    astr.len() == _astr.len()
}

#[test]
fn test(){
    println!("{}",is_unique("leetcode".to_string()))
}