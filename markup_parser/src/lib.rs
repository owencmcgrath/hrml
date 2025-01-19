use wasm_bindgen::prelude::*;
use web_sys::console;  // Add this for browser console logging
mod node;
mod parser;

// Enable better panic messages in debug builds
#[cfg(debug_assertions)]
extern crate console_error_panic_hook;

#[wasm_bindgen]
pub struct MarkupParserWrapper {
    parser: parser::MarkupParser,
}

#[wasm_bindgen]
impl MarkupParserWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Set panic hook for better error messages
        #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();

        console::log_1(&"Initializing MarkupParserWrapper".into());
        MarkupParserWrapper {
            parser: parser::MarkupParser::new()
        }
    }

    #[wasm_bindgen]
    pub fn parse_to_html(&self, input: &str) -> String {
        console::log_1(&format!("Parsing input: '{}'", input).into());

        // Ensure input isn't empty
        if input.trim().is_empty() {
            return String::new();
        }

        let nodes = self.parser.parse(input);
        let html = self.parser.to_html(&nodes);

        console::log_1(&format!("Generated HTML: '{}'", html).into());
        html
    }

    // Add a test method
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
