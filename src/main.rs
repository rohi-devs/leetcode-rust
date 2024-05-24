use leetcode::dp;
fn main() {
    let res = dp::longest_palindrome::solve(String::from("ababbaammmma"));
    dbg!(res);
}
