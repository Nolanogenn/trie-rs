//use std::{
//    io::{prelude::*, BufReader},
//    net::{TcpListener, TcpStream},
//};
use std::env;
use std::fs;
use std::fmt;

fn main() {
    //TODO: load vocab
//    let contents = fs::read_to_string("../data/games_list").expect("Should be able to read the file");
    //TODO: map vocab to a trie struct
    let mut trie = Trie::new();
    trie.addNode("gennaro".into());
    trie.addNode("simona".into());
    trie.addNode("gelato".into());
    println!("{}", trie);
    //println!("{}", trie.search("gelato".into()));
    //println!("{}", trie.search("gennaro".into()));
    trie.addNode("simpatica".into());
    println!("{}", trie.search("gelato".into()));
    trie.deleteNode("gennaro".into());
    println!("{}", trie);
    println!("{}", trie.search("gelato".into()));
    //println!("{}", trie.search("gennaro".into()));

    //let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //for stream in listener.incoming(){
    //    let stream = stream.unwrap();
    //    handle_connection(stream);
    //}
}

//fn handle_connection(mut stream: TcpStream){
//    let buf_reader = BufReader::new(&mut stream);
//    let request_line = buf_reader.lines().next().unwrap().unwrap();
//    if request_line == "GET / HTTP/1.1" {
//        let status_line = "HTTP/1.1 200 OK";
////        let contents = fs::read_to_string;
//        //stream.write_all(response.as_bytes()).unwrap();
//        //TODO: get input from request
//        //TODO: look up for the input
//    } else {
//        let status_line = "HTTP/1.1 404 NOT FOUND";
//        let contents = fs::read_to_string("404.html").unwrap();
//        let length = contents.len();
//        let response = format!(
//            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}",
//            );
//        stream.write_all(response.as_bytes()).unwrap();
//    }
//}

pub type NodeIndex = usize;
pub type EdgeIndex = usize;

#[derive(Debug)]
struct Node{
    value: char,
    outgoingEdges: Vec<EdgeIndex>,
    isTerminal: bool
}
impl fmt::Display for Node{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "value: {} || isTerminal: {}", self.value, self.isTerminal)
    }
}
#[derive(Debug)]
struct Edge{
    target: NodeIndex
}
#[derive(Debug)]
struct Trie{
    nodes: Vec<Node>,
    edges: Vec<Edge>
}
impl Trie{
    pub fn new() -> Self {
        let rootChar: char = ' ';
        let root = Node{value: rootChar, isTerminal: true, outgoingEdges: Vec::new()};
        let mut n = Vec::new();
        n.push(root);
        Self{nodes: n, edges: Vec::new()}
    }
    pub fn search(&self, value: String) -> bool {
        let mut currentNodeIndex: NodeIndex = 0;
        for c in value.chars(){
            let edgeIndex = self.findEdge(currentNodeIndex,c);
            match edgeIndex {
                None => {
                    return false
                        }
                Some(_) => {
                    let ei = edgeIndex.unwrap();
                    println!("letter: {}, index: {}", c, ei);
                    let newNodeIndex = self.edges[ei].target;
                    currentNodeIndex = newNodeIndex;
                            }
            }
        }
        return true
    }
    pub fn findEdge(&self, source:NodeIndex, target: char) -> Option<EdgeIndex>{
        let n = &self.nodes[source];
        for edgeIndex in &n.outgoingEdges{
            let edge = &self.edges[*edgeIndex];
            let tgt = edge.target;
            println!("LOOKING FOR: {}, LOOKING AT: {}", target, self.nodes[tgt].value);
            if self.nodes[tgt].value == target{
                return Some(*edgeIndex)
            }
        }
        None
    }
    pub fn deleteNode(&mut self, value: String) -> bool{
        let mut currentNodeIndex: NodeIndex = 0;
        let mut ei: EdgeIndex = 0;
        for c in value.chars(){
            let edgeIndex = self.findEdge(currentNodeIndex,c);
            match edgeIndex {
                None => {
                    return false
                        }
                Some(_) => {
                    ei = edgeIndex.unwrap();
                    currentNodeIndex = self.edges[ei].target;
                            }
            }
        }
        self.nodes.remove(currentNodeIndex);
        self.removeEdge(&ei);
        return true
    }
    pub fn removeEdge(&mut self, &ei: &EdgeIndex){
        self.edges.remove(ei);
        for mut node in &mut self.nodes{
            if node.outgoingEdges.contains(&ei){
                let index = node.outgoingEdges.iter().position(|x| *x == ei).unwrap();
                node.outgoingEdges.remove(index);
            }
            println!("{}", node);
            for edge_i in 0..node.outgoingEdges.len(){
                if node.outgoingEdges[edge_i] > ei {
                    println!("{}", node.outgoingEdges[edge_i]);
                    node.outgoingEdges.remove(edge_i);
                    node.outgoingEdges.push(edge_i.saturating_sub(1));
                }
            }
        }

    }
    pub fn addNode(&mut self, value: String){
        let mut currentNodeIndex: NodeIndex = 0;
        for c in value.chars(){
            let edgeIndex = self.findEdge(currentNodeIndex,c);
            match edgeIndex {
                None => {
                    let freeSpace = self.nodes.len();
                    self.nodes.push(Node {value:c, isTerminal: true, outgoingEdges: Vec::new()});
                    self.addEdge(currentNodeIndex,freeSpace);
                    let edgeIndex = self.findEdge(currentNodeIndex,c);
                    let ei = edgeIndex.unwrap();
                    let newNodeIndex = self.edges[ei].target;
                    currentNodeIndex = newNodeIndex;
                        }
                Some(_) => {
                    let ei = edgeIndex.unwrap();
                    let newNodeIndex = self.edges[ei].target;
                    currentNodeIndex = newNodeIndex;
                            }
        }
    }
    }
    pub fn addEdge(&mut self, source:NodeIndex, target:NodeIndex){
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(Edge {target:target});
        node_data.isTerminal = false;
        node_data.outgoingEdges.push(edge_index);
    }
}
impl fmt::Display for Trie{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "nodes: {:?}", self.nodes)
    }
}
impl fmt::Display for Edge{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "targetIndex: {:?}", self.target)
    }
}
