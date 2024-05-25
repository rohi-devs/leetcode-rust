pub fn longest_palindrome(s: String) -> String {
    if s.is_empty(){
        return String::new();
    }
    let s_char : Vec<char> = s.chars().collect();
    let mut start = 0;
    let mut end = 0;
    fn expand_around_cntr(s_char : &Vec<char>, left : usize, right : usize) -> (usize, usize)
    {
        let (mut l, mut r) = (left as isize, right as isize);
        while l>=0 && (r as usize) < s_char.len() && s_char[l as usize] == s_char[r as usize] {
            l -= 1;
            r += 1;
        }
        println!(
            "Expand around center: left = {}, right = {}, result = ({}, {})",
            left, right, l + 1, r
        );
        ((l+1) as usize, r as usize)
    }
    for i in 0..s_char.len()
    {
        println!("Checking center at index {}", i);
        let (l1,r1) = expand_around_cntr(&s_char,i,i);
        let (l2,r2) = expand_around_cntr(&s_char,i,i+1);
        if r1-l1 > end-start {
            start = l1;
            end = r1;
            println!(
                "Updated longest palindrome (odd length): start = {}, end = {}, substring = {}",
                start,
                end,
                s_char[start..end].iter().collect::<String>()
            );
        }
        if r2-l2 > end-start {
            start = l2;
            end = r2;
            println!(
                "Updated longest palindrome (even length): start = {}, end = {}, substring = {}",
                start,
                end,
                s_char[start..end].iter().collect::<String>()
            );
        }
    }
    s_char[start..end].iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn longest_palindrome_test() {
        assert_eq!(longest_palindrome("ababakammalappa".to_string()),"ababa".to_string());
        assert_eq!(longest_palindrome("a".to_string()),"a".to_string());
    }
}