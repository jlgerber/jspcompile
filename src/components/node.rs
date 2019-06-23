use crate::Metadata;

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    /// `rd`
    Simple(String, Option<Metadata>),

    /// `rd = RD`
    Pair{name: String, value: String, metadata: Option<Metadata>}, 

    /// `rd = $rd_re`
    ReVar{name: String, variable: String}, 

    /// `rd = "[a-z]+"`
    RegexSimple{name: String, re: String },

    /// `rd = "[a-z]+" "(foo|bar)"`
    RegexComplex{name:String, pos: String, neg: String}, 
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
