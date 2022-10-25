extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;
mod tokeniser;
mod preprocessor;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn calculate(input: &str) -> String {
    let preprocessed = preprocessor::process(input.to_string());
    log_many("preprocessed: ", &preprocessed);

    let tokenised = match tokeniser::tokenise(preprocessed) {
        Ok(tokens) => tokens,
        Err(e) => {
            return format!("err: {}", e);
        },
    };

    log_many("tokensised: ", &format!("{:?}", tokenised));
    
    "".to_string()
}
