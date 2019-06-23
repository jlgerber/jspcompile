use crate::Metadata;

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    /// `rd`
    Simple(String, Option<Metadata>),

    /// `rd = RD`
    Pair{name: String, value: String, metadata: Option<Metadata>}, 

    /// `rd = $rd_re`
    ReVar{name: String, variable: String, metadata: Option<Metadata>}, 

    /// `rd = "[a-z]+"`
    RegexSimple{name: String, re: String, metadata: Option<Metadata> },

    /// `rd = "[a-z]+" "(foo|bar)"`
    RegexComplex{name:String, pos: String, neg: String, metadata: Option<Metadata>}, 
}

impl Node {
    pub fn new_simple<I>(name: I, metadata: Option<Metadata>) -> Node 
    where  
        I: Into<String>
    {
        Node::Simple(name.into(), metadata)
    }

    pub fn new_pair<I>(name: I, value: I, metadata: Option<Metadata>) -> Node 
    where
        I:Into<String> 
    {
        Node::Pair{
            name: name.into(),
            value: value.into(),
            metadata,
        }
    }

    pub fn new_revar<I>(name: I, variable: I, metadata: Option<Metadata>) -> Node 
    where 
        I:Into<String> 
    {
        Node::ReVar {
            name: name.into(),
            variable: variable.into(),
            metadata
        }
    }

    pub fn new_regexsimple<I>(name: I, re: I, metadata: Option<Metadata>) -> Node 
    where 
        I:Into<String> 
    {
        Node::RegexSimple {
            name: name.into(),
            re: re.into(),
            metadata
        }
    }

    pub fn new_regexcomplex<I>(name: I, pos: I, neg: I, metadata: Option<Metadata>) -> Node 
    where 
        I:Into<String> 
    {
        Node::RegexComplex {
            name: name.into(),
            pos: pos.into(),
            neg: neg.into(),
            metadata
        }
    }

}
