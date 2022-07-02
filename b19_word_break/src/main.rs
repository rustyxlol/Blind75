fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut breakable = vec![false; s.len() + 1];
    breakable[0] = true;

    for end in 0..s.len() + 1 {
        for start in 0..end {
            let word = s[start..end].to_string();
            if breakable[start] && word_dict.contains(&word) {
                breakable[end] = true;
            }
        }
    }

    breakable[s.len()]
}
fn main() {
    println!(
        "{}",
        word_break(
            "applepenapple".to_string(),
            vec!["apple".to_string(), "pen".to_string()]
        )
    );
}
