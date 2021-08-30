mod mchain;
use std::fs;

fn main() {
    //Read file to String
    let text = fs::read_to_string("text1.txt").expect("Can't open file.");

    //Specify sequence length and create a Model
    let mut chain = mchain::Model::new(6);

    //Fit ngrams to the Model
    let ngrams = mchain::tools::get_ngrams(4, &text);
    chain.fit_ngrams(ngrams);

    //Optional: fit startwords to the Model
    let startwords = mchain::tools::get_startwords(&text);
    chain.fit_startwords(startwords);

    //Generate sequences
    let force_startword: Option<&String> = None;
    for _ in 0..100 {
        let output = chain.generate(force_startword);
        println!("{:?}\n", output);
    }
}
