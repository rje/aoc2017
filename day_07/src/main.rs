extern crate regex;

use std::str;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Node<'a> {
    pub name : String,
    pub weight : u32,
    pub children_keys : Vec<String>,
    pub parent : Option<Box<&'a Node<'a>>>
}

fn main() {
    let mut node_map = load_data("data/test_data_1.txt");
    set_parents(&node_map);
}

fn set_parents(node_map : &HashMap<String, Node>) {
    for pair in node_map {
        let node = pair.1;
        for key in &node.children_keys {
            let child = node_map.get_mut(key).unwrap();
            child.parent = Some(Box::new(node));
        }
    }
}

fn load_data(path: &str) -> HashMap<String, Node> {
    let file = BufReader::new(File::open(path).unwrap());

    let with_children_regex = Regex::new(r"^(\w*) \((\d*)\) -> ([\s\S]*)").unwrap();
    let no_children_regex = Regex::new(r"^(\w*) \((\d*)\)").unwrap();

    let node_collection : Vec<Node> = file.lines().map(|line| { 
        let to_parse = line.unwrap().into_boxed_str();
        println!("Checking {}", to_parse);
        let node : Node;
        if with_children_regex.is_match(&to_parse) {
            let captures = with_children_regex.captures(&to_parse).unwrap();
            let weight = captures[2].parse::<u32>().unwrap();
            let children_keys = captures[3].split(", ").map(|x| { let owned = x.to_owned(); return owned; }).collect();
            node = Node { name: captures[1].to_owned(), weight, children_keys, parent: None };
        }
        else if no_children_regex.is_match(&to_parse) {
            let captures = no_children_regex.captures(&to_parse).unwrap();
            let weight = captures[2].parse::<u32>().unwrap();
            node = Node { name: captures[1].to_owned(), weight, children_keys : Vec::new(), parent: None };
        }
        else {
            node = Node { name: String::new(), weight: 0, children_keys : Vec::new(), parent: None };
        }
        return node;
    }).collect();

    let mut to_return : HashMap<String, Node> = HashMap::new();

    for node in &node_collection {
        println!("{:?}", &node);
        to_return.insert(node.name.clone(), node.clone());
    }

    return to_return;
}

