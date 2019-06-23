use crate::{
    StateMachine,
    JSPTemplateLineError,
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
use jsp::{JGraph, NIndex, Node, Regexp, jspnode, EntryType, NodeType, Metadata};
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
                                self.process_node(node, line.as_str(), &statemachine)?;
                            }

                            ParseResult::Regex(regex) => {
                                log::info!("line: {} {:?}", statemachine.line_number(), regex);
                                self.process_regex(regex)?;
                            }
                            ParseResult::Edges(edges) => {
                                log::info!("line: {} {:?}", statemachine.line_number(), edges);
                                self.process_edges(edges, line.as_str(), &statemachine)?;
                            }
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

    fn process_edges(&mut self, edges: Vec<Edge>, line: &str, statemachine: &StateMachine) -> Result<(), JSPTemplateError> {
        for edge in edges {
            let from_node = self.keymap.get(&edge.from).ok_or(
                JSPTemplateLineError::from((
                    statemachine.line_number(),
                    line.to_owned(),
                    statemachine.state().clone(),
                    JSPTemplateError::KeyMapLookupError(edge.from.clone())
                ))
            )?;
            let to_node = self.keymap.get(&edge.to).ok_or(
                JSPTemplateLineError::from((
                    statemachine.line_number(),
                    line.to_owned(),
                    statemachine.state().clone(),
                    JSPTemplateError::KeyMapLookupError(edge.to.clone())
                ))
            )?;
            self.graph.extend_with_edges(&[(from_node.clone(), to_node.clone())]);
        }
        Ok(())
    }

    fn process_node(&mut self, node: SNode, line: &str, statemachine: &StateMachine) -> Result<(), JSPTemplateError> {
        match node {
            // `rd`
            SNode::Simple(ref s) => {
                self.keymap.insert(s.clone(), self.graph.add_node(jspnode!(s.clone())));
            }
            // `rd = RD`
            SNode::Pair{ref name, ref value} => {
                self.keymap.insert(name.clone(), self.graph.add_node(jspnode!(value.clone())));
            }
            // `rd = $rd_re`
            SNode::ReVar{ref name, ref variable} => {
                let var = self.regexmap.get(variable).ok_or(
                    JSPTemplateLineError::from((
                        statemachine.line_number(),
                        line.to_owned(),
                        statemachine.state().clone(),
                        JSPTemplateError::RegexMapLookupError(variable.clone()
                    ))
                ))?;
                self.keymap.insert(
                    name.clone(), 
                    self.graph.add_node( 
                        Node::new_simple(
                            var.clone(),
                            EntryType::Directory,
                            Metadata::new()
                        )
                    )
                );
            } 
            // `rd = "[a-z]+"`
            SNode::RegexSimple{ref name, ref re} => {
                let regx = Regexp::new(format!("^{}$", re.as_str()).as_str())?;
                self.keymap.insert(
                    name.clone(), 
                    self.graph.add_node( 
                        Node::new_simple(
                            NodeType::new_regex( name.clone(), regx, None),
                            EntryType::Directory,
                            Metadata::new()
                        )
                    )
                );
            }
            // `rd = "[a-z]+" "(foo|bar)"`
            SNode::RegexComplex{ref name, ref pos, ref neg} => {
                let regx_pos = Regexp::new(format!("^{}$", pos.as_str()).as_str())?;
                let regx_neg = Regexp::new(format!("^{}$", neg.as_str()).as_str())?;
                self.keymap.insert(
                    name.clone(), 
                    self.graph.add_node( 
                        Node::new_simple(
                            NodeType::new_regex( name.clone(), regx_pos, Some(regx_neg)),
                            EntryType::Directory,
                            Metadata::new()
                        )
                    )
                );
            }
        };

        Ok(())
    }

    // match against the various flavors or regex and construct Regex objects in the regexmap store
    // these will be used in node later.
    fn process_regex(&mut self, regex: Regex)-> Result<(), JSPTemplateError> {
        match regex {

            Regex::Simple{ ref name,  ref value} => {
                let re = Regexp::new(format!("^{}$", value.as_str()).as_str())?;
                self.regexmap.insert(name.clone(), NodeType::new_regex( name.clone(), re, None));
            }

            Regex::Complex{ ref name, ref positive, ref negative} => {
                let pos_re = Regexp::new(format!("^{}$", positive.as_str()).as_str())?;
                let neg_re = Regexp::new(format!("^{}$", negative.as_str()).as_str())?;
                self.regexmap.insert(name.clone(), NodeType::new_regex(name.clone(), pos_re, Some(neg_re)));
            }
        }
        Ok(())
    }

}

