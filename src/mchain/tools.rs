pub fn get_ngrams(n: usize, text: &str) -> Vec<Vec<&str>> {
    let mut ngrams = Vec::new();
    let words: Vec<&str> = text.split_whitespace().collect();

    for i in 0..words.len() - n {
        let ngram = words[i..i + n].to_vec();
        ngrams.push(ngram);
    }

    ngrams
}

#[allow(dead_code)]
pub fn get_stopwords(text: &str) -> Vec<String> {
    text.split_whitespace()
        .filter(|word| word.contains("."))
        .map(|word| word.replace(".", ""))
        .collect()
}

pub fn get_startwords(text: &str) -> Vec<String> {
    text.split(". ")
        .map(|sent| sent.split_once(" ").unwrap_or(("", "")).0.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_startwords() {
        assert_eq!(
            get_startwords("test. test Test. test. test"),
            [
                String::from("test"),
                String::from("Test"),
                String::from("test")
            ]
        );
    }
}
