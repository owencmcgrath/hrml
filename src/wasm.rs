// wasm.rs
use wasm_bindgen::prelude::*;
use web_sys::console;  // For console logging
use crate::parser::MarkupParser;

#[wasm_bindgen]
pub struct MarkupParserWrapper {
    parser: MarkupParser,
}

#[wasm_bindgen]
impl MarkupParserWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();

        console::log_1(&"Initializing MarkupParserWrapper".into());

        Self {
            parser: MarkupParser::new()
        }
    }

    #[wasm_bindgen]
    pub fn parse_to_html(&self, input: &str) -> String {
        if input.trim().is_empty() {
            return String::new();
        }

        console::log_1(&format!("Parsing input: '{}'", input).into());

        let nodes = self.parser.parse(input);
        let html = self.parser.to_html(&nodes);

        console::log_1(&format!("Generated HTML: '{}'", html).into());

        html
    }

    #[wasm_bindgen]
    pub fn test_parser(&self) -> String {
        let test_cases = vec![
            "js bold text sj",
            "ja list item",
            "kl quote"
        ];

        let mut results = String::new();
        for test in test_cases {
            let html = self.parse_to_html(test);
            results.push_str(&format!("Input: {}\nOutput: {}\n\n", test, html));
        }
        results
    }
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
