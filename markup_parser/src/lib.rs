use wasm_bindgen::prelude::*;
mod node;
mod parser;

#[wasm_bindgen]
pub struct MarkupParserWrapper {
    parser: parser::MarkupParser,
}

#[wasm_bindgen]
impl MarkupParserWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        MarkupParserWrapper {
            parser: parser::MarkupParser::new()
        }
    }

    #[wasm_bindgen]
    pub fn parse_to_html(&self, input: &str) -> String {
        let nodes = self.parser.parse(input);
        self.parser.to_html(&nodes)
    }
}
