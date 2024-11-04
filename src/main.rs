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
    let trie = Trie::new();
    println!("{}", trie);
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
    pub fn findEdge(&self, source:NodeIndex, target: char) -> Option<EdgeIndex>{
        let n = &self.nodes[source];
        let outgoingEdges = &n.outgoingEdges;
        for edgeIndex in outgoingEdges{
            let edge = self.edges[*edgeIndex];
            let tgt = edge.target;
            if self.nodes[tgt].value == target{
                return edgeIndex
            }
        }
        None
    }
    pub fn addNode(&mut self, value: String){
        let currentNodeIndex: NodeIndex = 0;
        for c in value.chars(){
            let mut currentNode = self.nodes[currentNodeIndex];
            if currentNode.isTerminal {
                currentNode.isTerminal = false;
                let freeSpace = self.nodes.len();
                self.nodes.push(Node {value:c, isTerminal: true, outgoingEdges: Vec::new()});
                self.addEdge(currentNodeIndex,freeSpace);
            }
            let newNodeIndex = self.findEdge(currentNodeIndex,c).unwrap();
            currentNodeIndex = newNodeIndex;
        }
    }
    pub fn addEdge(&mut self, source:NodeIndex, target:NodeIndex){
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(Some(Edge {target:target}));
        node_data.outgoingEdges.push(Some(edge_index));
    }
}
impl fmt::Display for Trie{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "nodes: {:?} || edges: {:?}", self.nodes, self.edges)
    }
}
impl fmt::Display for Edge{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "targetIndex: {:?}", self.target)
    }
}
