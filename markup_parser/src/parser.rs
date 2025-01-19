use regex::Regex;
use crate::node::Node;

pub struct MarkupParser {
    pub heading_regex: Regex,
    pub bold_regex: Regex,
    pub underline_regex: Regex,
    pub italic_regex: Regex,
    pub ulist_regex: Regex,
    pub olist_regex: Regex,
    pub link_regex: Regex,
    pub image_regex: Regex,
    pub code_block_regex: Regex,
    pub quote_regex: Regex,
    pub hr_regex: Regex,
    pub nested_quote_regex: Regex,
}

impl MarkupParser {
    pub fn new() -> Self {
        MarkupParser {
            heading_regex: Regex::new(r"^jf+\s+(.+)").unwrap(),
            bold_regex: Regex::new(r"js\s*(.+?)\s*sj").unwrap(),
            italic_regex: Regex::new(r"jd\s*(.+?)\s*dj").unwrap(),
            underline_regex: Regex::new(r"ju\s*(.+?)\s*uj").unwrap(),
            ulist_regex: Regex::new(r"^ja\s+(.+)").unwrap(),
            olist_regex: Regex::new(r"^jl\s+(.+)").unwrap(),
            link_regex: Regex::new(r"^jg\s*\[(.+?)\]\s*gh\s*\[(.+?)\]\s*hg$").unwrap(),
            image_regex: Regex::new(r"^jh\s*\[(.+?)\]\s*gh\s*\[(.+?)\]\s*hj$").unwrap(),
            code_block_regex: Regex::new(r"^jkd(?:\s+(\w+))?$").unwrap(),
            quote_regex: Regex::new(r"^kl\s+(.+)").unwrap(),
            nested_quote_regex: Regex::new(r"^kll\s+(.+)").unwrap(),
            hr_regex: Regex::new(r"^js\s*$").unwrap(),
        }
    }

    fn parse_inline_styles(&self, text: &str) -> Vec<Node> {
        let mut nodes = Vec::new();
        let mut last_end = 0;
        let mut segments: Vec<(usize, usize, String, String)> = Vec::new();

        //bold
        for cap in self.bold_regex.captures_iter(text) {
            let full_match = cap.get(0).unwrap();
            segments.push((
                full_match.start(),
                full_match.end(),
                "strong".to_string(),
                cap[1].to_string()
            ));
        }

        //italic
        for cap in self.italic_regex.captures_iter(text) {
            let full_match = cap.get(0).unwrap();
            segments.push((
                full_match.start(),
                full_match.end(),
                "em".to_string(),
                cap[1].to_string()
            ));
        }

        //underline
        for cap in self.underline_regex.captures_iter(text) {
            let full_match = cap.get(0).unwrap();
            segments.push((
                full_match.start(),
                full_match.end(),
                "u".to_string(),
                cap[1].to_string()
            ));
        }

        //sort segments by start index
        segments.sort_by_key(|s| s.0);

        //generate nodes
        for (start, end, tag, content) in segments {
            if start > last_end {
                let mut text_node = Node::new("text");
                text_node.content = Some(text[last_end..start].to_string());
                nodes.push(text_node);
            }

            let mut styled_node = Node::new(&tag);
            styled_node.content = Some(content);
            nodes.push(styled_node);

            last_end = end;
        }

        //add remaining text
        if last_end < text.len() {
            let mut text_node = Node::new("text");
            text_node.content = Some(text[last_end..].to_string());
            nodes.push(text_node);
        }

        //if no inline styles found, return a single text node
        if nodes.is_empty() {
            let mut text_node = Node::new("text");
            text_node.content = Some(text.to_string());
            nodes.push(text_node);
        }

        nodes
    }

