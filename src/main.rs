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
	}
}



fn main() {
	use markov::Markov;
    //use markov::Token;

	let mut markov = Markov::new(2);

	let feed = "
    La société russe Kaspersky, l’un des principaux éditeurs de logiciels antivirus au monde, a été accusée la semaine dernière dans plusieurs articles de la presse américaine d’avoir été utilisée pour espionner les agences de renseignement américaines. Entretien avec son fondateur, Eugène Kaspersky, qui nie toute collusion avec les services de renseignement russe.    

Je ne le pense pas, notre entreprise n’est pas assez grande pour être utilisée dans ces turbulences politiques. Il y a sans doute plusieurs raisons derrière ces accusations, et peut-être que la situation politique est l’une d’entre elles.
Nous n’avons aucune coopération secrète avec quelque agence de renseignement que ce soit dans le monde, ni en Russie ni ailleurs. Nous n’aidons aucun pays à faire de l’espionnage. Lors de notre recherche de fichiers malveillants [sur un ordinateur], en cas de détection de programmes dangereux qui viennent de faire leur apparition sur Internet, nous les envoyons dans le cloud [sur les serveurs de Kaspersky] si notre produit est connecté à ce service, et seulement dans ce cas. Si notre produit n’est pas connecté au cloud que nous fournissons, nous ne voyons rien.

La plupart d’entre eux sont analysés automatiquement et stockés dans nos archives. Dans certains cas, ces données arrivent dans les mains de nos chercheurs. Et ces experts ont des instructions très strictes : s’il y a la moindre information classifiée, de quelque pays que ce soit, qui a été envoyée dans le cloud, cette information est supprimée.
Les programmes malveillants ne peuvent pas être classifiés.
Je l’ai dit à de nombreuses reprises : mon entreprise est ouverte à un audit, pas seulement au code source, mais aussi aux mises à jour que nous faisons. Nous avons des requêtes de certains pays, et nous y accédons. Nous l’avons fait récemment au Brésil, en août. Ils sont venus dans nos locaux et ont inspecté notre code. Toutes les portes étaient ouvertes.

Nous travaillons activement sur un transparency center qui sera basé dans un pays d’Europe occidentale. Nous allons y mettre à disposition notre code source et nos bases de données pour inspection. Tout gouvernement ou toute organisation respectée qui auront une question sur nos produits seront les bienvenus. Ce centre prouvera que tout est en ordre dans nos produits, technologies, mises à jour, services.
Il se trouvera dans un des pays européens qui ont la législation la plus stricte en matière de données personnelles. Pour être honnête, le gagnant est votre voisin de l’Est ; l’Allemagne est bien connue pour sa législation en matière de données personnelles.
L’audition aura lieu après-demain [mercredi 25 octobre], mais nous ne sommes pas invités. Ils ne nous ont pas dit pourquoi, nous n’avons aucune information ; je ne comprends pas ce qui se passe. De grands médias américains ont publié de fausses accusations contre notre entreprise, sans aucune preuve. Cette attaque est disproportionnée par rapport à notre taille : notre part de marché n’est pas si importante, nous sommes juste une entreprise de sécurité informatique. Je ne comprends pas pourquoi on parle tant de mon entreprise, et surtout pourquoi maintenant.
Si je suis invité, j’irai.
Nous sommes très intéressés par tout fait, toute preuve, qui étaierait les accusations qui sont portées contre Kaspersky. Je suis sûr, presque à cent pour cent, qu’il n’y a aucun problème de sécurité dans mon entreprise : nous faisons des audits de sécurité, à la fois pour nos logiciels et nos mises à jour, et nous n’avons détecté aucun problème. Mais s’il y a un problème, je veux en discuter. Je ne veux pas qu’un pays soit mécontent de mon entreprise et de ses produits. Nous protégeons nos clients contre les cybermenaces, et nous voulons que tous les gouvernements, toutes les entreprises comprennent cela. Nous sommes ouverts à la discussion et aux questions. Posez-les-nous ! Montrez-nous des preuves de ce qui ne va pas avec Kaspersky.
Chaque année, nous auditons notre réseau, nous simulons des attaques. Désormais, il est impossible de pénétrer notre réseau sans être détecté. 
Nous n’avons ce genre de problème qu’aux Etats-Unis. Nous nous attendons à une croissance négative dans ce pays, mais dans le reste du monde nous sommes en croissance. Cette tension avec les Etats-Unis n’affecte pas notre activité.
Je ne veux pas m’approfondir sur les scénarios possibles. Je ne sais pas, mais la question c’est pourquoi maintenant ? Et pourquoi pas avant ? Nous travaillons sur les attaques étatiques depuis des années, d’abord il y a eu Stuxnet [logiciel malveillant ayant visé le programme nucléaire iranien], puis Duqu [autre programme lié à Stuxnet], et beaucoup d’autres, depuis près de dix ans. Nous protégeons nos clients contre tous les types d’attaques étatiques.



    ";

	markov.feed_text(feed);

    println!("\ncontent:");
	for val in markov.h.iter() {
		println!("{:?}", val);
	}


    println!("");

    //let tokens = vec![Token::Start, Token::Word("Ceci".into())];
    //let prediction = markov.predict_from_tokens(tokens);
    //println!("{:?}", prediction);


    println!("generating text...");
    for i in 0..10 {
        let generated = markov.markov();
        println!("generated: {:?}", generated);
    }
}

