pub fn replace_spaces(s: String, length: i32) -> String {
    s[0..length as usize].replace(" ", "%20")
}

#[test]
fn test(){
    println!("{}",replace_spaces("Mr John Smith    ".to_string(),13))
}