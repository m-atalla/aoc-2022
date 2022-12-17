use std::rc::{Rc, Weak};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node {
    name: String,
    size: usize,
    children: HashMap<String, Rc<Node>>,
    parent: Option<Weak<Node>>
}

impl Node {
    pub fn new(name: &str, size: usize) -> Node {
        Node { 
            name: name.to_string(), 
            size, 
            children: HashMap::default(),
            parent: None
        }
    }
}

#[inline]
fn is_command(line: &str) -> bool {
    line.starts_with("$")
}

fn process_cmd<'a>(line: &str, mut current_dir: Node, mut lines: impl Iterator<Item = &'a str>) {
    println!("line .... {}", line);
    if line == "ls" {
        println!("got an ls");

        let output = lines.by_ref().take_while(|&line| !is_command(&line));

        for l in output {
            println!("Output: {l}");
        }

        for l in lines.take(5) {
            println!("next: {l}"); 
        }

        std::process::exit(0);
    } else {
        // cd cmd
        let (_, arg) = line.split_once(" ").unwrap();

        if current_dir.children.contains_key(arg) {
            // current_dir = *current_dir.children.get(arg).unwrap().clone();
        }
    }

}

fn main() {
    let input = std::fs::read_to_string("./inputs/07.in").unwrap();

    let mut root = Node::new("/", 0);

    let mut current_dir = root.clone();

    let mut lines = input.lines().skip(1);

    let line = lines.next().unwrap();
    process_cmd(&line[2..].trim(), current_dir, lines);
} 
