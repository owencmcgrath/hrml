use regex::Regex;

pub(crate) struct Patterns {
    pub heading: Regex,
    pub bold: Regex,
    pub underline: Regex,
    pub italic: Regex,
    pub ulist: Regex,
    pub olist: Regex,
    pub link: Regex,
    pub image: Regex,
    pub code_block: Regex,
    pub quote: Regex,
    pub hr: Regex,
    pub nested_quote: Regex,
}

impl Patterns {
    pub fn new() -> Self {
        Self {
            heading: Regex::new(r"^jf+\s+(.+)").unwrap(),
            bold: Regex::new(r"js\s*(.+?)\s*sj").unwrap(),
            italic: Regex::new(r"jd\s*(.+?)\s*dj").unwrap(),
            underline: Regex::new(r"ju\s*(.+?)\s*uj").unwrap(),
            ulist: Regex::new(r"^ja\s+(.+)").unwrap(),
            olist: Regex::new(r"^jl\s+(.+)").unwrap(),
            link: Regex::new(r"^jg\s*\[(.+?)\]\s*gh\s*\[(.+?)\]\s*hg$").unwrap(),
            image: Regex::new(r"^jh\s*\[(.+?)\]\s*gh\s*\[(.+?)\]\s*hj$").unwrap(),
            code_block: Regex::new(r"^jkd(?:\s+(\w+))?$").unwrap(),
            quote: Regex::new(r"^kl\s+(.+)").unwrap(),
            nested_quote: Regex::new(r"^kll\s+(.+)").unwrap(),
            hr: Regex::new(r"^js\s*$").unwrap(),
        }
    }
}
