use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use rand::prelude::ThreadRng;
use rand::{self, Rng};

pub struct Dictionary {
    words: Vec<String>,
    rng: ThreadRng,
}

impl Dictionary {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        Ok(Self {
            words: reader.lines().collect::<io::Result<Vec<_>>>()?,
            rng: rand::thread_rng(),
        })
    }

    pub fn get_random_word(&mut self) -> String {
        let rand_n = self.rng.gen_range(0..self.words.len());
        self.words[rand_n].clone()
    }
}
