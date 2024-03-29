use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;

const LENGTH: usize = 5;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn pass_gen() -> String {
    let mut random_strings: Vec<String> = Vec::new();
    for _x in 0..LENGTH {
        random_strings.push(
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(LENGTH)
                .map(char::from)
                .collect()
        )
    }
    random_strings.join("_")
}
