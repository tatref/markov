extern crate rand;


#[allow(unused_variables)]
#[allow(dead_code)]

mod markov {
	use std::collections::HashMap;


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
		pub h: HashMap<Vec<Token>, HashMap<Token, f32>>,
		prefix_len: usize,
	}

	impl Markov {
		pub fn new(prefix_len: usize) -> Self {
			let h = HashMap::new();

			Markov {
				h,
				prefix_len,
			}
		}

		pub fn parse_text(&mut self, text: &str) {
			let phrases = text.split(".");

			for phrase in phrases {
				let mut words = phrase
                    .split_whitespace()
                    .map(|w| Token::Word(String::from(w)))
                    .collect::<Vec<_>>();
                words.insert(0, Token::Start);
                words.push(Token::End);

                for w in words.clone() {
                    println!("{:?}", w);
                }

				for window in words.windows(self.prefix_len) {
                    println!("{:?}", window);
					if let Some((last, prefix)) = window.split_last() {
						//println!("{:?} -> {}", prefix, last);

						let values = self.h.entry(prefix.into()).or_insert(HashMap::new());
                        let counter = values.entry(last.clone()).or_insert(0.);
                        *counter += 1.;
					}
					else {
						unreachable!();
					}
				}
			}
		}

		fn predict(&self, prefix: Vec<Token>) -> Option<Prediction> {
            assert!(prefix.len() == self.prefix_len);

			match self.h.get(&prefix) {
				None => None,
				Some(ref predictions) => {
                    predictions.aaa();
                    None

					//match suffix[0] {
                    //    Token::Start => unreachable!(),
                    //    Token::End => Some(Prediction::End),
                    //    Token::Word(ref w) => Some(Prediction::Word(w.clone())),
                    //}
				},
			}
		}
	}
}



fn main() {
	use markov::Markov;

	let mut markov = Markov::new(3);

	let text = "Ceci est une phrase. Ceci est un avion.\
    Ceci est une pipe. Ceci c'est pas une pipe";

	markov.parse_text(text);

    println!("\ncontent:");
	for val in markov.h.iter() {
		println!("{:?}", val);
	}


}

