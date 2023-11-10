fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let first_str = strs[0].clone();

    for (i, c) in first_str.chars().enumerate() {
        for s in strs.iter().skip(1) {
            if let Some(sc) = s.chars().nth(i) {
                if sc != c {
                    return first_str[..i].to_string();
                }
            } else {
                return first_str[..i].to_string();
            }
        }
    }

    first_str
}
