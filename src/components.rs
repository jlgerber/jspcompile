
pub mod edge;
pub use edge::Edge;
pub mod header;
pub use header::Header;
pub mod regex;
pub use regex::Regex;
pub mod node;
pub use node::Node;
pub mod metadata;
pub use metadata::Metadata;

#[derive(Debug,PartialEq,Eq)]
pub enum ParseResult {
    Header(Header),
    Regex(Regex),
    Node(Node),
    Edges(Vec<Edge>),
    Comment(String),
    Empty,
}