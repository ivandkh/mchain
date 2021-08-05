pub fn get_ngrams(n: usize, text: &str) -> Vec<Vec<&str>> {
    let mut ngrams = Vec::new();
    let words: Vec<&str> = text.split_whitespace().collect();

    for i in 0..words.len() - n {
        let ngram = words[i..i + n].to_vec();
        ngrams.push(ngram);
    }

    ngrams
}
