use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub tag: String,
    pub content: Option<String>,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(tag: &str) -> Self {
        Node {
            tag: tag.to_string(),
            content: None,
            attributes: HashMap::new(),
            children: Vec::new(),
        }
    }
}