    pub fn parse(&self, text: &str) -> Vec<Node> {
        let mut nodes = Vec::new();
        let mut lines = text.lines().peekable();
        let mut in_code_block = false;
        let mut code_content = String::new();
        let mut code_language = String::new();

        while let Some(line) = lines.next() {
            let line = line.trim();

            if line.is_empty() {
                nodes.push(Node::new("br"));
                continue;
            }

            // Handle code blocks first since they can contain other markup
            if in_code_block {
                if line == "dkj" {
                    let mut code_node = Node::new("pre");
                    if !code_language.is_empty() {
                        code_node.attributes.insert("class".to_string(), format!("language-{}", code_language));
                    }
                    let mut code_inner = Node::new("code");
                    code_inner.content = Some(code_content.trim().to_string());
                    code_node.children.push(code_inner);
                    nodes.push(code_node);
                    in_code_block = false;
                    code_content.clear();
                    code_language.clear();
                } else {
                    code_content.push_str(line);
                    code_content.push('\n');
                }
                continue;
            }

            // Check for code block start
            if let Some(cap) = self.code_block_regex.captures(line) {
                if let Some(lang) = cap.get(1) {
                    code_language = lang.as_str().to_string();
                }
                in_code_block = true;
                continue;
            }

            // Handle horizontal rules
            if self.hr_regex.is_match(line) {
                nodes.push(Node::new("hr"));
                continue;
            }

            // Handle headings
            if let Some(cap) = self.heading_regex.captures(line) {
                let level = line.chars().take_while(|&c| c == 'j' || c == 'f').count() - 1;
                let mut node = Node::new(&format!("h{}", level));
                node.content = Some(cap[1].trim().to_string());
                nodes.push(node);
                continue;
            }

            if let Some(cap) = self.nested_quote_regex.captures(line) {
                let mut outer_node = Node::new("blockquote");
                let mut inner_node = Node::new("blockquote");
                let quote_content = cap[1].trim();
                let inline_nodes = self.parse_inline_styles(quote_content);
                if !inline_nodes.is_empty() {
                    inner_node.children = inline_nodes;
                } else {
                    inner_node.content = Some(quote_content.to_string());
                }
                outer_node.children.push(inner_node);
                nodes.push(outer_node);
                continue;
            }

            // Handle quotes with inline styling support
            if let Some(cap) = self.quote_regex.captures(line) {
                let mut node = Node::new("blockquote");
                let quote_content = cap[1].trim();
                let inline_nodes = self.parse_inline_styles(quote_content);
                if !inline_nodes.is_empty() {
                    node.children = inline_nodes;
                } else {
                    node.content = Some(quote_content.to_string());
                }
                nodes.push(node);
                continue;
            }

            // Handle unordered lists
            if self.ulist_regex.is_match(line) {
                let mut list_node = Node::new("ul");
                let mut current_line = line;

                loop {
                    if let Some(cap) = self.ulist_regex.captures(current_line) {
                        let mut item_node = Node::new("li");
                        let content = cap[1].trim();

                        let inline_nodes = self.parse_inline_styles(content);
                        if !inline_nodes.is_empty() {
                            item_node.children = inline_nodes;
                        } else {
                            item_node.content = Some(content.to_string());
                        }
                        list_node.children.push(item_node);
                    }

                    match lines.peek() {
                        Some(next_line) if self.ulist_regex.is_match(next_line.trim()) => {
                            current_line = lines.next().unwrap().trim();
                        },
                        _ => break
                    }
                }
                nodes.push(list_node);
                continue;
            }

            if self.olist_regex.is_match(line) {
                let mut list_node = Node::new("ol");
                let mut current_line = line;

                loop {
                    if let Some(cap) = self.olist_regex.captures(current_line) {
                        let mut item_node = Node::new("li");
                        let item_content = cap[1].trim();
                        let inline_nodes = self.parse_inline_styles(item_content);
                        if !inline_nodes.is_empty() {
                            item_node.children = inline_nodes;
                        } else {
                            item_node.content = Some(item_content.to_string());
                        }
                        list_node.children.push(item_node);
                    }

                    match lines.peek() {
                        Some(next_line) if self.olist_regex.is_match(next_line.trim()) => {
                            current_line = lines.next().unwrap().trim();
                        },
                        _ => break
                    }
                }
                nodes.push(list_node);
                continue;
            }

            if let Some(cap) = self.link_regex.captures(line) {
                let mut node = Node::new("a");
                node.attributes.insert("href".to_string(), cap[2].trim().to_string());
                node.content = Some(cap[1].trim().to_string());
                nodes.push(node);
                continue;
            }

            if let Some(cap) = self.image_regex.captures(line) {
                let mut node = Node::new("img");
                node.attributes.insert("alt".to_string(), cap[1].trim().to_string());
                node.attributes.insert("src".to_string(), cap[2].trim().to_string());
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

    pub fn to_html(&self, nodes: &[Node]) -> String {
        let mut result = String::new();
        for node in nodes {
            match node.tag.as_str() {
                "text" => {
                    if let Some(content) = &node.content {
                        result.push_str(content);
                    }
                }
                "br" => result.push_str("<br>\n"),
                "hr" => result.push_str("<hr>\n"),
                "img" => {
                    result.push_str("<img");
                    for (key, value) in &node.attributes {
                        let trimmed_value = value.trim_matches(|c| c == '[' || c == ']');
                        result.push_str(&format!(" {}=\"{}\"", key, trimmed_value));
                    }
                    result.push_str(">\n");
                }
                "a" => {
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
                "blockquote" => {
                    result.push_str("<blockquote>");
                    if let Some(content) = &node.content {
                        result.push_str(content);
                    }
                    for child in &node.children {
                        result.push_str(&self.to_html(&[child.clone()]));
                    }
                    result.push_str("</blockquote>\n");
                }
                "ul" | "ol" => {
                    result.push_str(&format!("<{}>\n", node.tag));
                    for child in &node.children {
                        result.push_str(&self.to_html(&[child.clone()]));
                    }
                    result.push_str(&format!("</{}>\n", node.tag));
                }
                "li" => {
                    result.push_str("<li>");
                    if let Some(content) = &node.content {
                        result.push_str(content);
                    }
                    for child in &node.children {
                        result.push_str(&self.to_html(&[child.clone()]));
                    }
                    result.push_str("</li>\n");
                }
                "p" => {
                    result.push_str("<p>");
                    for child in &node.children {
                        result.push_str(&self.to_html(&[child.clone()]));
                    }
                    result.push_str("</p>\n");
                }
                "pre" => {
                    let lang_class = node.attributes.get("class")
                        .map(|c| format!(" class=\"{}\"", c))
                        .unwrap_or_default();
                    result.push_str(&format!("<pre{}>\n", lang_class));
                    for child in &node.children {
                        if child.tag == "code" {
                            result.push_str("<code>");
                            if let Some(content) = &child.content {
                                use html_escape::encode_text;
                                result.push_str(&encode_text(content));
                            }
                            result.push_str("</code>");
                        }
                    }
                    result.push_str("</pre>\n");
                }
                _ => {
                    if let Some(content) = &node.content {
                        result.push_str(&format!("<{}>{}</{}>\n",
                            node.tag, content, node.tag));
                    } else if !node.children.is_empty() {
                        result.push_str(&format!("<{}>\n", node.tag));
                        for child in &node.children {
                            result.push_str(&self.to_html(&[child.clone()]));
                        }
                        result.push_str(&format!("</{}>\n", node.tag));
                    }
                }
            }
        }
        result
    }
}
