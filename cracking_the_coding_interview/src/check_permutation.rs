pub fn check_permutation(s1: String, s2: String) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    s1.chars().all(|c| s1.matches(c).count() == s2.matches(c).count())
}

#[test]
fn test(){
    println!("{}",check_permutation("aqbd".to_string(),"bca".to_string()))
}