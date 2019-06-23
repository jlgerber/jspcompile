
#[derive(Debug, PartialEq, Eq)]
pub enum Header {
    Regex,
    Node,
    Edge,
    Unknown(String),
}

