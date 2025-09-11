use crate::node::Node;
use super::MarkupParser;

impl MarkupParser {
    pub(super) fn parse_block_elements_impl(&self, line: &str) -> Option<Node> {
        if self.patterns.hr.is_match(line) {
            return Some(Node::new("hr"));
        }

        if let Some(cap) = self.patterns.heading.captures(line) {
            let level =
                line
                    .chars()
                    .take_while(|&c| (c == 'j' || c == 'f'))
                    .count() - 1;
            let mut node = Node::new(&format!("h{}", level));
            node.content = Some(cap[1].trim().to_string());
            return Some(node);
        }

        if let Some(cap) = self.patterns.nested_quote.captures(line) {
            let mut outer_node = Node::new("blockquote");
            let mut inner_node = Node::new("blockquote");
            let quote_content = cap[1].trim();
            inner_node.children = self.parse_inline_styles_impl(quote_content);
            outer_node.children.push(inner_node);
            return Some(outer_node);
        }

        if let Some(cap) = self.patterns.quote.captures(line) {
            let mut node = Node::new("blockquote");
            let quote_content = cap[1].trim();
            node.children = self.parse_inline_styles_impl(quote_content);
            return Some(node);
        }

        None
    }

    pub(super) fn parse_lists_impl(
        &self,
        line: &str,
        lines: &mut std::iter::Peekable<std::str::Lines>
    ) -> Option<Node> {
        if self.patterns.ulist.is_match(line) {
            Some(self.parse_list_items(line, lines, "ul", &self.patterns.ulist))
        } else if self.patterns.olist.is_match(line) {
            Some(self.parse_list_items(line, lines, "ol", &self.patterns.olist))
        } else {
            None
        }
    }

    fn parse_list_items(
        &self,
        line: &str,
        lines: &mut std::iter::Peekable<std::str::Lines>,
        list_type: &str,
        pattern: &regex::Regex
    ) -> Node {
        let mut list_node = Node::new(list_type);
        let mut current_line = line;

        loop {
            if let Some(cap) = pattern.captures(current_line) {
                let mut item_node = Node::new("li");
                let content = cap[1].trim();
                item_node.children = self.parse_inline_styles_impl(content);
                list_node.children.push(item_node);
            }

            match lines.peek() {
                Some(next_line) if pattern.is_match(next_line.trim()) => {
                    current_line = lines.next().unwrap().trim();
                }
                _ => {
                    break;
                }
            }
        }
        list_node
    }
}
