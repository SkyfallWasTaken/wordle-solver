use std::collections::HashMap;

use common_macros::hash_map;
use dictionary::DICTIONARY;
use logic::*;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use std::cmp::Ordering;

pub struct Solver<'a> {
    dictionary: Vec<&'a str>,
    game: Game,
}

impl Solver<'_> {
    pub fn new(game: Game) -> Solver<'static> {
        Solver {
            dictionary: DICTIONARY.to_vec(),
            game,
        }
    }

    fn find_optimal_word(&mut self) -> &str {
        let dict = &mut self.dictionary;
        dict.par_sort_unstable_by(|a, b| {
            let score_a = score_word(a);
            let score_b = score_word(b);

            if score_a > score_b {
                Ordering::Less
            } else if score_a == score_b {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        });
        dict[0]
    }

    pub fn r#try(&mut self) -> Result<i32, GuessError> {
        let mut count = 0;
        println!("Correct word: {:?}", self.game.word_vec);
        loop {
            count += 1;
            let optimal_word = self.find_optimal_word();
            println!("Trying {optimal_word}");

            let optimal_word_vec = &optimal_word.chars().collect();
            let guess_result = match self.game.guess(optimal_word_vec) {
                Ok(vec) => vec,
                Err(e) => match e {
                    GuessError::AlreadyCompleted => return Ok(count),
                    _ => return Err(e),
                },
            };

            for (count, r#type) in guess_result.iter().enumerate() {
                let r#char = optimal_word_vec[count];
                self.dictionary = match r#type {
                    LetterType::NotInWord => (&self.dictionary)
                        .par_iter()
                        .filter(|word| !word.contains(r#char))
                        .copied()
                        .collect(),
                    LetterType::RightPlace => (&self.dictionary)
                        .par_iter()
                        .filter(|word| {
                            word.contains(r#char) && word.chars().nth(count).unwrap() == r#char
                        })
                        .copied()
                        .collect(),
                    LetterType::WrongPlace => (&self.dictionary)
                        .par_iter()
                        .filter(|word| {
                            word.contains(r#char) && word.chars().nth(count).unwrap() != r#char
                        })
                        .copied()
                        .collect(),
                };
            }

            if self.dictionary.len() == 1 {
                return Ok(count);
            }
        }
    }
}

static LETTER_FREQUENCIES: Lazy<HashMap<char, f32>> = Lazy::new(|| {
    // https://en.wikipedia.org/wiki/Letter_frequency
    hash_map! {
        'a' => 14.0,
        'b' => 2.0,
        'c' => 4.0,
        'd' => 3.8,
        'e' => 15.0,
        'f' => 1.4,
        'g' => 3.0,
        'h' => 2.3,
        'i' => 10.0,
        'j' => 0.21,
        'k' => 0.97,
        'l' => 5.3,
        'm' => 2.7,
        'n' => 7.2,
        'o' => 8.5,
        'p' => 2.8,
        'q' => 0.19,
        'r' => 7.3,
        's' => 8.7,
        't' => 6.7,
        'u' => 6.0,
        'v' => 1.0,
        'w' => 0.91,
        'x' => 0.27,
        'y' => 1.6,
        'z' => 0.44
    }
});

fn score_word(word: &str) -> f32 {
    let mut score: f32 = 0.0;
    let mut hash_map = LETTER_FREQUENCIES.clone();
    for r#char in word.chars() {
        score += hash_map.get(&r#char).unwrap_or(&-4.0);
        hash_map.remove(&r#char);
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use logic::Game;

    #[test]
    fn run() {
        let mut solver = Solver::new(Game::with_random_word());
        println!("\nGame solved in {} guesses.", solver.r#try().unwrap());
    }

    #[test]
    fn average() {
        let mut averages = Vec::with_capacity(50);
        for _ in 0..50 {
            let mut solver = Solver::new(Game::with_random_word());
            averages.push(solver.r#try().unwrap_or_else(|_| {
                println!("FAILED.");
                -i32::from(MAX_GUESSES)
            }))
        }

        let sum: i32 = averages.iter().sum();
        println!("Average guesses: {}", f64::from(sum) / (averages.len() as f64));
    }
}
