use regex::Regex;
use crate::node::Node;

pub struct MarkupParser
{
    heading_regex: Regex,
    bold_regex: Regex,
    underline_regex: Regex,
    italic_regex: Regex,
    ulist_regex: Regex,
    olist_regex: Regex,
    link_regex: Regex,
    image_regex: Regex,
    code_block_regex: Regex,
    quote_regex: Regex,
    hr_regex: Regex,
}

impl MarkupParser
{
    pub fn new() -> Self
    {
        MarkupParser
        {
            heading_regex: Regex::new(r"(j[f]+)\s+([^\n]+)").unwrap(), //headings
            bold_regex: Regex::new(r"js\s*(.+?)\s*sj").unwrap(), //bold
            italic_regex: Regex::new(r"jd\s*(.+?)\s*dj").unwrap(), //italic
            underline_regex: Regex::new(r"ju\s*(.+?)\s*uj").unwrap(), //underline
            ulist_regex: Regex::new(r"ja\s+([^\n]+)").unwrap(), //unordered list
            olist_regex: Regex::new(r"jl\s+([^\n]+)").unwrap(), //ordered list
            link_regex: Regex::new(r"jg\s+(.+?)\s+gh\s+(.+?)\s+hg").unwrap(), //links
            image_regex: Regex::new(r"jh\s+(.+?)\s+gh\s+(.+?)\s+hj").unwrap(), //images
            code_block_regex: Regex::new(r"jkd(?:\s+(\w+))?\s*([\s\S]*?)\s*dkj").unwrap(), //code block
            quote_regex: Regex::new(r"(kl)\s+([^\n]+)").unwrap(), //quote
            hr_regex: Regex::new(r"js\s*$").unwrap(), //horizontal rule
        }
    }

