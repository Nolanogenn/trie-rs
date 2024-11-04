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
    let testNode1 = Node::new("Gennaro".into());
    let testNode2 = Node::new("Moka".into());
    let testNode3 = Node::new("Paola".into());
    let testNode4 = Node::new("Ivana".into());
    //TODO: map vocab to a trie struct
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

pub type NodeIndex = usize,
pub type EdgeIndex = usize,

struct Node{
    value: String,
    outgoingEdge: Option<EdgeIndex>
}
impl Node{
    fn new(value: String) -> Self {
        if value.is_empty(){
            panic!("Only root node is allowed to be empty");
        }
        Self{
            value
        }
    }
    }
}
impl fmt::Display for Node{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "value: {}", self.value, self.isTerminal)
    }
}
struct Edge{
    target: NodeIndex
}
struct Trie{
    nodes: Vec<Node>,
    edges: Vec<Edge>
}
impl Trie{
    pub fn addEdge(&mut self, source:NodeIndex, target:NodeIndex){
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(Edge {target:target});
        node_data.outgoingEdge = edge_index;
    }
}
