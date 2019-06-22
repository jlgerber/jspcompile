
#[derive(Debug, PartialEq, Eq)]
pub struct Edge {
    pub from: String,
    pub to: String,
}

impl Edge {
    pub fn new<I>(from: I, to: I) -> Edge 
    where
        I: Into<String>
    {
        Edge{
            from: from.into(),
            to: to.into()
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Header {
    Regex,
    Node,
    Edge,
    Unknown(String),
}


#[derive(Debug, PartialEq, Eq)]
pub enum Regex {
    Simple{
        name:String, 
        value: String
    },
    Complex{
        name: String, 
        positive: String, 
        negative: String
    },
}


#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    /// `rd`
    Simple(String),

    /// `rd = RD`
    Pair{name: String, value: String}, 

    /// `rd = $rd_re`
    ReVar{name: String, variable: String}, 

    /// `rd = "[a-z]+"`
    RegexSimple{name: String, re: String },

    /// `rd = "[a-z]+" "(foo|bar)"`
    RegexComplex{name:String, pos: String, neg: String}, 
}

impl Node {

    pub fn new_pair<I>(name: I, value: I) -> Node 
    where
        I:Into<String> 
    {
        Node::Pair{
            name: name.into(),
            value: value.into()
        }
    }

    pub fn new_revar<I>(name: I, variable: I) -> Node 
    where 
        I:Into<String> 
    {
        Node::ReVar {
            name: name.into(),
            variable: variable.into()
        }
    }

    pub fn new_regexsimple<I>(name: I, re: I) -> Node 
    where 
        I:Into<String> 
    {
        Node::RegexSimple {
            name: name.into(),
            re: re.into()
        }
    }

    pub fn new_regexcomplex<I>(name: I, pos: I, neg: I) -> Node 
    where 
        I:Into<String> 
    {
        Node::RegexComplex {
            name: name.into(),
            pos: pos.into(),
            neg: neg.into(),
        }
    }

}

#[derive(Debug,PartialEq,Eq)]
pub enum ParseResult {
    Header(Header),
    Regex(Regex),
    Node(Node),
    Edges(Vec<Edge>),
    Comment(String),
    Empty,
}