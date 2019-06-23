use crate::{
    StateMachine,
    JSPTemplateLineError,
    JSPTemplateError,
    //State,
    ParseResult,
    Regex,
    Node as SNode,
    Edge,
    Metadata
};
use std::{
    io::BufRead,
    collections::HashMap,
};
use jsp::{JGraph, NIndex, User, Node, Regexp, EntryType, NodeType, Metadata as JspMetadata };
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
                                
                                // deal with root
                                if edges.len() > 0 && edges[0].from != "root" {
                                    let root = Edge::new(s!("root"), edges[0].from.clone());
                                    self.process_edges(vec![root],line.as_str(), &statemachine)?;
                                }
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
            SNode::Simple(ref name, ref metadata) => {
                //TODO:: convert to non macro to handle metadata
                //self.keymap.insert(name.clone(), self.graph.add_node(jspnode!(name.clone())));
                let entrytype = if is_volume(metadata) {EntryType::Volume} else {EntryType::Directory};
                self.keymap.insert(
                    name.clone(), 
                    self.graph.add_node( 
                        Node::new_simple(
                            NodeType::Simple(name.clone()),
                            entrytype,
                            //EntryType::Directory,
                            //JspMetadata::new()
                            new_jsp_metadata(metadata)
                        )
                    )
                );
            }
            // `rd = RD`
            SNode::Pair{ref name, ref value, ref metadata} => {
                //self.keymap.insert(name.clone(), self.graph.add_node(jspnode!(value.clone())));
                let entrytype = if is_volume(metadata) {EntryType::Volume} else {EntryType::Directory};
                self.keymap.insert(
                    name.clone(), 
                    self.graph.add_node( 
                        Node::new_simple(
                            NodeType::Simple(value.clone()),
                            //EntryType::Directory,
                            entrytype,
                            //JspMetadata::new()
                            new_jsp_metadata(metadata)
                        )
                    )
                );
            }
            // `rd = $rd_re`
            SNode::ReVar{ref name, ref variable, ref metadata} => {
                let var = self.regexmap.get(variable).ok_or(
                    JSPTemplateLineError::from((
                        statemachine.line_number(),
                        line.to_owned(),
                        statemachine.state().clone(),
                        JSPTemplateError::RegexMapLookupError(variable.clone()
                    ))
                ))?;
                let entrytype = if is_volume(metadata) {EntryType::Volume} else {EntryType::Directory};
                self.keymap.insert(
                    name.clone(), 
                    self.graph.add_node( 
                        Node::new_simple(
                            var.clone(),
                            //EntryType::Directory,
                            entrytype,
                            //JspMetadata::new()
                            new_jsp_metadata(metadata)
                        )
                    )
                );
            } 
            // `rd = "[a-z]+"`
            SNode::RegexSimple{ref name, ref re, ref metadata} => {
                let regx = Regexp::new(format!("^{}$", re.as_str()).as_str())?;
                let entrytype = if is_volume(metadata) {EntryType::Volume} else {EntryType::Directory};
                self.keymap.insert(
                    name.clone(), 
                    self.graph.add_node( 
                        Node::new_simple(
                            NodeType::new_regex( name.clone(), regx, None),
                            //EntryType::Directory,
                            entrytype,
                            //JspMetadata::new()
                            new_jsp_metadata(metadata)
                        )
                    )
                );
            }
            // `rd = "[a-z]+" "(foo|bar)"`
            SNode::RegexComplex{ref name, ref pos, ref neg, ref metadata} => {
                let regx_pos = Regexp::new(format!("^{}$", pos.as_str()).as_str())?;
                let regx_neg = Regexp::new(format!("^{}$", neg.as_str()).as_str())?;
                let entrytype = if is_volume(metadata) {EntryType::Volume} else {EntryType::Directory};
                self.keymap.insert(
                    name.clone(), 
                    self.graph.add_node( 
                        Node::new_simple(
                            NodeType::new_regex( name.clone(), regx_pos, Some(regx_neg)),
                            //EntryType::Directory,
                            entrytype,
                            //JspMetadata::new()
                            new_jsp_metadata(metadata)
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

/**
 * FUggly 
 * 
 * This bit of conversion will go away once I unify the two metadata representations
 */
fn new_jsp_metadata( meta: &Option<crate::Metadata> ) -> JspMetadata {
    let mut jspmeta = JspMetadata::new();

    if let Some(meta) = meta {

        if meta.owner().is_some() { 
            let owner = meta.owner().unwrap();
            jspmeta.set_owner(
                Some(
                    User::from(owner.to_string()) 
                )
            );
        }

        if meta.permissions().is_some() {
            let perms = meta.permissions().unwrap();
            jspmeta.set_perms(Some(perms.to_string()));
        }

        if meta.varname().is_some() {
            let varname = meta.varname().unwrap();
            jspmeta.set_varname(Some(varname.to_string()));
        }
    }
    jspmeta
}

// 
fn is_volume(meta: &Option<Metadata>) -> bool {
    if let Some(meta) = meta {
        meta.is_volume()
    } else {
        false
    }
}