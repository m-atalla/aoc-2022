#[derive(Debug, Clone)]
enum Node {
    Dir(String, Vec<Node>),
    File(usize)
}


impl Node {
    fn from_str(s: &str) -> Self {
        let (lhs, rhs) = s.split_once(" ").unwrap();

        return match lhs {
            "dir" => Node::Dir(rhs.to_string(), Vec::new()),
            _ => Node::File(lhs.parse().unwrap())
        };
    }
}


#[derive(Debug)]
struct FileSystem {
    pub current: Node, 
    pub stack: Vec<Node>
}

impl FileSystem {
    pub fn new(root: Node) -> Self {
        Self {
            current: root,
            stack: Vec::new(),
        }
    }
}


fn process_input(input: &str, fs: &mut FileSystem) {
    for line in input.lines().skip(1) { 

        let parts = line.split(" ").collect::<Vec<_>>();

        if line.starts_with("$"){
            match parts.get(1) {
                Some(&"cd") => {

                    // updating stack
                    let mut old = fs.current.clone();

                    // Processing argument
                    match parts.get(2).unwrap() {
                        // move to parent.. e.g. unwind the stack
                        &".." => {
                            let parent = fs.stack.pop().unwrap();
                            
                            fs.current = parent;

                            if let Node::Dir(_, children) = &mut fs.current {
                                children.push(old);
                            }

                            continue;
                        },
                        &arg => {
                            if let Node::Dir(_, children) = &mut fs.current {
                                for (idx, child) in children.iter().enumerate() {
                                    match child {
                                        Node::File(_) => continue,
                                        Node::Dir(name, _) => {
                                            if name.as_str() == arg {

                                                if let Node::Dir(_, children) = &mut old {
                                                    fs.current = children.remove(idx);
                                                }

                                                fs.stack.push(old);

                                                break;
                                            }
                                        }
                                    }
                                }

                                continue;
                            }
                        },
                    };

                    // update current node

                },
                Some(&"ls") => continue,
                _ => panic!("Invalid command")
            }
        } else {
            if let Node::Dir(_, children) = &mut fs.current {
                children.push(
                    Node::from_str(line)
                );
            }
        }
    }
}

fn calc_size(node: &Node, mut dir_sizes: &mut Vec<usize>) -> usize {
    match node {
        Node::Dir(_, children) => {

            let dir_size  = children
                .iter()
                .map(|child| calc_size(child, &mut dir_sizes))
                .sum();

            dir_sizes.push(dir_size);

            dir_size
        },
        &Node::File(size) => {
            size
        }
    }
}

fn part_1(input: &str) -> (Vec<usize>, usize){
    let mut fs = FileSystem::new(
        Node::Dir("/".to_string(), Vec::new())
    );

    process_input(&input, &mut fs);

    let root = fs.current;
    let mut dir_sizes = Vec::new();


    let root_size = calc_size(&root, &mut dir_sizes);


    // to be used in part 2
    let dir_sizes_result = dir_sizes.clone();

    let part_1: usize = dir_sizes
        .into_iter()
        .filter(|&s| s <= 100000)
        .sum();

    println!("part 1: {}", part_1);

    return (dir_sizes_result, root_size)
}

const DISK_SPACE: usize = 70000000;
const UPDATE_SPACE: usize = 30000000;
fn main() {
    let input = std::fs::read_to_string("./inputs/07.in")
        .unwrap();
    
    let (mut dir_sizes, root_size) = part_1(&input);

    let free_space = DISK_SPACE - root_size;

    let required_space = UPDATE_SPACE - free_space;

    dir_sizes.sort();

    for size in dir_sizes {
        if size >= required_space {
            println!("part 2: {}",size);
            break;
        }
    }
}
