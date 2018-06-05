#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate wbg_rand;

use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use wbg_rand::{Rng, wasm_rng};

#[wasm_bindgen]
pub struct Markov {
  map: HashMap<String, Vec<String>>
}

#[wasm_bindgen]
impl Markov {
  pub fn new() -> Markov {
    Markov {
      map: HashMap::new()
    }
  }
  
  pub fn parse(&mut self, sentence: &str) {
    let words = sentence.split(" ").collect::<Vec<&str>>();
    let word_count = words.len();

    for n in 0..word_count {
      if n + 2 < word_count {
        let key = format!("{} {}", words[n], words[n + 1]);
        let value = words[n + 2];

        self.insert(key, value.to_string());
      }
    }
  }
  
  pub fn generate_sentence(&self) -> String {
    let mut rng = wasm_rng();
    let keys = self.map.keys().collect::<Vec<&String>>();

    let mut key = rng.choose(&keys).expect("could not get random value").to_string();
    let mut sentence = key.clone();

    loop {
        match self.map.get(&key) {
            Some(values) => {
                let value = rng.choose(values).expect("could not get value");
                sentence = format!("{} {}", sentence, value);

                key = next_key(&key, value);
            }
            None => break
        }
    }

    sentence
  }
}

impl Markov {
    fn insert(&mut self, key: String, value: String) {
      if self.map.contains_key(&key) {
        let current_value = self.map.get_mut(&key).unwrap();
        current_value.push(value);
      } else {
        self.map.insert(key, vec![value]);
      }
  }
}

fn next_key(key: &str, value: &str) -> String {
    let last_word = key.split(" ").last().expect("could not get last word");
    format!("{} {}", last_word, value)
}