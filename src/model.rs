use std::collections::HashMap;

pub struct Model<'a> {
    pub chain:HashMap<&'a str, Vec<&'a str>>,
    startwords: Option<Vec<&'a str>>,
    stopwords: Option<Vec<&'a str>>,
    seq_length: i32,
    is_fitted: bool,
}

impl<'a> Model<'a> {
    pub fn new(seq_length: i32) -> Model<'a>{
        Model{
            chain: HashMap::new(),
            startwords: None,
            stopwords: None,
            seq_length,
            is_fitted: false
        }
    }

    pub fn fit_ngrams(&mut self, ngrams:Vec<Vec<&'a str>> ) {
        let n = ngrams[0].len();

        for ngram in ngrams{
            let key = ngram[0];
            let val = Vec::from(&ngram[1..]);
            //TODO:
            //Get around this vector recreation
            self.chain.insert(key, val);
        }
    }

    pub fn fit_startwords() {
        unimplemented!()
    }

    pub fn fit_stopwords() {
        unimplemented!()
    }

    pub fn generate() {
        unimplemented!()
    }

}
