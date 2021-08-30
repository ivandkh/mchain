mod model;
mod tools;
use std::fs;

fn main() {

    //Read file to String
    let text = fs::read_to_string("text1.txt").expect("Can't open file.");

    //Specify sequence length and create a Model
    let mut chain = model::Model::new(6);

    //Fit ngrams to the Model
    let ngrams = tools::get_ngrams(4, &text);
    chain.fit_ngrams(ngrams);

    //Optional: fit start and stopwords to the Model
    let startwords = tools::get_startwords(&text);
    let stopwords = tools::get_stopwords(&text);
    chain.fit_startwords(startwords);
    chain.fit_stopwords(stopwords);

    //Generate sequences
    let startwith: Option<&String> = None;
    for _ in 0..100 {
        let output = chain.generate(startwith);
        println!("{:?}\n", output);
    }
}
