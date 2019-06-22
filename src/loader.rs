use crate::{
    StateMachine,
    //JSPTemplateLineError,
    JSPTemplateError,
    State,
    ParseResult,
    Regex,
    Node as SNode,
    Edge,

};
use std::{
    io::BufRead,
    collections::HashMap,
};
use jsp::{JGraph, NIndex, Node, Regexp, jspnode, EntryType, NodeType};
use log;

#[macro_use]
pub mod macros {
    macro_rules! s {
        ($val: expr) => {
            $val.to_string();
        }
    }
}

pub type JGraphKeyMap = HashMap<String, NIndex>;
pub type RegexMap     = HashMap<String, NodeType>;

pub struct Loader<'a> {
    graph: &'a mut JGraph,
    keymap: &'a mut JGraphKeyMap,
    regexmap: &'a mut RegexMap,
}

impl<'a> Loader<'a> {
    pub fn new(graph: &'a mut JGraph, keymap: &'a mut JGraphKeyMap, regexmap: &'a mut RegexMap) -> Self {
        // add in the root node
        keymap.insert(s!("root"), graph.add_node(Node::new_root()));

        Self {
            graph, keymap, regexmap
        }
    }

    /// Load 
    pub fn load<R>(&mut self, reader: R) -> Result<(), JSPTemplateError> 
    where
        R: BufRead
    {
        let mut statemachine = StateMachine::new();
        for line in reader.lines() {
            if let Ok(line) = line {
                match statemachine.parse(&line) {
                    Ok(v) => {
                        match v {
                            ParseResult::Empty => {}
                            ParseResult::Header(header) => {log::info!("line: {} {:?}", statemachine.line_number(), header)}

                            ParseResult::Comment(comment) =>{log::debug!("line: {} {}", statemachine.line_number(), comment)}

                            ParseResult::Node(node) => {
                                log::info!("line: {} {:?}", statemachine.line_number(), node);
                                self.process_node(node)?;
                            }

                            ParseResult::Regex(regex) => {
                                log::info!("line: {} {:?}", statemachine.line_number(), regex);
                                self.process_regex(regex)?;
                            }
                            ParseResult::Edges(edges) => {log::info!("line: {} {:?}", statemachine.line_number(), edges)}
                            _ => println!("line: {} {:?}",statemachine.line_number(), v)
                        }
                    },
                    Err(e) => {
                        return Err(JSPTemplateError::from(e))
                    },
                }
            } 
        }
        Ok(())
    }

    /*
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
    */
    fn process_node(&mut self, node: SNode) -> Result<(), JSPTemplateError> {
        match node {
            SNode::Simple(ref s) => {

                self.keymap.insert(s.clone(), self.graph.add_node(jspnode!(s.clone())));
            }

            SNode::Pair{ref name, ref value} => {}
            SNode::ReVar{ref name, ref variable} => {}
            SNode::RegexSimple{ref name, ref re} => {}
            SNode::RegexComplex{ref name, ref pos, ref neg} => {}
        };

        Ok(())
    }

    // match against the various flavors or regex and construct Regex objects in the regexmap store
    // these will be used in node later.
    fn process_regex(&mut self, regex: Regex)-> Result<(), JSPTemplateError> {
        match regex {
            Regex::Simple{ ref name,  ref value} => {
                let re = Regexp::new(value.as_str())?;
                self.regexmap.insert(name.clone(), NodeType::new_regex( name.clone(), re, None));
            }
            Regex::Complex{ ref name, ref positive, ref negative} => {
                let pos_re = Regexp::new(positive.as_str())?;
                let neg_re = Regexp::new(negative.as_str())?;
                self.regexmap.insert(name.clone(), NodeType::new_regex(name.clone(), pos_re, Some(neg_re)));
            }
        }
        Ok(())
    }

}

