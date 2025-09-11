use crate::node::Node;
use super::MarkupParser;

impl MarkupParser {
    pub(super) fn parse_inline_elements_impl(&self, line: &str) -> Option<Node> {
        if let Some(cap) = self.patterns.link.captures(line) {
            let mut node = Node::new("a");
            node.attributes.insert("href".to_string(), cap[2].trim().to_string());
            node.content = Some(cap[1].trim().to_string());
            return Some(node);
        }

        if let Some(cap) = self.patterns.image.captures(line) {
            let mut node = Node::new("img");
            node.attributes.insert("alt".to_string(), cap[1].trim().to_string());
            node.attributes.insert("src".to_string(), cap[2].trim().to_string());
            return Some(node);
        }

        None
    }

    pub(super) fn parse_inline_styles_impl(&self, text: &str) -> Vec<Node> {
        let mut nodes = Vec::new();
        let mut last_end = 0;
        let mut segments = self.collect_inline_segments(text);
        segments.sort_by_key(|s| s.0);

        for (start, end, tag, content) in segments {
            if start > last_end {
                nodes.push(self.create_text_node(&text[last_end..start]));
            }
            nodes.push(self.create_styled_node(&tag, &content));
            last_end = end;
        }

        if last_end < text.len() {
            nodes.push(self.create_text_node(&text[last_end..]));
        }

        if nodes.is_empty() {
            nodes.push(self.create_text_node(text));
        }

        nodes
    }

    fn collect_inline_segments(&self, text: &str) -> Vec<(usize, usize, String, String)> {
        let mut segments = Vec::new();

        for cap in self.patterns.bold.captures_iter(text) {
            let full_match = cap.get(0).unwrap();
            segments.push((
                full_match.start(),
                full_match.end(),
                "strong".to_string(),
                cap[1].to_string(),
            ));
        }

        for cap in self.patterns.italic.captures_iter(text) {
            let full_match = cap.get(0).unwrap();
            segments.push((
                full_match.start(),
                full_match.end(),
                "em".to_string(),
                cap[1].to_string(),
            ));
        }

        for cap in self.patterns.underline.captures_iter(text) {
            let full_match = cap.get(0).unwrap();
            segments.push((
                full_match.start(),
                full_match.end(),
                "u".to_string(),
                cap[1].to_string(),
            ));
        }

        segments
    }

    fn create_text_node(&self, content: &str) -> Node {
        let mut text_node = Node::new("text");
        text_node.content = Some(content.to_string());
        text_node
    }

    fn create_styled_node(&self, tag: &str, content: &str) -> Node {
        let mut styled_node = Node::new(tag);
        styled_node.content = Some(content.to_string());
        styled_node
    }
}
