pub fn solve(s: String) -> String {
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
        ((l+1) as usize, r as usize)
    }
    for i in 0..s_char.len()
    {
        let (l1,r1) = expand_around_cntr(&s_char,i,i);
        let (l2,r2) = expand_around_cntr(&s_char,i,i+1);
        if r1-l1 > end-start {
            start = l1;
            end = r1;
        }
        if r2-l2 > end-start {
            start = l2;
            end = r2;
        }
    }
    s_char[start..end].iter().collect()
}

#[cfg(test)]
mod test {
    use crate::dp::longest_palindrome;
    use super::*;
    #[test]
    fn test() {
        assert_eq!(solve("ababakammalappa".to_string()),"ababa".to_string());
        assert_eq!(solve("a".to_string()),"a".to_string());
    }
}