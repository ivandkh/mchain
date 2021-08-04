use rand::seq::SliceRandom;
use std::collections::HashMap;

pub struct Model<'a> {
    pub chain: HashMap<&'a str, Vec<(String, String)>>,
    startwords: Option<Vec<&'a str>>,
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
            //let new_val = Vec::from(&ngram[1..]);
            let last_word = ngram[ngram.len() - 1].to_string();
            let new_val = ngram[1..].join(" ");

            match self.chain.get_mut(key) {
                Some(val) => val.push((new_val, last_word)),
                None => {
                    self.chain.insert(key, vec![(new_val, last_word)]);
                }
            }
        }
        self.is_fitted = true
    }

    #[allow(dead_code)]
    pub fn fit_startwords() {
        unimplemented!()
    }

    #[allow(dead_code)]
    pub fn fit_stopwords() {
        unimplemented!()
    }

    pub fn generate(self, start_word: Option<&str>) -> String {
        assert!(self.is_fitted, "Model not fitted.");

        let mut output = String::new();

        //if start word specified in generate() params
        if let Some(mut word) = start_word {
            output.push_str(&format!("{} ", &word));

            for _ in 1..self.seq_length {
                match self.chain.get(word) {
                    Some(ngrams) => {
                        let mut rng = rand::thread_rng();
                        word = ngrams.choose(&mut rng).unwrap();
                        output.push_str(&format!("{} ", &word));
                    }
                    None => {
                        println!("!!!!{:?}===", word);
                        break;
                    }
                }
            }
        //if model has a list of fitted start words
        } else if let Some(_words) = self.startwords {
            unreachable!()
        } else {
            panic!("No start words specified.")
        }

        output + ":)"
    }
}
