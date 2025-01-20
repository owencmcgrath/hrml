use crate::node::Node;
use super::MarkupParser;
use html_escape;

impl MarkupParser {
    pub(super) fn to_html_impl(&self, nodes: &[Node]) -> String {
        let mut result = String::new();
        for node in nodes {
            match node.tag.as_str() {
                "text" => self.render_text(node, &mut result),
                "br" => result.push_str("<br>\n"),
                "hr" => result.push_str("<hr>\n"),
                "img" => self.render_img(node, &mut result),
                "a" => self.render_link(node, &mut result),
                "blockquote" => self.render_blockquote(node, &mut result),
                "ul" | "ol" => self.render_list(node, &mut result),
                "li" => self.render_list_item(node, &mut result),
                "p" => self.render_paragraph(node, &mut result),
                "pre" => self.render_pre(node, &mut result),
                _ => self.render_generic(node, &mut result),
            }
        }
        result
    }

    fn render_text(&self, node: &Node, result: &mut String) {
        if let Some(content) = &node.content {
            result.push_str(content);
        }
    }

    fn render_img(&self, node: &Node, result: &mut String) {
        result.push_str("<img");
        for (key, value) in &node.attributes {
            let trimmed_value = value.trim_matches(|c| c == '[' || c == ']');
            result.push_str(&format!(" {}=\"{}\"", key, trimmed_value));
        }
        result.push_str(">\n");
    }

    fn render_link(&self, node: &Node, result: &mut String) {
        let href = node.attributes.get("href")
            .map(|s| s.trim_matches(|c| c == '[' || c == ']'))
            .unwrap_or("");
        result.push_str(&format!("<a href=\"{}\">", href));
        if let Some(content) = &node.content {
            let trimmed_content = content.trim_matches(|c| c == '[' || c == ']');
            result.push_str(trimmed_content);
        }
        result.push_str("</a>");
    }

    fn render_blockquote(&self, node: &Node, result: &mut String) {
        result.push_str("<blockquote>");
        if let Some(content) = &node.content {
            result.push_str(content);
        }
        for child in &node.children {
            result.push_str(&self.to_html_impl(&[child.clone()]));
        }
        result.push_str("</blockquote>\n");
    }

    fn render_list(&self, node: &Node, result: &mut String) {
        result.push_str(&format!("<{}>\n", node.tag));
        for child in &node.children {
            result.push_str(&self.to_html_impl(&[child.clone()]));
        }
        result.push_str(&format!("</{}>\n", node.tag));
    }

    fn render_list_item(&self, node: &Node, result: &mut String) {
        result.push_str("<li>");
        if let Some(content) = &node.content {
            result.push_str(content);
        }
        for child in &node.children {
            result.push_str(&self.to_html_impl(&[child.clone()]));
        }
        result.push_str("</li>\n");
    }

    fn render_paragraph(&self, node: &Node, result: &mut String) {
        result.push_str("<p>");
        for child in &node.children {
            result.push_str(&self.to_html_impl(&[child.clone()]));
        }
        result.push_str("</p>\n");
    }

    fn render_pre(&self, node: &Node, result: &mut String) {
        let lang_class = node.attributes.get("class")
            .map(|c| format!(" class=\"{}\"", c))
            .unwrap_or_default();
        result.push_str(&format!("<pre{}>\n", lang_class));
        for child in &node.children {
            if child.tag == "code" {
                result.push_str("<code>");
                if let Some(content) = &child.content {
                    result.push_str(&html_escape::encode_text(content));
                }
                result.push_str("</code>");
            }
        }
        result.push_str("</pre>\n");
    }

    fn render_generic(&self, node: &Node, result: &mut String) {
        if let Some(content) = &node.content {
            result.push_str(&format!("<{}>{}</{}>\n",
                node.tag, content, node.tag));
        } else if !node.children.is_empty() {
            result.push_str(&format!("<{}>\n", node.tag));
            for child in &node.children {
                result.push_str(&self.to_html_impl(&[child.clone()]));
            }
            result.push_str(&format!("</{}>\n", node.tag));
        }
    }
}
