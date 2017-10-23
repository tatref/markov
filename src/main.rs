extern crate rand;


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

	#[derive(Eq,PartialEq,Hash,Clone,Debug)]
	pub enum Prediction {
		End,
		Word(String),
	}

	pub struct Markov {
		pub h: HashMap<Vec<Token>, HashMap<Prediction, u32>>,
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

		pub fn parse_text(&mut self, text: &str) {
			let phrases = text.split(".");

			for phrase in phrases {
                let words: Vec<Token> = Markov::get_tokens(phrase.trim());

                println!("phrase:");
                for w in words.clone() {
                    println!("{:?}", w);
                }

				for window in words.windows(self.prefix_len + 1) {
                    println!("{:?}", window);
					if let Some((last, prefix)) = window.split_last() {
                        let last = match last {
                            &Token::Start => unreachable!(),
                            &Token::End => Prediction::End,
                            &Token::Word(ref w) => Prediction::Word(w.clone()),
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


		pub fn predict_from_tokens(&mut self, prefix: Vec<Token>) -> Option<Prediction> {
            assert!(prefix.len() == self.prefix_len);

			match self.h.get(&prefix) {
				None => None,  //TODO: maybe Some(Prediction::End)
				Some(ref predictions) => {
                    println!("{:?}", predictions);

                    let mut choices: Vec<Weighted<Prediction>> = predictions.iter()
                        .map(|(word, proba)| Weighted
                             { weight: *proba, item: word.clone() })
                        .collect();
                    let wc = WeightedChoice::new(&mut choices);

                    let prediction = wc.ind_sample(&mut self.rng);

                    Some(prediction)
				},
			}
		}  // predict_from_tokens
	}
}



fn main() {
	use markov::Markov;
    use markov::Token;

	let mut markov = Markov::new(2);

	let text = "Ceci est une phrase. Ceci est un avion.\
    Ceci est une pipe. Ceci n'est pas une pipe";

	markov.parse_text(text);

    println!("\ncontent:");
	for val in markov.h.iter() {
		println!("{:?}", val);
	}


    println!("");

    let tokens = vec![Token::Start, Token::Word("Ceci".into())];
    let prediction = markov.predict_from_tokens(tokens);
    println!("{:?}", prediction);
}

