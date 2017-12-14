extern crate rand;
extern crate glob;


#[allow(unused_variables)]
#[allow(dead_code)]

mod markov {
	use std::collections::HashMap;
    use rand::distributions::{WeightedChoice,Weighted};
    use rand::{ThreadRng,thread_rng};
    use rand::distributions::IndependentSample;


	#[derive(Eq,PartialEq,Hash,Clone,Debug)]
	pub enum Token {
		Start,
		End,
		Word(String),
	}

	pub struct Markov {
		pub h: HashMap<Vec<Token>, HashMap<Token, u32>>,
		pub prefix_len: usize,
        rng: ThreadRng,
	}

	impl Markov {
		pub fn new(prefix_len: usize) -> Self {
			let h = HashMap::new();
            let rng = thread_rng();

			Markov {
				h,
				prefix_len,
                rng,
			}
		}

        fn get_tokens(phrase: &str) -> Vec<Token> {
            let mut words = phrase
                .split_whitespace()
                .map(|w| Token::Word(String::from(w)))
                .collect::<Vec<_>>();
            words.insert(0, Token::Start);
            words.push(Token::End);

            words
        }


        /// Feed the Markov chain some raw text
        /// Will split phrases on "."
		pub fn feed_text(&mut self, text: &str) {
			let phrases = text.split(|c| c == '.' || c == '!' || c == '?');

			for phrase in phrases {
                let words: Vec<Token> = Markov::get_tokens(phrase.trim());

                //println!("phrase:");
                //for w in words.clone() {
                //    println!("{:?}", w);
                //}

				for window in words.windows(self.prefix_len + 1) {
                    //println!("{:?}", window);
					if let Some((last, prefix)) = window.split_last() {
                        let last = match last {
                            &Token::Start => unreachable!(),
                            &Token::End => Token::End,
                            &Token::Word(ref w) => Token::Word(w.clone()),
                        };

                        let predictions = self.h.entry(prefix.into()).or_insert(HashMap::new());

                        let proba = predictions.entry(last).or_insert(0);
                        *proba += 1;
					}
					else {
                        // as long as the window is not empty, we can split
						unreachable!();
					}
				}
			}
		}

        /// Main function here
        /// Generate text from nothing
        pub fn markov(&mut self) -> String {
            let mut result: Vec<Token> = Vec::new();

            let mut starters: Vec<_> = self.h
                .iter()
                .filter(|&(prefix, proba)|
                        prefix[0] == Token::Start)
                .map(|(prefix, predictions)| {
                    //sum all probabilities for given prefix
                    let mut total = 0;
                    for (prediction, proba) in predictions.iter() {
                        total += *proba;
                    }
                    Weighted {
                        weight: total, item: prefix.clone() }
                }
                )
                .collect();
            let starters_choice = WeightedChoice::new(&mut starters);

            let mut current_tokens = starters_choice.ind_sample(&mut self.rng);
            result.extend(current_tokens.clone());

            while let Some(next_word) = self.predict_from_tokens(current_tokens.clone()) {
                // push next_word to result
                result.push(next_word.clone());

                // shift current_tokens
                let _first = current_tokens.remove(0);

                // append next_word
                current_tokens.push(next_word);
            }

            let vec_result: Vec<_> = result.iter()
                .map(|prediction|
                     match prediction {
                         &Token::Word(ref w) => w.clone(),
                         &Token::End => String::from("."),
                         &Token::Start => String::new(),
                     })
                .collect();
            vec_result.join(" ")
        }

        /// Generate the next token, from a given prefix
        /// Will panic if prefix.len() != self.prefix_len
		pub fn predict_from_tokens(&mut self, prefix: Vec<Token>) -> Option<Token> {
            assert!(prefix.len() == self.prefix_len);

			match self.h.get(&prefix) {
				None => None,  //TODO: maybe Some(Token::End)
				Some(ref predictions) => {
                    //println!("prefix={:?}  => {:?}", prefix, predictions);

                    let mut choices: Vec<Weighted<Token>> = predictions.iter()
                        .map(|(word, proba)| Weighted
                             { weight: *proba, item: word.clone() })
                        .collect();
                    let wc = WeightedChoice::new(&mut choices);

                    let prediction = wc.ind_sample(&mut self.rng);

                    Some(prediction)
				},
			}
        }  // predict_from_tokens


        pub fn load_dir(&mut self) {
            use std::io::prelude::*;
            use std::io::BufReader;
            use std::fs::File;
            use std::path::Path;

            fn load_file(f: &Path) -> String {
                let f = File::open(f).unwrap();
                let mut reader = BufReader::new(f);
                let mut buffer = String::new();

                reader.read_to_string(&mut buffer).expect("Unable to read file");

                buffer
            }


            use glob::glob;
            for entry in glob("feed/*.txt").unwrap() {
                match entry {
                    Ok(ref path) => {
                        println!("{:?}", path);
                        let feed = load_file(&path);

                        self.feed_text(&feed);
                    },
                    Err(e) => println!("{:?}", e),
                }
            }

        }
    }

}



fn main() {
	use markov::Markov;


	let mut markov = Markov::new(3);
    markov.load_dir();

    println!("\nElements in map: {}:", markov.h.len());
    println!("");

    println!("Generating text...");
    for _ in 0..10 {
        let generated = markov.markov();
        println!("generated: {:?}", generated);
    }
}

