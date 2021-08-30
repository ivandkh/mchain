# MChain
Simple Markov chain written in Rust.

# Usage
```rust
//Specify sequence length and create a Model
let mut chain = mchain::Model::new(6);

//Fit ngrams to the Model
chain.fit_ngrams(ngrams);
//Optional: fit start/stop words to the Model
chain.fit_startwords(startwords);
chain.fit_stopwords(stopwords);

//Generate sequences
let startwith: Option<&String> = None;
for _ in 0..100 {
  let output = chain.generate(startwith);
  println!("{:?}\n", output);
}
```

You can fit any start/stop words and ngrams or build default ones with `tools` module.
```rust
let ngrams = mchain::tools::get_ngrams(4, &text);
let startwords = mchain::tools::get_startwords(&text);
let stopwords = mchain::tools::get_stopwords(&text);
```