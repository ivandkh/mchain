use std::collections::HashMap;
use rand::seq::SliceRandom;

pub struct Model<'a> {
    pub chain:HashMap<&'a str, Vec<&'a str>>,
    startwords: Option<Vec<&'a str>>,
    _stopwords: Option<Vec<&'a str>>,
    #[allow(dead_code)]
    seq_length: i32,
    is_fitted: bool,
}

impl<'a> Model<'a> {
    pub fn new(seq_length: i32) -> Model<'a>{
        Model{
            chain: HashMap::new(),
            startwords: None,
            _stopwords: None,
            seq_length,
            is_fitted: false,
        }
    }

    pub fn fit_ngrams(&mut self, ngrams:Vec<Vec<&'a str>> ) {

        for ngram in ngrams{
            let key = ngram[0];
            let val = Vec::from(&ngram[1..]);
            //TODO:
            //Get around this vector recreation
            self.chain.insert(key, val);
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


    pub fn generate(self, start_word: Option<&str>) -> String{

        let mut output = String::new();

        if let Some(word) = start_word{
            output.push_str(&format!("{} ", &word));

            for _ in 1..self.seq_length{
                match self.chain.get(word){
                    Some(words) => {
                        let mut rng = rand::thread_rng();
                        let word = words.choose(&mut rng).unwrap();
                        output.push_str(&format!("{} ", &word));
                    },
                    None => break
                }
            }

        }

        else if let Some(_words) = self.startwords{
            unreachable!()
        }

        else{
            panic!("No start words specified.")
        };

        output + ":)"
    }

}
