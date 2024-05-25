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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn find_median_sorted_arrays_test() {
        assert_eq!(find_median_sorted_arrays(vec![1,2], vec![2]), 2 as f64);
        assert_eq!(find_median_sorted_arrays(vec![1,2], vec![3,4]), 2.5 as f64);
    }
}