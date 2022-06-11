use rand::seq::SliceRandom;

mod dict;
pub use dict::DICTIONARY;

pub fn random_word() -> &'static str {
    let mut rng = rand::thread_rng();
    let dict = DICTIONARY;
    dict.choose(&mut rng).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn generates_word() {
        dbg!(random_word());
    }
}
