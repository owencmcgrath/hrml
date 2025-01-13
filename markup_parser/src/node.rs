use std::collections::HashMap;

//auto generate implementations for the debug and clone traits
#[derive(Debug, Clone)]
pub struct Node
{
    pub tag: String, //<a>, <div>, etc.
    pub content: Option<String>, //whatever the string is between the tags
    pub attributes: HashMap<String, String>, //href, src, etc.
    pub children: Vec<Node>, //for nested nodes
}

//implement the Node struct
impl Node
{
    pub fn new(tag: &str) -> Self
    {
        Node
        {
            tag: tag.to_string(),
            content: None,
            attributes: HashMap::new(),
            children: Vec::new(),
        }
    }
}
