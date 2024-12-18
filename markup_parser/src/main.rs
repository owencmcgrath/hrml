mod node;
mod parser;

use parser::MarkupParser;

fn main() {
    let parser = MarkupParser::new();

    let test_markup = r#"jf Main Title

jff Subtitle

ja First item
ja Second item
ja Third item

jl First ordered item
jl Second ordered item
jl Third ordered item

js

jh Cute cat gh https://example.com/cat.jpg hj

jg Visit my website gh https://example.com hg

jkd rust
fn main() {
    println!("Hello, World!");
}
dkj

jkd python
def hello():
    print("Hello!")
dkj

jd This text should be italic dj
js This text should be bold sj
ju This text should be underlined uj

jd Important quote coming up! dj

jd A single quote dj
jdd A nested quote dj
jddd A deeply nested quote dj

js

This is a regular paragraph.
It should be wrapped in p tags.

jf Conclusion
Thanks for reading!"#;

    let nodes = parser.parse(test_markup);
    println!("Generated HTML:");
    println!("{}", parser.to_html(&nodes));
}
