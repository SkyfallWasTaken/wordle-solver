use dictionary::random_word;
use smallvec::SmallVec;

pub const MAX_GUESSES: i8 = 6;
pub const WORD_LENGTH: usize = 5;

#[derive(PartialEq, Debug)]
pub struct Game {
    pub word_vec: CharSmallVec,
    pub guesses: i8,
    pub completed: bool,
}

type CharSmallVec = SmallVec<[char; WORD_LENGTH]>;

#[derive(PartialEq, Debug)]
pub enum GuessError {
    WordNotValidLength,
    WordNotInList,
    AllGuessesUsed,
    AlreadyCompleted,
}

#[derive(PartialEq, Debug)]
pub enum LetterType {
    RightPlace,
    WrongPlace,
    NotInWord,
}

type GuessResult = SmallVec<[LetterType; WORD_LENGTH]>;

impl Game {
    pub fn new(word_vec: CharSmallVec) -> Game {
        Game {
            word_vec,
            guesses: 0,
            completed: false,
        }
    }

    pub fn with_random_word() -> Game {
        let word = random_word();
        let mut vec: SmallVec<[char; WORD_LENGTH]> = SmallVec::new();
        for r#char in word.chars() {
            vec.push(r#char);
        }
        Game::new(vec)
    }

    pub fn guess(&mut self, word_vec: &CharSmallVec) -> Result<GuessResult, GuessError> {
        if self.completed {
            return Err(GuessError::AlreadyCompleted);
        }
        if self.guesses == MAX_GUESSES {
            return Err(GuessError::AllGuessesUsed);
        }
        if word_vec.len() != WORD_LENGTH {
            return Err(GuessError::WordNotValidLength);
        }

        let mut result = SmallVec::with_capacity(WORD_LENGTH);
        let mut correct_letter_count: i8 = 0;

        for (count, letter) in word_vec.iter().enumerate() {
            result.push(if letter == &self.word_vec[count] {
                correct_letter_count += 1;
                LetterType::RightPlace
            } else if self.word_vec.contains(letter) {
                LetterType::WrongPlace
            } else {
                LetterType::NotInWord
            });
        }

        if correct_letter_count == WORD_LENGTH.try_into().unwrap() {
            self.completed = true;
        }

        assert_eq!(result.len(), WORD_LENGTH);

        self.guesses += 1;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smallvec::smallvec;

    fn aloft() -> CharSmallVec {
        smallvec!['a', 'l', 'o', 'f', 't']
    }

    #[test]
    fn correct() {
        let mut game = Game::new(aloft());

        assert_eq!(
            game.guess(&aloft()),
            Ok(smallvec![
                LetterType::RightPlace,
                LetterType::RightPlace,
                LetterType::RightPlace,
                LetterType::RightPlace,
                LetterType::RightPlace,
            ])
        )
    }

    #[test]
    fn wrong_place() {
        let mut game = Game::new(aloft());

        assert_eq!(
            game.guess(&smallvec!['f', 'l', 'o', 'a', 't']),
            Ok(smallvec![
                LetterType::WrongPlace,
                LetterType::RightPlace,
                LetterType::RightPlace,
                LetterType::WrongPlace,
                LetterType::RightPlace,
            ])
        )
    }

    #[test]
    fn not_in_place() {
        let mut game = Game::new(smallvec!['f', 'f', 'f', 'f', 'a']);

        assert_eq!(
            game.guess(&smallvec!['z', 'e', 'b', 'r', 'a']),
            Ok(smallvec![
                LetterType::NotInWord,
                LetterType::NotInWord,
                LetterType::NotInWord,
                LetterType::NotInWord,
                LetterType::RightPlace
            ])
        )
    }

    #[test]
    fn error_on_max_guesses() {
        let mut game = Game::new(smallvec!['f', 'f', 'f', 'f', 'a']);

        for _ in 0..MAX_GUESSES {
            game.guess(&aloft()).unwrap();
        }
        assert_eq!(game.guess(&aloft()), Err(GuessError::AllGuessesUsed))
    }
}
