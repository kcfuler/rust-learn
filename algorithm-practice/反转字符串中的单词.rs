impl Solution {
    pub fn reverse_words(s: String) -> String {
        // 这里的split_whitespace方法会忽略词间所有的空格
        let mut words: Vec<&str> = s.trim().split_whitespace().collect();
        words.reverse();
        words.join(" ")
    }
}