    //self refers to the instance of markup parser, ampersand means reference
    //takes in a string and returns a vector of nodes
    pub fn parse(&self, text: &str) -> Vec<Node>
    {
        let mut nodes = Vec::new();
        let mut processed_text = text.to_string();

        while let Some(cap) = self.code_block_regex.captures(&processed_text)
        {
                let mut node = Node::new("code-block");
                if let Some(lang) = cap.get(1)
                {
                    node.attributes.insert("language".to_string(), lang.as_str().to_string());
                }
                node.content = Some(cap[2].to_string());
                nodes.push(node);

                let code_block_text = cap[0].to_string();
                let num_lines = code_block_text.lines().count();
                let replacement = "\n".repeat(num_lines - 1);
                processed_text = processed_text.replace(&code_block_text, &replacement);
            }

        for line in processed_text.lines()
        {
            if line.trim().is_empty()
            {
                continue;
            }

            if line.contains("jkd") || line.contains("dkj") {
                continue;
            }

            if self.hr_regex.is_match(line.trim())
            {
                nodes.push(Node::new("hr"));
                continue;
            }

            if let Some(cap) = self.heading_regex.captures(line)
            {
                let level = cap[1].len() - 1;
                let mut node = Node::new(&format!("h{}", level));
                node.content = Some(cap[2].to_string());
                nodes.push(node);
                continue;
            }

            if let Some(cap) = self.quote_regex.captures(line)
            {
                let level = cap[1].len() - 1;
                let mut node = Node::new("blockquote");
                let content = cap[2].to_string().replace(" dj", "");
                node.content = Some(content);
                node.attributes.insert("level".to_string(), level.to_string());
                nodes.push(node);
                continue;
            }

            let mut handled = false;

            if let Some(cap) = self.bold_regex.captures(line)
            {
                let mut node = Node::new("strong");
                node.content = Some(cap[1].to_string());
                nodes.push(node);
                handled = true;
            }

            if let Some(cap) = self.italic_regex.captures(line)
            {
                let mut node = Node::new("em");
                node.content = Some(cap[1].to_string());
                nodes.push(node);
                handled = true;
            }

            if let Some(cap) = self.underline_regex.captures(line)
            {
                let mut node = Node::new("u");
                node.content = Some(cap[1].to_string());
                nodes.push(node);
                handled = true;
            }

            if let Some(cap) = self.ulist_regex.captures(line)
            {
                let mut list_item = Node::new("li");
                list_item.content = Some(cap[1].to_string());

                match nodes.last_mut()
                {
                    Some(last_node) if last_node.tag == "ul" =>
                    {
                        last_node.children.push(list_item);
                    }
                    _ =>
                    {
                        let mut ul_node = Node::new("ul");
                        ul_node.children.push(list_item);
                        nodes.push(ul_node);
                    }
                }
                handled = true;
            }

            if let Some(cap) = self.olist_regex.captures(line)
            {
                let mut list_item = Node::new("li");
                list_item.content = Some(cap[1].to_string());

                match nodes.last_mut()
                {
                    Some(last_node) if last_node.tag == "ol" =>
                    {
                        last_node.children.push(list_item);
                    }
                    _ =>
                    {
                        let mut ol_node = Node::new("ol");
                        ol_node.children.push(list_item);
                        nodes.push(ol_node);
                    }
                }
                handled = true;
            }

            if let Some(cap) = self.link_regex.captures(line)
            {
                let mut node = Node::new("a");
                node.content = Some(cap[1].to_string());
                node.attributes.insert("href".to_string(), cap[2].to_string());
                nodes.push(node);
                handled = true;
            }

            if let Some(cap) = self.image_regex.captures(line)
            {
                let mut node = Node::new("img");
                node.attributes.insert("alt".to_string(), cap[1].to_string());
                node.attributes.insert("src".to_string(), cap[2].to_string());
                nodes.push(node);
                handled = true;
            }

            if !handled && !line.trim().is_empty()
            {
                let mut node = Node::new("p");
                node.content = Some(line.to_string());
                nodes.push(node);
            }
        }

        nodes
    }

pub fn to_html(&self, nodes: &[Node]) -> String
{
    let mut result = String::new();

    for node in nodes
    {
        match node.tag.as_str()
        {
            "hr" =>
            {
                result.push_str("<hr>\n");
            },
            "blockquote" =>
            {
                let level: usize = node.attributes.get("level")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1);

                for _ in 0..level
                {
                    result.push_str("<blockquote>");
                }

                result.push_str(&format!("{}", node.content.as_ref().unwrap()));

                for _ in 0..level
                {
                    result.push_str("</blockquote>");
                }
                result.push_str("\n");
            },
            "code-block" =>
            {
                if let Some(lang) = node.attributes.get("language")
                {
                    result.push_str(&format!("<pre><code class=\"language-{}\">", lang));
                }
                else
                {
                    result.push_str("<pre><code>");
                }
                result.push_str(node.content.as_ref().unwrap());
                result.push_str("</code></pre>\n");
            },
            "ul" | "ol" =>
            {
                result.push_str(&format!("<{}>\n", node.tag));
                for child in &node.children
                {
                    result.push_str("  ");
                    result.push_str(&format!("<li>{}</li>\n",
                        child.content.as_ref().unwrap()));
                }
                result.push_str(&format!("</{}>\n", node.tag));
            },
            "a" =>
            {
                let href = node.attributes.get("href").unwrap();
                let text = node.content.as_ref().unwrap();
                result.push_str(&format!("<a href=\"{}\">{}</a>\n", href, text));
            },
            "img" =>
            {
                let src = node.attributes.get("src").unwrap();
                let alt = node.attributes.get("alt").unwrap();
                result.push_str(&format!("<img src=\"{}\" alt=\"{}\"/>\n", src, alt));
            },
            _ =>
            {
                if let Some(content) = &node.content
                {
                    result.push_str(&format!("<{}>{}</{}>\n",
                        node.tag, content, node.tag));
                }
            }
        }
    }
    result
    }
}
