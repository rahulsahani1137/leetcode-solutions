impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut a = 0;
        let mut b = s.len() - 1;
        
        while a < b {
            s.swap(a, b);
            a += 1;
            b -= 1;
        }
    }
}