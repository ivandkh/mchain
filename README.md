# MChain
Simple Markov chain written in Rust.

# Usage
```rust
//Specify sequence length and create a Model
let mut chain = model::Model::new(6);

//Fit ngrams to the Model
let text = fs::read_to_string("text.txt").expect("Can't open file.");
let ngrams = tools::get_ngrams(4, &text);
chain.fit_ngrams(ngrams);

 //Optional: fit start and stopwords to the Model
 let startwords = tools::get_startwords(&text);
 chain.fit_startwords(startwords);


  //Generate sequences
  let startwith: Option<&String> = None;
  for _ in 0..100 {
      let output = chain.generate(startwith);
      println!("{:?}\n", output);
  }
```
