use std::collections::HashMap;

pub struct Model<'a> {
    pub chain:HashMap<&'a str, Vec<&'a str>>,
    _startwords: Option<Vec<&'a str>>,
    _stopwords: Option<Vec<&'a str>>,
    #[allow(dead_code)]
    seq_length: i32,
    is_fitted: bool,
}

impl<'a> Model<'a> {
    pub fn new(seq_length: i32) -> Model<'a>{
        Model{
            chain: HashMap::new(),
            _startwords: None,
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

    #[allow(dead_code)]
    pub fn generate() {
        unimplemented!()
    }

}
