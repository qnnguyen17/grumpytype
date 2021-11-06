use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use rand::prelude::ThreadRng;
use rand::{self, Rng};

use crate::error::ApplicationError;
use crate::state::State;

pub struct Dictionary {
    words: Vec<String>,
    rng: ThreadRng,
}

impl Dictionary {
    pub fn from_file<P: AsRef<Path>>(
        path: P,
        min_word_len: usize,
        max_word_len: usize,
    ) -> Result<Self, ApplicationError> {
        let f = File::open(path).map_err(ApplicationError::DictionaryLoad)?;
        let reader = BufReader::new(f);
        Ok(Self {
            words: reader
                .lines()
                .filter_map(|line| match line {
                    Ok(l) if min_word_len <= l.len() && l.len() <= max_word_len => Some(l),
                    Ok(_) => None,
                    Err(_) => None,
                })
                .collect(),
            rng: rand::thread_rng(),
        })
    }

    pub fn get_random_word(&mut self) -> String {
        let rand_n = self.rng.gen_range(0..self.words.len());
        self.words[rand_n].clone()
    }

    pub fn load_words(&mut self, state: &mut State, num_words: usize) {
        let text = &mut state.all_words;
        while text.len() < num_words {
            text.push(self.get_random_word());
        }
    }
}
