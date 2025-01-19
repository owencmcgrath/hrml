mod node;
mod parser;

use parser::MarkupParser;

fn test_regex_patterns(parser: &MarkupParser) {
    let test_cases = vec![
        // Basic formatting
        "js bold text sj",
        "jd italic text dj",
        "ju underlined text uj",

        // Lists
        "ja list item",
        "jl numbered item",

        // Block elements
        "kl quote",
        "js",  // horizontal rule

        // Links and images
        "jg Link text gh https://example.com hg",
        "jh Image alt gh https://example.com/img.jpg hj",

        // Code blocks
        "jkd python\nprint('hello')\ndkj",

        // Malformed inputs
        "js unclosed bold",
        "jk invalid tag",
        "kl quote without content"
    ];

    println!("\nTesting individual patterns:");
    println!("----------------------------");

    for test in test_cases {
        println!("\n=== Test Case: {}", test);
        println!("--- Regex Matches:");

        if let Some(cap) = parser.bold_regex.captures(test) {
            println!("  Bold: {}", &cap[1]);
        }
        if let Some(cap) = parser.italic_regex.captures(test) {
            println!("  Italic: {}", &cap[1]);
        }
        if let Some(cap) = parser.underline_regex.captures(test) {
            println!("  Underline: {}", &cap[1]);
        }
        if let Some(cap) = parser.ulist_regex.captures(test) {
            println!("  Unordered list: {}", &cap[1]);
        }
        if let Some(cap) = parser.olist_regex.captures(test) {
            println!("  Ordered list: {}", &cap[1]);
        }
        if let Some(cap) = parser.quote_regex.captures(test) {
            println!("  Quote: {}", &cap[1]);
        }
        if let Some(cap) = parser.link_regex.captures(test) {
            println!("  Link text: {}, URL: {}", &cap[1], &cap[2]);
        }
        if let Some(cap) = parser.image_regex.captures(test) {
            println!("  Image alt: {}, URL: {}", &cap[1], &cap[2]);
        }
        if let Some(cap) = parser.code_block_regex.captures(test) {
            println!("  Code block language: {:?}", cap.get(1).map(|m| m.as_str()));
        }
        if parser.hr_regex.is_match(test) {
            println!("  Horizontal rule");
        }

        println!("--- HTML Output:");
        println!("{}", parser.to_html(&parser.parse(test)));
        println!("=== End Test Case\n");
    }
}

fn test_full_document(parser: &MarkupParser) {
    let test_markup = r#"jf Main Title

jff Subtitle with js bold sj text

ja First list item with jd italic dj
ja Second item with ju underline uj
ja Third item with js bold sj and jd italic dj

jl First ordered with jg link gh https://example.com hg
jl Second ordered with jh image gh https://example.com/img.jpg hj
jl Third ordered plain

js

kl This is a blockquote with js bold sj text

jkd rust
fn main() {
    println!("Hello, World!");
}
dkj

jkd python
def hello():
    print("Hello!")
dkj

Mixed formatting in a paragraph: js bold sj and jd italic dj and ju underline uj.

This is a paragraph with jg a link gh https://example.com hg in it.
This is a paragraph with jh an image gh https://example.com/img.jpg hj in it.

js

This is a regular paragraph.
It should be wrapped in p tags.

jf Conclusion
Thanks for reading!"#;

    println!("\nTesting full document:");
    println!("----------------------");
    let nodes = parser.parse(test_markup);
    println!("{}", parser.to_html(&nodes));
}

fn test_edge_cases(parser: &MarkupParser) {
    let edge_cases = vec![
        // Empty and whitespace
        "",
        "   ",
        "\n\n\n",

        // Malformed tags
        "js no closing tag",
        "jd incomplete dj extra",
        "ju  uj", // empty content

        // Invalid combinations
        "js jd nested italic dj sj",
        "kl quote with js unclosed bold",

        // Multiple identical tags
        "js bold sj js another bold sj",

        // Lists with formatting
        "ja list with js bold sj text",
        "jl list with jd italic dj text",

        // Code blocks with formatting
        "jkd python\njs this should not be bold sj\ndkj"
    ];

    println!("\nTesting edge cases:");
    println!("------------------");

    for (i, test) in edge_cases.iter().enumerate() {
        println!("\n=== Edge Case {}: {}", i + 1, test);
        println!("--- HTML Output:");
        println!("{}", parser.to_html(&parser.parse(test)));
        println!("=== End Edge Case\n");
    }
}

fn main() {
    let parser = MarkupParser::new();

    // Test each feature individually
    println!("=== Basic Elements Test ===\n");
    let basic_tests = vec![
        // Quotes
        "kl A simple quote",
        "kl A quote with js bold text sj",

        // Links
        "jg Click here gh https://example.com hg",

        // Images
        "jh Alt text gh https://example.com/image.jpg hj",

        // Code blocks
        "jkd python\nprint('Hello')\ndkj",

        // Lists with formatting
        "ja List item with js bold sj text",
        "jl Numbered item with jd italic dj text"
    ];

    for test in basic_tests {
        println!("Input:\n{}\n", test);
        println!("Output:");
        println!("{}\n", parser.to_html(&parser.parse(test)));
        println!("-----------------\n");
    }

    // Test combined elements
    println!("\n=== Combined Elements Test ===\n");
    let combined_test = r#"kl A quote with js bold sj and jd italic dj text

ja First item with js bold sj
ja Second item with jd italic dj

jkd python
print('Hello')
# Code comment
print('World')
dkj

jg Link gh https://example.com hg with js bold sj text"#;

    println!("Input:\n{}\n", combined_test);
    println!("Output:");
    println!("{}", parser.to_html(&parser.parse(combined_test)));
}
