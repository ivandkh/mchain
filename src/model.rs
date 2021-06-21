use std::collections::HashMap;

pub struct Model<'a> {
    chain:HashMap<&'a str, Vec<&'a str>>,
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

    pub fn fit_ngrams() {
        unimplemented!()
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
