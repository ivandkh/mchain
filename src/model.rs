use rand::seq::SliceRandom;
use std::collections::HashMap;

pub struct Model {
    pub chain: HashMap<String, Vec<(String, String)>>,
    startwords: Option<Vec<String>>,
    stopwords: Option<Vec<String>>,
    seq_length: i32,
    is_fitted: bool,
}

impl Model {
    pub fn new(seq_length: i32) -> Model {
        Model {
            chain: HashMap::new(),
            startwords: None,
            stopwords: None,
            seq_length,
            is_fitted: false,
        }
    }

    pub fn fit_ngrams(&mut self, ngrams: Vec<Vec<&str>>) {
        for ngram in ngrams {
            let key = ngram[0];
            let last_word = ngram[ngram.len() - 1].to_string();
            let new_val = ngram[1..].join(" ");

            match self.chain.get_mut(key) {
                Some(val) => val.push((new_val, last_word)),
                None => {
                    self.chain
                        .insert(key.to_string(), vec![(new_val, last_word)]);
                }
            }
        }
        self.is_fitted = true;
    }

    pub fn fit_startwords(&mut self, startwords: Vec<String>) {
        self.startwords = Some(startwords);
    }

    pub fn fit_stopwords(&mut self, stopwords: Vec<String>) {
        //TODO Add stopwords to generate()
        self.stopwords = Some(stopwords);
    }

    pub fn generate(&self, start_word: Option<&String>) -> String {
        assert!(self.is_fitted, "Model not fitted.");

        let mut output = String::new();
        let mut rng = rand::thread_rng();

        //unwrap start_word if specified in generate() params
        //choose random_start word from model.startwords else
        //panic otherwise
        let mut word = start_word.unwrap_or_else(|| {
            self.startwords
                .as_ref()
                .expect("No start words specified.")
                .choose(&mut rng)
                .unwrap()
        });
        output.push_str(&format!("{} ", word));

        for _ in 1..self.seq_length {
            match self.chain.get(word) {
                Some(ngrams) => {
                    let (ngram, end_word) = ngrams.choose(&mut rng).unwrap();
                    output.push_str(&format!("{} ", &ngram));
                    word = end_word;
                }
                None => break,
            }
        }

        output + ":)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools;

    #[test]
    #[should_panic(expected = "Model not fitted.")]
    fn test_not_fitted_panic() {
        let mut chain = Model::new(20);
        chain.generate(None);
    }

    #[test]
    #[should_panic(expected = "No start words specified.")]
    fn test_no_startword_panic() {
        let mut chain = Model::new(20);
        let text = String::from("It is a long established.");
        let ngrams = tools::get_ngrams(2, &text);
        chain.fit_ngrams(ngrams);
        chain.generate(None);
    }
}
