use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fs::File,
};
use std::env;
use std::fs;
use std::fmt;

fn lines_from_file(filename: String) -> Vec<String>{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|x| x.expect("Could not parse line"))
        .collect()
}

fn main() {
    let contents = lines_from_file("../data/italian.dic".into());
    let mut trie = Trie::new();
    for line in contents{
        trie.addNode(line.into());
    }
    println!("TRIE CREATED\nAWAITING FOR MESSAGES\n");    
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream, &mut trie);
    }
}

fn handle_connection(mut stream: TcpStream, trie: &mut Trie){
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    println!("{}", trie.search(request_line));
//    if request_line == "GET / HTTP/1.1" {
//        let status_line = "HTTP/1.1 200 OK";
//        let contents = fs::read_to_string.unwrap();
//        let length = contents.len();
//        let response = format!(
//            "{status_line}\r\nContent-Length {length}\r\n\r\n{contents}"
//            );
//        stream.write_all(response.as_bytes()).unwrap();
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
}

pub type NodeIndex = usize;

#[derive(Default, Debug, Clone)]
struct Node{
    value: char,
    outgoingEdges: Vec<NodeIndex>,
    isTerminal: bool
}
impl fmt::Display for Node{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "value: {} || isTerminal: {} || egdes: {:?}", self.value, self.isTerminal, self.outgoingEdges)
    }
}
#[derive(Default, Debug)]
struct Trie{
    nodes: Vec<Node>,
}
impl Trie{
    pub fn new() -> Self {
        let rootChar: char = ' ';
        let root = Node{value: rootChar, isTerminal: false, outgoingEdges: Vec::new()};
        Self{nodes: vec![root]}
    }
    pub fn checkChildren(&mut self, nodeIndex: NodeIndex, c: char) -> Option<NodeIndex>{
        let node = &self.nodes[nodeIndex];
        for i in &node.outgoingEdges{
            if self.nodes[*i].value == c {
                return Some(*i);
            }
        }
        return None;
    }
    pub fn addAt(&mut self, nodeIndex: NodeIndex, c:char){
        let freeSpace = self.nodes.len();
        let mut node = &mut self.nodes[nodeIndex];
        node.outgoingEdges.push(freeSpace);
        let newNode = Node{value:c, outgoingEdges:Vec::new(), isTerminal:false};
        self.nodes.push(newNode);
    }
    pub fn addNode(&mut self, value: String){
        let mut currentNodeIndex = 0;
        for c in value.chars(){
            let child = self.checkChildren(currentNodeIndex, c);
            match child{
                None => {
                    self.addAt(currentNodeIndex, c);
                    currentNodeIndex = self.nodes.len()-1;
                },
                Some(i) => {
                    currentNodeIndex = i;
                }
            }
        }
        let mut lastNode = &mut self.nodes[currentNodeIndex];
        lastNode.isTerminal = true;
    }
    pub fn search(&mut self, value: String) -> bool {
        let mut currentNodeIndex: NodeIndex = 0;
        for c in value.chars(){
            let child = self.checkChildren(currentNodeIndex, c);
            match child{
                None => {
                    return false
                },
                Some(i) => {
                    currentNodeIndex = i;
                }
            }
        }
        return self.nodes[currentNodeIndex].isTerminal
    }
    pub fn deleteNode(&mut self, value: String) -> bool{
        let mut currentNodeIndex: NodeIndex = 0;
        let mut dangling = Vec::new();
        let mut tofix = Vec::new();
        for c in value.chars(){
            let child = self.checkChildren(currentNodeIndex,c);
            match child {
                None => {
                    return false
                        }
                Some(i) => {
                    currentNodeIndex = i;
                    }
            }
        }
        self.nodes.remove(currentNodeIndex);
        self.fixIndices(currentNodeIndex, true);
        
        for node in self.nodes.iter(){
            dangling.push(checkTerminal(&node,&self.nodes));
            if !checkTerminal(&node, &self.nodes){
                tofix.push(dangling.len());
            }
            };
        for elem in tofix.iter().rev(){
            println!("FIXING {:?}", elem);
            self.fixIndices(*elem, false);
        }
        let mut iter = dangling.iter();
        self.nodes.retain(|_| *iter.next().unwrap());
        return true
    }
    pub fn fixIndices(&mut self, nodeIndex: NodeIndex, ret: bool){
        for mut node in &mut self.nodes{
            if ret{
                node.outgoingEdges.retain(|&x| x!= nodeIndex);
            }
            node.outgoingEdges.iter_mut().for_each(|x| {
                if *x >= nodeIndex {
                    *x -= 1;
                }
            });
        }
    }
    pub fn printNodes(&self){
        for (count,v) in self.nodes.iter().enumerate(){
            println!("{}: {}", count,v)
        }
    }
}
fn checkTerminal(node: &Node, nodes: &Vec<Node>)->bool{
    if node.isTerminal{
        return true
    }
    for next in &node.outgoingEdges{
        if checkTerminal(&nodes[*next], &nodes){
            return true
        }
    }
    return false
}



impl fmt::Display for Trie{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.nodes)
    }
}
