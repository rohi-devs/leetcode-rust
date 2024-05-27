pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    fn isodd(num : i32) -> bool {
        return num%2 != 0;
    }
    let mut a1 = nums1;
    let mut nums2 = nums2.clone();
    a1.append(&mut nums2);
    a1.sort();
    let arr_len = a1.len();
    if isodd(arr_len as i32) {
        return a1[(arr_len/2) as usize] as f64;
    }
    else {
        let k1 = a1[(arr_len/2-1) as usize];
        let k2 = a1[(arr_len/2) as usize];
        let res : f64 = ((k1 as f64)+(k2 as f64))/2.0;
        return res;
    }
}

pub fn longest_valid_parentheses(s: impl Into<String>) -> i32 {
    let s = s.into();
    let mut stack : Vec<usize> = Vec::new();
    let mut max_length = 0;
    let mut start = 0;
    for (i,c) in s.chars().enumerate() {
        if c == '(' {
            stack.push(i);
        }
        else {
            if stack.is_empty() {
                start = i+1;
            }
            else {
                stack.pop();
                max_length = max_length.max(if stack.is_empty() {
                    i - start + 1
                }
                else {
                    i - stack.last().unwrap()
                });
            }
        }
    }
    return max_length as i32;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn find_median_sorted_arrays_test() {
        assert_eq!(find_median_sorted_arrays(vec![1,2], vec![2]), 2 as f64);
        assert_eq!(find_median_sorted_arrays(vec![1,2], vec![3,4]), 2.5 as f64);
    }
    #[test]
    fn longest_valid_parentheses_test() {
        assert_eq!(longest_valid_parentheses("((()"),2);
        assert_eq!(longest_valid_parentheses("(()()"),4);
    }
}