use super::tools; //move this import to test module
use rand::seq::SliceRandom;
use std::collections::HashMap;

pub struct Model<'a> {
    pub chain: HashMap<&'a str, Vec<(String, String)>>,
    startwords: Option<Vec<String>>,
    _stopwords: Option<Vec<&'a str>>,
    seq_length: i32,
    is_fitted: bool,
}

impl<'a> Model<'a> {
    pub fn new(seq_length: i32) -> Model<'a> {
        Model {
            chain: HashMap::new(),
            startwords: None,
            _stopwords: None,
            seq_length,
            is_fitted: false,
        }
    }

    pub fn fit_ngrams(&mut self, ngrams: Vec<Vec<&'a str>>) {
        for ngram in ngrams {
            let key = ngram[0];
            let last_word = ngram[ngram.len() - 1].to_string();
            let new_val = ngram[1..].join(" ");

            match self.chain.get_mut(key) {
                Some(val) => val.push((new_val, last_word)),
                None => {
                    self.chain.insert(key, vec![(new_val, last_word)]);
                }
            }
        }
        self.is_fitted = true;
    }

    pub fn fit_startwords(&mut self, startwords: Vec<String>) {
        self.startwords = Some(startwords);
    }

    #[allow(dead_code)]
    pub fn fit_stopwords() {
        unimplemented!()
    }

    pub fn generate(self, start_word: Option<&String>) -> String {
        assert!(self.is_fitted, "Model not fitted.");

        //let mut output = String::new();
        let mut rng = rand::thread_rng();
        let output = start_word.unwrap_or_else(|| {
            self.startwords
                .as_ref()
                .expect("No start words specified.")
                .choose(&mut rng)
                .unwrap()
        });

        // //if start word specified in generate() params
        // if let Some(mut start_word) = start_word {
        //     output.push_str(&format!("{} ", &start_word));
        //
        //     for _ in 1..self.seq_length {
        //         match self.chain.get(start_word) {
        //             Some(ngrams) => {
        //                 let mut rng = rand::thread_rng();
        //                 let (ngram, end_word) = ngrams.choose(&mut rng).unwrap();
        //                 output.push_str(&format!("{} ", &ngram));
        //                 start_word = end_word;
        //             }
        //             None => break,
        //         }
        //     }
        // //if model has a list of fitted start words
        // } else if let Some(_words) = self.startwords {
        //     unreachable!()
        // } else {
        //     panic!("No start words specified.")
        // }

        //*output + ":)"
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let ngrams = super::tools::get_ngrams(2, &text);
        chain.fit_ngrams(ngrams);
        chain.generate(None);
    }
}
