use super::patterns::Patterns;
use crate::node::Node;

pub struct MarkupParser {
    pub(crate) patterns: Patterns,
}

impl MarkupParser {
    pub fn new() -> Self {
        Self {
            patterns: Patterns::new(),
        }
    }

    pub fn parse(&self, text: &str) -> Vec<Node> {
        let mut nodes = Vec::new();
        let mut lines = text.lines().peekable();
        let mut in_code_block = false;
        let mut code_content = String::new();
        let mut code_language = String::new();

        while let Some(line) = lines.next() {
            let line = line.trim();

            // Skip empty lines outside code blocks
            if line.is_empty() && !in_code_block {
                nodes.push(Node::new("br"));
                continue;
            }

            // Handle code block markers without rendering them
            if self.patterns.code_block.is_match(line) {
                in_code_block = true;
                if let Some(cap) = self.patterns.code_block.captures(line) {
                    if let Some(lang) = cap.get(1) {
                        code_language = lang.as_str().to_string();
                    }
                }
                continue;
            }

            if line == "dkj" {
                if in_code_block {
                    let mut code_node = Node::new("pre");
                    if !code_language.is_empty() {
                        code_node.attributes.insert(
                            "class".to_string(),
                            format!("language-{}", code_language)
                        );
                    }
                    let mut code_inner = Node::new("code");
                    code_inner.content = Some(code_content.trim().to_string());
                    code_node.children.push(code_inner);
                    nodes.push(code_node);

                    in_code_block = false;
                    code_content.clear();
                    code_language.clear();
                }
                continue;  // Skip rendering the end marker
            }

            // Handle code block content
            if in_code_block {
                code_content.push_str(line);
                code_content.push('\n');
                continue;
            }

            // Regular parsing for non-code content
            if let Some(node) = self.parse_block_elements(line) {
                nodes.push(node);
                continue;
            }

            if let Some(node) = self.parse_lists(line, &mut lines) {
                nodes.push(node);
                continue;
            }

            if let Some(node) = self.parse_inline_elements(line) {
                nodes.push(node);
                continue;
            }

            let inline_nodes = self.parse_inline_styles(line);
            if !inline_nodes.is_empty() {
                let mut p_node = Node::new("p");
                p_node.children = inline_nodes;
                nodes.push(p_node);
            }
        }
        nodes
    }

    fn parse_block_elements(&self, line: &str) -> Option<Node> {
        self.parse_block_elements_impl(line)
    }

    fn parse_lists(
        &self,
        line: &str,
        lines: &mut std::iter::Peekable<std::str::Lines>
    ) -> Option<Node> {
        self.parse_lists_impl(line, lines)
    }

    fn parse_inline_elements(&self, line: &str) -> Option<Node> {
        self.parse_inline_elements_impl(line)
    }

    fn parse_inline_styles(&self, text: &str) -> Vec<Node> {
        self.parse_inline_styles_impl(text)
    }

    pub fn to_html(&self, nodes: &[Node]) -> String {
        self.to_html_impl(nodes)
    }
}
