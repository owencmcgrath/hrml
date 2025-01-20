use std::collections::HashMap;

#[derive(Clone)]
pub struct Node {
    pub tag: String,
    pub content: Option<String>,
    pub children: Vec<Node>,
    pub attributes: HashMap<String, String>,
}

impl Node {
    pub fn new(tag: &str) -> Self {
        Node {
            tag: tag.to_string(),
            content: None,
            children: Vec::new(),
            attributes: HashMap::new(),
        }
    }
}
