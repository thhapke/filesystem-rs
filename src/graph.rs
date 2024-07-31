
use std::fmt;
use std::collections::HashMap;

use colored::Colorize;

const DOWNRIGHT: &str = "├──";
const DOWN: &str =  "│  ";
const TURN_RIGHT: &str =  "└──";
//const RIGHT: &str = "─";
const EMPTY: &str = "   ";

pub struct Node <T> {
    pub id: usize,
    pub name: String,
    pub label: String, 
    pub content: T,
}

impl <T> Node <T>{
    pub fn new(id: usize, name: &str, label: &str, content: T) -> Self {
        Self {
            id: id,
            name: name.to_string(),
            label: label.to_string(),
            content: content,
        }
    }
}
impl <T> fmt::Display for Node <T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",self.name)
    }
}

#[derive(Debug)]
pub struct GraphError {
    details: String,
}

impl GraphError {
    pub fn new(msg:String) -> GraphError {
        GraphError {details: msg}
    }
}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Node: {}", self.details)
    }
}

pub trait GraphBuilder <T> {
    fn build_graph(&self) -> Graph<T>;
}

pub struct Graph <T> {
    pub sources: Option<Vec<usize>>,
    pub nodes: Vec<Node<T>>,
    pub next_nodes: HashMap<usize, Vec<usize>>,
    pub byname: HashMap<String, usize>,
    pub max_display_level: usize,
}

impl<T> Graph <T>{
    pub fn new() -> Self {
        Self {
            sources:None,
            nodes: Vec::<Node<T>>::new(),
            next_nodes: HashMap::<usize,Vec<usize>>::new(),
            byname: HashMap::<String,usize>::new(),
            max_display_level: 10,
        }
    }
    pub fn set_max_display_level(&mut self, max_level: &usize) {
        self.max_display_level = max_level.clone();
    }

    pub fn add_node(&mut self, name: &str, label: &str, content:T) -> usize {
        let new_id = self.nodes.len();
        let node: Node<T> = Node::new(new_id, &name, &label, content);
        self.nodes.push(node);
        self.byname.insert(name.to_string(), new_id);
        let n_ids: Vec<usize> = Vec::new();
        self.next_nodes.insert(new_id, n_ids);
        new_id
        //new_id
    }

    pub fn add_edge_byname(&mut self, name1: &str, name2: &str ) -> Result<(),GraphError> {
        match self.byname.get(name1) {
            None => Err(GraphError::new(format!("{} not in graph. Cannot add edge.",name1))),
            Some(id1) => {
                match self.next_nodes.get_mut(id1) {
                    None => Err(GraphError::new(format!("{} not in edges of graph. Cannot add edge",name1))),
                    Some(from_node) =>  {
                        match self.byname.get(name2) {
                            None => Err(GraphError::new(format!("{} not in graph. Cannot add edge",name2))),
                            Some(id2) => {
                                from_node.push(id2.clone());
                                Ok(())
                            },
                        }
                    }
                }
            }
        }
    }

    pub fn to_name(&self, id: usize) -> &str {
        &self.nodes[id].name
    }
    
    pub fn find_sources(&mut self) -> &mut Self {
        let mut sources = Vec::<usize>::new();
        for n in 0..self.nodes.len() {
            let mut found: bool = false;
            for n1 in 0..self.nodes.len() {
                if let Some(nn) = self.next_nodes.get(&n1) {
                    if nn.contains(&n) {
                        found = true;
                        break;
                    }
                }
            }
            if !found { sources.push(n);}
        }
        self.sources = Some(sources);
        self
    }

    pub fn add_sources(&mut self, node_id: usize) -> &mut Self {
        self.sources = Some(vec![node_id]);
        self
    }

    pub fn add_root(&mut self, root_name: &str, content:T) -> usize {
        let mut root_id: usize = 0;
        if let Some(sources) = &self.sources.clone() {
            root_id = self.add_node(root_name, root_name, content).clone();
            let nn = self.next_nodes.get_mut(&root_id).unwrap();
            for s in sources.clone() {
                nn.push(s);
            }
        }
        self.sources = Some(vec![root_id]);
        root_id
    }  

    pub fn print_edges(&self) {
        println!("Edges of graph");
        for (node_id, to_nodes) in self.next_nodes.iter()  {
            if let Some(node) = self.nodes.get(node_id.clone()) {
                println!("{}: ",node.label);
                for nn in to_nodes {
                    if let Some(next_node) = self.nodes.get(nn.clone()) {
                        println!("-> {}: ",next_node.label);
                    }
                }
            }
        }
    }

    pub fn print_sources(&self) {
        println!("Sources of graph:");
        if let Some(sources) = &self.sources {
            for s in sources.clone() {
                println!("{}",self.nodes[s].name)
            }
        }
    }

    pub fn recursive_write(&self, f: &mut fmt::Formatter, id:&usize, prefix:&str, prefix_node: &str, depth: u32)  -> fmt::Result {
        if depth <= self.max_display_level as u32 {
            if let Some(node) = self.nodes.get(id.clone()) {
            
                write!(f, "{}", prefix.blue())?;
                write!(f, "{}", &node.label.cyan())?;
                writeln!(f, "")?;

                if let Some(nn) = self.next_nodes.get(&id) {
                    if let Some((last_node, nnodes)) = nn.split_last() {
                        let rp = prefix_node.to_string() + DOWNRIGHT;
                        let np = prefix_node.to_string() + &DOWN;

                        for nid in nnodes {
                            self.recursive_write(f, nid, &rp, &np, depth+1)?;
                        }
                        let rp = prefix_node.to_string() + TURN_RIGHT;
                        let np = prefix_node.to_string()+ EMPTY;        
                        self.recursive_write(f, last_node, &rp,&np,depth+1)?;
                    }
                }
            }
        }
        Ok(())
    }
}


impl <T> fmt::Display for Graph<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(sources) = &self.sources {
            for s in sources.clone() {
                self.recursive_write(f, &s, "", "", 0)?;
            }
        }
        Ok(())
    }
}
